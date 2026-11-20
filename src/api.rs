//! Read-only HTTP API over cached scan runs, served by `parsentry serve`.
//!
//! `GET /api/runs` lists the cache directories under [`cache_base`] and
//! `GET /api/findings?run=&verdict=&level=&cwe=` flattens their SARIF results.
//! Run ids are only accepted when they match an enumerated cache directory, so a
//! crafted `run` cannot reach outside the cache. When a token is configured,
//! every `/api/` request needs `Authorization: Bearer <token>`.
//!
//! [`cache_base`]: crate::cli::commands::common::cache_base

use anyhow::{Result, bail};
use serde::Serialize;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use crate::webhook::Request;

pub const API_PREFIX: &str = "/api/";

/// Refuse to expose the API beyond loopback without a token.
pub fn check_bind(host: &str, token: Option<&str>) -> Result<()> {
    if token.is_some() || is_loopback(host) {
        return Ok(());
    }
    bail!("refusing to bind non-loopback host {host} without --token (or PARSENTRY_API_TOKEN)")
}

fn is_loopback(host: &str) -> bool {
    let host = host.trim_start_matches('[').trim_end_matches(']');
    host.eq_ignore_ascii_case("localhost")
        || host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

/// Check `Authorization: Bearer <token>` in constant time. No token means open.
pub fn authorized(req: &Request, token: Option<&str>) -> bool {
    let Some(token) = token else {
        return true;
    };
    let Some(given) = req
        .header("authorization")
        .and_then(|h| h.strip_prefix("Bearer "))
    else {
        return false;
    };
    given.len() == token.len()
        && given
            .bytes()
            .zip(token.bytes())
            .fold(0u8, |acc, (a, b)| acc | (a ^ b))
            == 0
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunInfo {
    pub id: String,
    pub has_model: bool,
    pub has_report: bool,
    pub surfaces: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    pub run: String,
    pub rule_id: String,
    pub level: String,
    pub verdict: String,
    pub cwe: Vec<String>,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u64>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FindingFilter {
    pub run: Option<String>,
    pub verdict: Option<String>,
    pub level: Option<String>,
    pub cwe: Option<String>,
}

fn is_run_dir(dir: &Path) -> bool {
    dir.join("model.json").is_file()
        || dir.join("merged.sarif.json").is_file()
        || dir.join("reports").is_dir()
}

fn subdirs(dir: &Path) -> Vec<(String, PathBuf)> {
    let mut out: Vec<_> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .filter_map(|e| Some((e.file_name().into_string().ok()?, e.path())))
        .collect();
    out.sort();
    out
}

/// Enumerate run directories: `<base>/<id>` and network targets `<base>/url/<hash>`.
fn run_dirs(base: &Path) -> Vec<(String, PathBuf)> {
    let mut runs = Vec::new();
    for (name, path) in subdirs(base) {
        if name == "url" {
            runs.extend(
                subdirs(&path)
                    .into_iter()
                    .filter(|(_, p)| is_run_dir(p))
                    .map(|(hash, p)| (format!("url/{hash}"), p)),
            );
        } else if is_run_dir(&path) {
            runs.push((name, path));
        }
    }
    runs
}

pub fn list_runs(base: &Path) -> Vec<RunInfo> {
    run_dirs(base)
        .into_iter()
        .map(|(id, dir)| RunInfo {
            id,
            has_model: dir.join("model.json").is_file(),
            has_report: dir.join("merged.sarif.json").is_file(),
            surfaces: subdirs(&dir.join("reports")).len(),
        })
        .collect()
}

/// Resolve a run id to its directory, only if it is one of the enumerated runs.
pub fn resolve_run(base: &Path, id: &str) -> Option<PathBuf> {
    run_dirs(base)
        .into_iter()
        .find(|(rid, _)| rid == id)
        .map(|(_, p)| p)
}

fn read_json(path: &Path) -> Option<serde_json::Value> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

/// SARIF documents of a run: the merged report, else every per-surface result.
fn run_sarifs(dir: &Path) -> Vec<serde_json::Value> {
    if let Some(merged) = read_json(&dir.join("merged.sarif.json")) {
        return vec![merged];
    }
    subdirs(&dir.join("reports"))
        .into_iter()
        .filter_map(|(_, p)| read_json(&p.join("result.sarif.json")))
        .collect()
}

fn str_at(v: &serde_json::Value, ptr: &str) -> Option<String> {
    v.pointer(ptr).and_then(|s| s.as_str()).map(str::to_string)
}

fn extract_findings(run: &str, sarif: &serde_json::Value) -> Vec<Finding> {
    let mut out = Vec::new();
    let runs = sarif.get("runs").and_then(|r| r.as_array());
    for sarif_run in runs.into_iter().flatten() {
        let results = sarif_run.get("results").and_then(|r| r.as_array());
        for r in results.into_iter().flatten() {
            let cwe = match r.pointer("/properties/cwe") {
                Some(serde_json::Value::Array(a)) => a
                    .iter()
                    .filter_map(|c| c.as_str().map(str::to_string))
                    .collect(),
                Some(serde_json::Value::String(s)) => vec![s.clone()],
                _ => Vec::new(),
            };
            let loc = "/locations/0/physicalLocation";
            out.push(Finding {
                run: run.to_string(),
                rule_id: str_at(r, "/ruleId").unwrap_or_default(),
                level: str_at(r, "/level").unwrap_or_else(|| "warning".into()),
                verdict: str_at(r, "/properties/verdict").unwrap_or_else(|| "indeterminate".into()),
                cwe,
                message: str_at(r, "/message/text").unwrap_or_default(),
                file: str_at(r, &format!("{loc}/artifactLocation/uri")),
                line: r
                    .pointer(&format!("{loc}/region/startLine"))
                    .and_then(|l| l.as_u64()),
            });
        }
    }
    out
}

fn cwe_number(s: &str) -> &str {
    let s = s.trim();
    s.get(..4)
        .filter(|p| p.eq_ignore_ascii_case("cwe-"))
        .map_or(s, |_| &s[4..])
}

impl FindingFilter {
    pub fn matches(&self, f: &Finding) -> bool {
        self.verdict
            .as_deref()
            .is_none_or(|v| f.verdict.eq_ignore_ascii_case(v))
            && self
                .level
                .as_deref()
                .is_none_or(|l| f.level.eq_ignore_ascii_case(l))
            && self
                .cwe
                .as_deref()
                .is_none_or(|c| f.cwe.iter().any(|x| cwe_number(x) == cwe_number(c)))
    }
}

/// Findings across runs (or one run). `None` means the run id is unknown.
pub fn list_findings(base: &Path, filter: &FindingFilter) -> Option<Vec<Finding>> {
    let runs = match &filter.run {
        Some(id) => vec![(id.clone(), resolve_run(base, id)?)],
        None => run_dirs(base),
    };
    Some(
        runs.iter()
            .flat_map(|(id, dir)| {
                run_sarifs(dir)
                    .iter()
                    .flat_map(|s| extract_findings(id, s))
                    .collect::<Vec<_>>()
            })
            .filter(|f| filter.matches(f))
            .collect(),
    )
}

fn parse_query(query: &str) -> FindingFilter {
    let mut filter = FindingFilter::default();
    for pair in query.split('&').filter(|p| !p.is_empty()) {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        let v = urlencoding::decode(&v.replace('+', " "))
            .map(|c| c.into_owned())
            .unwrap_or_default();
        if v.is_empty() {
            continue;
        }
        match k {
            "run" => filter.run = Some(v),
            "verdict" => filter.verdict = Some(v),
            "level" => filter.level = Some(v),
            "cwe" => filter.cwe = Some(v),
            _ => {}
        }
    }
    filter
}

fn json_error(msg: &str) -> String {
    serde_json::json!({ "error": msg }).to_string()
}

/// Route an `/api/` request and return `(status, json body)`.
pub fn handle(req: &Request, token: Option<&str>, base: &Path) -> (u16, String) {
    if !authorized(req, token) {
        return (401, json_error("unauthorized"));
    }
    if req.method != "GET" {
        return (405, json_error("method not allowed"));
    }
    let (path, query) = req.path.split_once('?').unwrap_or((&req.path, ""));
    let body = match path {
        "/api/runs" => serde_json::to_string(&list_runs(base)),
        "/api/findings" => match list_findings(base, &parse_query(query)) {
            Some(findings) => serde_json::to_string(&findings),
            None => return (404, json_error("unknown run")),
        },
        _ => return (404, json_error("not found")),
    };
    match body {
        Ok(b) => (200, b),
        Err(e) => (500, json_error(&e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get(path: &str, auth: Option<&str>) -> Request {
        Request {
            method: "GET".into(),
            path: path.into(),
            headers: auth
                .map(|a| vec![("Authorization".into(), a.into())])
                .unwrap_or_default(),
            body: Vec::new(),
        }
    }

    fn sarif(results: serde_json::Value) -> String {
        serde_json::json!({ "version": "2.1.0", "runs": [{ "results": results }] }).to_string()
    }

    fn fixture() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let a = tmp.path().join("owner__app");
        std::fs::create_dir_all(a.join("reports/s1")).unwrap();
        std::fs::write(a.join("model.json"), "{}").unwrap();
        std::fs::write(
            a.join("merged.sarif.json"),
            sarif(serde_json::json!([
                { "ruleId": "sqli", "level": "error", "message": { "text": "SQL" },
                  "properties": { "verdict": "confirmed", "cwe": ["CWE-89"] },
                  "locations": [{ "physicalLocation": {
                      "artifactLocation": { "uri": "app.py" }, "region": { "startLine": 7 } } }] },
                { "ruleId": "xss", "level": "warning", "message": { "text": "XSS" },
                  "properties": { "cwe": ["CWE-79"] } }
            ])),
        )
        .unwrap();
        let b = tmp.path().join("url/abc123/reports/s1");
        std::fs::create_dir_all(&b).unwrap();
        std::fs::write(
            b.join("result.sarif.json"),
            sarif(serde_json::json!([
                { "ruleId": "ssrf", "level": "error",
                  "properties": { "verdict": "ruled_out", "cwe": ["CWE-918"] } }
            ])),
        )
        .unwrap();
        // Not a run: no model, report, or reports dir.
        std::fs::create_dir_all(tmp.path().join("stray")).unwrap();
        tmp
    }

    fn findings(base: &Path, query: &str) -> Vec<Finding> {
        let (status, body) = handle(&get(&format!("/api/findings{query}"), None), None, base);
        assert_eq!(status, 200, "{body}");
        serde_json::from_str::<Vec<serde_json::Value>>(&body)
            .unwrap()
            .into_iter()
            .map(|v| Finding {
                run: v["run"].as_str().unwrap().into(),
                rule_id: v["rule_id"].as_str().unwrap().into(),
                level: v["level"].as_str().unwrap().into(),
                verdict: v["verdict"].as_str().unwrap().into(),
                cwe: serde_json::from_value(v["cwe"].clone()).unwrap(),
                message: v["message"].as_str().unwrap().into(),
                file: v["file"].as_str().map(Into::into),
                line: v["line"].as_u64(),
            })
            .collect()
    }

    fn rules(fs: &[Finding]) -> Vec<&str> {
        fs.iter().map(|f| f.rule_id.as_str()).collect()
    }

    #[test]
    fn lists_runs() {
        let tmp = fixture();
        let (status, body) = handle(&get("/api/runs", None), None, tmp.path());
        assert_eq!(status, 200);
        let runs: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
        let ids: Vec<_> = runs.iter().map(|r| r["id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["owner__app", "url/abc123"]);
        assert_eq!(runs[0]["has_model"], true);
        assert_eq!(runs[0]["has_report"], true);
        assert_eq!(runs[1]["surfaces"], 1);
    }

    #[test]
    fn lists_all_findings() {
        let tmp = fixture();
        let all = findings(tmp.path(), "");
        assert_eq!(rules(&all), ["sqli", "xss", "ssrf"]);
        assert_eq!(all[0].file.as_deref(), Some("app.py"));
        assert_eq!(all[0].line, Some(7));
        assert_eq!(all[1].verdict, "indeterminate");
    }

    #[test]
    fn filters_findings() {
        let tmp = fixture();
        let p = tmp.path();
        assert_eq!(rules(&findings(p, "?run=owner__app")), ["sqli", "xss"]);
        assert_eq!(rules(&findings(p, "?run=url%2Fabc123")), ["ssrf"]);
        assert_eq!(rules(&findings(p, "?verdict=confirmed")), ["sqli"]);
        assert_eq!(rules(&findings(p, "?level=error")), ["sqli", "ssrf"]);
        assert_eq!(rules(&findings(p, "?cwe=79")), ["xss"]);
        assert_eq!(rules(&findings(p, "?cwe=CWE-918")), ["ssrf"]);
        assert!(findings(p, "?level=error&verdict=indeterminate").is_empty());
    }

    #[test]
    fn rejects_traversal_and_unknown_runs() {
        let tmp = fixture();
        for run in [
            "..",
            "../etc",
            "owner__app/..",
            "%2e%2e%2f",
            "/etc",
            "stray",
            "url",
        ] {
            let req = get(&format!("/api/findings?run={run}"), None);
            assert_eq!(handle(&req, None, tmp.path()).0, 404, "{run}");
        }
        assert!(resolve_run(tmp.path(), "../owner__app").is_none());
    }

    #[test]
    fn bind_policy() {
        for host in ["127.0.0.1", "127.1.2.3", "::1", "[::1]", "localhost"] {
            assert!(check_bind(host, None).is_ok(), "{host}");
        }
        for host in ["0.0.0.0", "::", "192.168.1.10", "example.com"] {
            assert!(check_bind(host, None).is_err(), "{host}");
            assert!(check_bind(host, Some("t")).is_ok(), "{host}");
        }
    }

    #[test]
    fn requires_bearer_token() {
        let tmp = fixture();
        let token = Some("s3cret");
        for auth in [
            None,
            Some("Bearer wrong"),
            Some("s3cret"),
            Some("Basic s3cret"),
        ] {
            let (status, body) = handle(&get("/api/runs", auth), token, tmp.path());
            assert_eq!(status, 401, "{auth:?}");
            assert!(body.contains("unauthorized"));
        }
        let ok = handle(&get("/api/runs", Some("Bearer s3cret")), token, tmp.path());
        assert_eq!(ok.0, 200);
    }

    #[test]
    fn rejects_other_methods_and_paths() {
        let tmp = fixture();
        let mut post = get("/api/runs", None);
        post.method = "POST".into();
        assert_eq!(handle(&post, None, tmp.path()).0, 405);
        assert_eq!(handle(&get("/api/nope", None), None, tmp.path()).0, 404);
    }
}
