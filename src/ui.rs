//! Static web UI (Runs and Findings views) embedded from `tools/web/public`
//! and served at `/` by `parsentry serve`. It reads data from [`crate::api`].

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "tools/web/public/"]
struct Assets;

/// Restrictive CSP: the UI only loads its own scripts/styles and calls `/api/`.
pub const CSP: &str = "default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'none'";

fn content_type(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        _ => "application/octet-stream",
    }
}

/// Look up an embedded asset for a request path; `/` maps to `index.html`.
pub fn asset(path: &str) -> Option<(&'static str, Vec<u8>)> {
    let path = path.split('?').next().unwrap_or("");
    let name = match path.trim_start_matches('/') {
        "" => "index.html",
        p => p,
    };
    let file = Assets::get(name)?;
    Some((content_type(name), file.data.into_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serves_index_and_assets() {
        let (ctype, body) = asset("/").unwrap();
        assert!(ctype.starts_with("text/html"));
        assert!(String::from_utf8(body).unwrap().contains("/app.js"));
        assert!(asset("/app.js").unwrap().0.starts_with("text/javascript"));
        assert!(asset("/style.css?v=1").is_some());
        assert!(asset("/missing.js").is_none());
        assert!(asset("/../Cargo.toml").is_none());
    }
}
