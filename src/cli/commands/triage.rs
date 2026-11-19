//! `parsentry triage`: a local web UI for suppressing findings.
//!
//! Serves the merged report on localhost. Suppressing a finding writes an
//! `external` / `accepted` entry to its SARIF `suppressions[]` in
//! `merged.sarif.json`; the next report phase carries it forward by
//! fingerprint so the decision survives a rescan.

use anyhow::{Context, Result, bail};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

use parsentry_reports::merge::fingerprint;
use parsentry_reports::{SarifReport, suppress_result, unsuppress_result};

use super::common::cache_dir_for;

/// Largest accepted request body; the form only carries a fingerprint and a note.
const MAX_BODY: usize = 64 * 1024;

pub fn run_triage_command(target: &str, port: u16) -> Result<()> {
    let merged = cache_dir_for(target).join("merged.sarif.json");
    if !merged.exists() {
        bail!(
            "no merged report at {} — run `parsentry scan` to completion first",
            merged.display()
        );
    }
    let listener = TcpListener::bind(("127.0.0.1", port))
        .with_context(|| format!("cannot bind 127.0.0.1:{port}"))?;
    eprintln!(
        "Triage UI: http://{} (Ctrl-C to stop)",
        listener.local_addr()?
    );
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        if let Err(e) = handle(stream, &merged) {
            eprintln!("triage: {e:#}");
        }
    }
    Ok(())
}

fn handle(mut stream: TcpStream, merged: &Path) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 || line.trim().is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':')
            && k.eq_ignore_ascii_case("content-length")
        {
            content_length = v.trim().parse().unwrap_or(0);
        }
    }
    let mut parts = request_line.split_whitespace();
    let (method, path) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    let response = match (method, path) {
        ("GET", "/") => {
            let report = load(merged)?;
            http(200, "text/html; charset=utf-8", &render_page(&report))
        }
        ("POST", "/suppress" | "/unsuppress") if content_length <= MAX_BODY => {
            let mut body = vec![0; content_length];
            reader.read_exact(&mut body)?;
            let form = parse_form(&String::from_utf8_lossy(&body));
            let fp = form_value(&form, "fingerprint");
            let mut report = load(merged)?;
            let found = if path == "/suppress" {
                suppress_result(&mut report, fp, form_value(&form, "justification"))
            } else {
                unsuppress_result(&mut report, fp)
            };
            if found {
                save(merged, &report)?;
                "HTTP/1.1 303 See Other\r\nLocation: /\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                    .to_string()
            } else {
                http(404, "text/plain", "finding not found")
            }
        }
        _ => http(400, "text/plain", "bad request"),
    };
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn http(status: u16, content_type: &str, body: &str) -> String {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "Bad Request",
    };
    format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

fn load(path: &Path) -> Result<SarifReport> {
    let json = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    serde_json::from_str(&json).with_context(|| format!("invalid SARIF: {}", path.display()))
}

fn save(path: &Path, report: &SarifReport) -> Result<()> {
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(report)?)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

fn parse_form(body: &str) -> Vec<(String, String)> {
    body.split('&')
        .filter_map(|pair| {
            let (k, v) = pair.split_once('=')?;
            let decode = |s: &str| {
                urlencoding::decode(&s.replace('+', " "))
                    .map(|c| c.into_owned())
                    .ok()
            };
            Some((decode(k)?, decode(v)?))
        })
        .collect()
}

fn form_value<'a>(form: &'a [(String, String)], key: &str) -> &'a str {
    form.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
        .unwrap_or("")
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn render_page(report: &SarifReport) -> String {
    let mut rows = String::new();
    for result in report.runs.iter().flat_map(|r| &r.results) {
        if result.baseline_state.as_deref() == Some("absent") {
            continue;
        }
        let fp = escape(&fingerprint(result));
        let location = result
            .locations
            .first()
            .map(|l| {
                let uri = &l.physical_location.artifact_location.uri;
                match l.physical_location.region.as_ref() {
                    Some(r) => format!("{uri}:{}", r.start_line),
                    None => uri.clone(),
                }
            })
            .unwrap_or_default();
        let suppression = result.suppressions.as_ref().and_then(|s| s.first());
        let action = match suppression {
            Some(s) => format!(
                "<span class=\"sup\">suppressed{}</span>\
                 <form method=\"post\" action=\"/unsuppress\">\
                 <input type=\"hidden\" name=\"fingerprint\" value=\"{fp}\">\
                 <button>Unsuppress</button></form>",
                s.justification
                    .as_deref()
                    .map(|j| format!(": {}", escape(j)))
                    .unwrap_or_default()
            ),
            None => format!(
                "<form method=\"post\" action=\"/suppress\">\
                 <input type=\"hidden\" name=\"fingerprint\" value=\"{fp}\">\
                 <input name=\"justification\" placeholder=\"justification\">\
                 <button>Suppress</button></form>"
            ),
        };
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{action}</td></tr>\n",
            escape(&result.level),
            escape(&result.rule_id),
            escape(&location),
            escape(&result.message.text),
        ));
    }
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>Parsentry triage</title>\
         <style>body{{font-family:sans-serif}}table{{border-collapse:collapse}}\
         td,th{{border:1px solid #ccc;padding:4px;vertical-align:top}}.sup{{color:#777}}</style>\
         </head><body><h1>Parsentry triage</h1><table>\
         <tr><th>Level</th><th>Rule</th><th>Location</th><th>Message</th><th>Triage</th></tr>\n\
         {rows}</table></body></html>"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_urlencoded_form() {
        let form = parse_form("fingerprint=ab%2Fc&justification=false+positive");
        assert_eq!(form_value(&form, "fingerprint"), "ab/c");
        assert_eq!(form_value(&form, "justification"), "false positive");
        assert_eq!(form_value(&form, "missing"), "");
    }

    #[test]
    fn page_escapes_and_shows_suppression() {
        let mut report: SarifReport = serde_json::from_value(serde_json::json!({
            "$schema": "s", "version": "2.1.0",
            "runs": [{"tool": {"driver": {"name": "t", "version": "1"}}, "results": [
                {"ruleId": "XSS", "level": "error", "message": {"text": "<script>"},
                 "fingerprints": {"parsentry/v1": "fp1"}}
            ]}]
        }))
        .unwrap();
        let page = render_page(&report);
        assert!(page.contains("&lt;script&gt;"));
        assert!(page.contains("action=\"/suppress\""));
        suppress_result(&mut report, "fp1", "fp");
        let page = render_page(&report);
        assert!(page.contains("suppressed: fp"));
        assert!(page.contains("action=\"/unsuppress\""));
    }
}
