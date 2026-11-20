//! `parsentry triage`: a std-only local web UI that lists merged findings and
//! writes suppressions back into merged.sarif.json. Always bound to loopback.

use anyhow::{Context, Result};
use parsentry_reports::{SarifReport, is_suppressed, suppress_result, unsuppress_result};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

/// Largest request body accepted (form posts are tiny).
const MAX_BODY: usize = 64 * 1024;

pub fn run_triage_command(target: &str, port: u16) -> Result<()> {
    let merged = super::common::cache_dir_for(target).join("merged.sarif.json");
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    eprintln!("parsentry triage UI: http://{}", listener.local_addr()?);
    for stream in listener.incoming() {
        if let Err(e) = stream
            .map_err(anyhow::Error::from)
            .and_then(|s| handle(s, &merged))
        {
            eprintln!("triage: {e:#}");
        }
    }
    Ok(())
}

fn handle(stream: TcpStream, merged: &Path) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut parts = line.split_whitespace();
    let (method, path) = (parts.next().unwrap_or(""), parts.next().unwrap_or("/"));
    let (method, path) = (method.to_string(), path.to_string());

    let mut length = 0usize;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header)? == 0 || header.trim().is_empty() {
            break;
        }
        if let Some((name, value)) = header.split_once(':')
            && name.eq_ignore_ascii_case("content-length")
        {
            length = value.trim().parse().unwrap_or(0);
        }
    }
    let mut stream = stream;
    if length > MAX_BODY {
        return respond(&mut stream, "413 Payload Too Large", "too large");
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    let form = parse_form(&String::from_utf8_lossy(&body));

    match (method.as_str(), path.as_str()) {
        ("GET", "/") => {
            let page = match load(merged) {
                Ok(report) => render(&report),
                Err(e) => format!("<p>{}</p>", escape(&format!("{e:#}"))),
            };
            respond(&mut stream, "200 OK", &page)
        }
        ("POST", "/suppress" | "/unsuppress") => {
            let fp = form_value(&form, "fp");
            let justification = form_value(&form, "justification");
            apply(merged, &path, fp, justification)?;
            write!(
                stream,
                "HTTP/1.1 303 See Other\r\nLocation: /\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )?;
            Ok(())
        }
        _ => respond(&mut stream, "404 Not Found", "not found"),
    }
}

/// Apply a triage action to merged.sarif.json and write it back atomically.
fn apply(merged: &Path, action: &str, fp: &str, justification: &str) -> Result<usize> {
    let mut report = load(merged)?;
    let n = if action == "/suppress" {
        suppress_result(&mut report, fp, justification)
    } else {
        unsuppress_result(&mut report, fp)
    };
    let tmp = merged.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(&report)?)?;
    std::fs::rename(&tmp, merged)?;
    Ok(n)
}

fn load(merged: &Path) -> Result<SarifReport> {
    let s = std::fs::read_to_string(merged)
        .with_context(|| format!("no merged report at {}", merged.display()))?;
    Ok(serde_json::from_str(&s)?)
}

fn respond(stream: &mut TcpStream, status: &str, body: &str) -> Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )?;
    Ok(())
}

fn render(report: &SarifReport) -> String {
    let mut rows = String::new();
    for r in report.runs.iter().flat_map(|r| &r.results) {
        let fp = r
            .fingerprints
            .as_ref()
            .and_then(|f| f.get("parsentry/v1"))
            .map(String::as_str)
            .unwrap_or("");
        let uri = r
            .locations
            .first()
            .map(|l| l.physical_location.artifact_location.uri.as_str())
            .unwrap_or("");
        let action = if is_suppressed(r) {
            let why = r
                .suppressions
                .iter()
                .flatten()
                .find_map(|s| s.justification.as_deref())
                .unwrap_or("");
            format!(
                "suppressed: {}<form method=post action=/unsuppress><input type=hidden name=fp value=\"{}\"><button>Unsuppress</button></form>",
                escape(why),
                escape(fp)
            )
        } else {
            format!(
                "<form method=post action=/suppress><input type=hidden name=fp value=\"{}\"><input name=justification placeholder=justification><button>Suppress</button></form>",
                escape(fp)
            )
        };
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{action}</td></tr>",
            escape(&r.level),
            escape(&r.rule_id),
            escape(uri),
            escape(&r.message.text),
        ));
    }
    format!(
        "<!doctype html><meta charset=utf-8><title>parsentry triage</title><h1>Triage</h1>\
         <table border=1><tr><th>Level</th><th>Rule</th><th>Location</th><th>Message</th><th>Triage</th></tr>{rows}</table>"
    )
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn parse_form(body: &str) -> Vec<(String, String)> {
    body.split('&')
        .filter_map(|pair| pair.split_once('='))
        .map(|(k, v)| (url_decode(k), url_decode(v)))
        .collect()
}

fn form_value<'a>(form: &'a [(String, String)], key: &str) -> &'a str {
    form.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
        .unwrap_or("")
}

fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
                match u8::from_str_radix(hex, 16) {
                    Ok(b) => {
                        out.push(b);
                        i += 2;
                    }
                    Err(_) => out.push(b'%'),
                }
            }
            b => out.push(b),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const SARIF: &str = r#"{"$schema":"s","version":"2.1.0","runs":[{"tool":{"driver":{"name":"t","version":"1"}},"results":[{"ruleId":"r<1>","level":"error","message":{"text":"m"},"locations":[],"fingerprints":{"parsentry/v1":"abc"}}]}]}"#;

    #[test]
    fn form_decoding() {
        assert_eq!(url_decode("a+b%20c%2"), "a b c%2");
        assert_eq!(url_decode("%zz"), "%zz");
        assert_eq!(url_decode("%e3%81%82x%"), "あx%");
        assert_eq!(url_decode("%あ"), "%あ");
        let form = parse_form("fp=abc&justification=not+reachable");
        assert_eq!(form_value(&form, "fp"), "abc");
        assert_eq!(form_value(&form, "justification"), "not reachable");
        assert_eq!(form_value(&form, "missing"), "");
    }

    #[test]
    fn escape_html() {
        assert_eq!(
            escape(r#"<a href="x">&'"#),
            "&lt;a href=&quot;x&quot;&gt;&amp;&#39;"
        );
    }

    #[test]
    fn apply_writes_back_and_render_reflects_state() {
        let dir = TempDir::new().unwrap();
        let merged = dir.path().join("merged.sarif.json");
        std::fs::write(&merged, SARIF).unwrap();
        let page = render(&load(&merged).unwrap());
        assert!(page.contains("action=/suppress"));
        assert!(page.contains("r&lt;1&gt;"));

        assert_eq!(apply(&merged, "/suppress", "abc", "test only").unwrap(), 1);
        let report = load(&merged).unwrap();
        assert!(is_suppressed(&report.runs[0].results[0]));
        let page = render(&report);
        assert!(page.contains("action=/unsuppress"));
        assert!(page.contains("suppressed: test only"));

        assert_eq!(apply(&merged, "/unsuppress", "abc", "").unwrap(), 1);
        assert!(!is_suppressed(&load(&merged).unwrap().runs[0].results[0]));
    }
}
