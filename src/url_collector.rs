use anyhow::Result;
use regex::Regex;
use std::path::{Path, PathBuf};

/// Maximum number of assets to collect from a single URL.
const MAX_ASSET_COUNT: usize = 50;

/// Maximum total download size in bytes (10 MB).
const MAX_TOTAL_SIZE: usize = 10 * 1024 * 1024;

/// Collects frontend assets from a URL for security analysis.
pub struct UrlAssetCollector {
    base_url: String,
    client: reqwest::Client,
}

/// A single collected frontend asset.
pub struct CollectedAsset {
    pub local_path: PathBuf,
    pub relative_path: String,
    pub source_url: String,
}

impl UrlAssetCollector {
    pub fn new(base_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("parsentry/0.1 (security scanner)")
            .build()?;
        Ok(Self {
            base_url: base_url.to_string(),
            client,
        })
    }

    /// Fetch the HTML page and collect all linked frontend assets.
    pub async fn collect(
        &self,
        asset_dir: &Path,
    ) -> Result<Vec<CollectedAsset>> {
        let response = self.client.get(&self.base_url).send().await?;
        if !response.status().is_success() {
            anyhow::bail!("HTTP {} fetching {}", response.status(), self.base_url);
        }
        let html = response.text().await?;

        // Save the original HTML
        let html_path = asset_dir.join("index.html");
        std::fs::write(&html_path, &html)?;
        let mut assets = vec![CollectedAsset {
            local_path: html_path,
            relative_path: "index.html".to_string(),
            source_url: self.base_url.clone(),
        }];

        // Extract linked asset URLs
        let linked = extract_asset_urls(&html);
        let mut total_size = html.len();

        for (url, kind) in linked {
            if assets.len() >= MAX_ASSET_COUNT {
                break;
            }
            let resolved = resolve_url(&self.base_url, &url);
            let filename = url_to_filename(&resolved, kind);
            let dest = asset_dir.join(&filename);

            match self.client.get(&resolved).send().await {
                Ok(resp) if resp.status().is_success() => {
                    let bytes = resp.bytes().await?;
                    total_size += bytes.len();
                    if total_size > MAX_TOTAL_SIZE {
                        break;
                    }
                    std::fs::write(&dest, &bytes)?;
                    assets.push(CollectedAsset {
                        local_path: dest,
                        relative_path: filename,
                        source_url: resolved,
                    });
                }
                Ok(resp) => {
                    log::warn!("Skipping {}: HTTP {}", resolved, resp.status());
                }
                Err(e) => {
                    log::warn!("Skipping {}: {}", resolved, e);
                }
            }
        }

        // Extract inline scripts and styles
        let inline = extract_inline_assets(&html);
        for (content, filename) in inline {
            if assets.len() >= MAX_ASSET_COUNT {
                break;
            }
            total_size += content.len();
            if total_size > MAX_TOTAL_SIZE {
                break;
            }
            let dest = asset_dir.join(&filename);
            std::fs::write(&dest, &content)?;
            assets.push(CollectedAsset {
                local_path: dest,
                relative_path: filename,
                source_url: format!("{}#inline", self.base_url),
            });
        }

        Ok(assets)
    }
}

#[derive(Clone, Copy)]
enum AssetKind {
    JavaScript,
    Stylesheet,
}

/// Extract linked asset URLs from HTML.
fn extract_asset_urls(html: &str) -> Vec<(String, AssetKind)> {
    let mut urls = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // <script src="...">
    let script_re = Regex::new(r#"<script[^>]+src\s*=\s*["']([^"']+)["']"#).unwrap();
    for cap in script_re.captures_iter(html) {
        let url = cap[1].to_string();
        if seen.insert(url.clone()) {
            urls.push((url, AssetKind::JavaScript));
        }
    }

    // <link rel="stylesheet" href="...">
    let link_re =
        Regex::new(r#"<link[^>]+rel\s*=\s*["']stylesheet["'][^>]+href\s*=\s*["']([^"']+)["']"#)
            .unwrap();
    for cap in link_re.captures_iter(html) {
        let url = cap[1].to_string();
        if seen.insert(url.clone()) {
            urls.push((url, AssetKind::Stylesheet));
        }
    }

    // <link href="..." rel="stylesheet"> (reversed attribute order)
    let link_re2 =
        Regex::new(r#"<link[^>]+href\s*=\s*["']([^"']+)["'][^>]+rel\s*=\s*["']stylesheet["']"#)
            .unwrap();
    for cap in link_re2.captures_iter(html) {
        let url = cap[1].to_string();
        if seen.insert(url.clone()) {
            urls.push((url, AssetKind::Stylesheet));
        }
    }

    urls
}

/// Extract inline <script> and <style> blocks as separate files.
fn extract_inline_assets(html: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    let mut script_idx = 0u32;
    let mut style_idx = 0u32;

    let script_re = Regex::new(r"<script[^>]*>([\s\S]*?)</script>").unwrap();
    for cap in script_re.captures_iter(html) {
        let content = cap[1].trim().to_string();
        if content.is_empty() || content.contains("src=") {
            continue;
        }
        let filename = format!("inline{}.js", if script_idx == 0 { String::new() } else { script_idx.to_string() });
        script_idx += 1;
        result.push((content, filename));
    }

    let style_re = Regex::new(r"<style[^>]*>([\s\S]*?)</style>").unwrap();
    for cap in style_re.captures_iter(html) {
        let content = cap[1].trim().to_string();
        if content.is_empty() {
            continue;
        }
        let filename = format!("inline{}.css", if style_idx == 0 { String::new() } else { style_idx.to_string() });
        style_idx += 1;
        result.push((content, filename));
    }

    result
}

/// Resolve a potentially relative URL against a base URL.
fn resolve_url(base: &str, relative: &str) -> String {
    // Absolute URL
    if relative.starts_with("http://") || relative.starts_with("https://") {
        return relative.to_string();
    }

    // Protocol-relative URL
    if relative.starts_with("//") {
        let scheme = base.split("://").next().unwrap_or("https");
        return format!("{}:{}", scheme, relative);
    }

    // Strip query/fragment from base for path resolution
    let base_path = base.split('?').next().unwrap_or(base);
    let base_path = base_path.split('#').next().unwrap_or(base_path);

    // Absolute path on same host
    if relative.starts_with('/') {
        let parts: Vec<&str> = base_path.splitn(2, "//").collect();
        if parts.len() == 2 {
            let rest = parts[1];
            let host_end = rest.find('/').unwrap_or(rest.len());
            let host = &rest[..host_end];
            return format!("{}//{}{}", parts[0], host, relative);
        }
        return relative.to_string();
    }

    // Relative path: strip filename from base, append relative
    if let Some(last_slash) = base_path.rfind('/') {
        let mut resolved = format!("{}{}", &base_path[..=last_slash], relative.trim_start_matches("./"));
        // Normalize path segments
        while let Some(idx) = resolved.find("/../") {
            if let Some(parent_end) = resolved[..idx].rfind('/') {
                resolved = format!("{}{}", &resolved[..parent_end], &resolved[idx + 3..]);
            } else {
                break;
            }
        }
        resolved = resolved.replace("/./", "/");
        resolved
    } else {
        relative.to_string()
    }
}

/// Generate a local filename from a URL.
fn url_to_filename(url: &str, _kind: AssetKind) -> String {
    // Strip query and fragment
    let path = url.split('?').next().unwrap_or(url);
    let path = path.split('#').next().unwrap_or(path);

    // Take the last path segment
    if let Some(segment) = path.rsplit('/').next() {
        if !segment.is_empty() && segment.contains('.') {
            return segment.to_string();
        }
    }

    // Fallback: hash the URL to produce a filename
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    format!("asset_{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_asset_urls_script() {
        let html = r#"<html><script src="/app.js"></script><script src="vendor.min.js"></script></html>"#;
        let urls = extract_asset_urls(html);
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0].0, "/app.js");
        assert_eq!(urls[1].0, "vendor.min.js");
    }

    #[test]
    fn test_extract_asset_urls_stylesheet() {
        let html = r#"<link rel="stylesheet" href="/style.css"><link href="theme.css" rel="stylesheet">"#;
        let urls = extract_asset_urls(html);
        assert_eq!(urls.len(), 2);
        assert!(urls.iter().any(|(u, _)| u == "/style.css"));
        assert!(urls.iter().any(|(u, _)| u == "theme.css"));
    }

    #[test]
    fn test_extract_asset_urls_dedup() {
        let html = r#"<script src="/app.js"></script><script src="/app.js"></script>"#;
        let urls = extract_asset_urls(html);
        assert_eq!(urls.len(), 1);
    }

    #[test]
    fn test_extract_inline_scripts() {
        let html = r#"<script>console.log("hi")</script><script src="skip.js"></script><script>alert(1)</script>"#;
        let inline = extract_inline_assets(html);
        assert_eq!(inline.len(), 2);
        assert_eq!(inline[0].1, "inline.js");
        assert!(inline[0].0.contains("console.log"));
        assert_eq!(inline[1].1, "inline1.js");
        assert!(inline[1].0.contains("alert"));
    }

    #[test]
    fn test_extract_inline_styles() {
        let html = r#"<style>body { color: red; }</style>"#;
        let inline = extract_inline_assets(html);
        assert_eq!(inline.len(), 1);
        assert_eq!(inline[0].1, "inline.css");
        assert!(inline[0].0.contains("color: red"));
    }

    #[test]
    fn test_resolve_url_absolute() {
        assert_eq!(
            resolve_url("https://example.com/page", "https://other.com/file.js"),
            "https://other.com/file.js"
        );
    }

    #[test]
    fn test_resolve_url_protocol_relative() {
        assert_eq!(
            resolve_url("https://example.com/page", "//cdn.example.com/lib.js"),
            "https://cdn.example.com/lib.js"
        );
    }

    #[test]
    fn test_resolve_url_absolute_path() {
        assert_eq!(
            resolve_url("https://example.com/app/page", "/assets/bundle.js"),
            "https://example.com/assets/bundle.js"
        );
    }

    #[test]
    fn test_resolve_url_relative_path() {
        assert_eq!(
            resolve_url("https://example.com/app/page", "bundle.js"),
            "https://example.com/app/bundle.js"
        );
    }

    #[test]
    fn test_resolve_url_parent_path() {
        assert_eq!(
            resolve_url("https://example.com/app/page", "../lib.js"),
            "https://example.com/lib.js"
        );
    }

    #[test]
    fn test_url_to_filename_with_path() {
        assert_eq!(
            url_to_filename("https://cdn.example.com/js/app.bundle.js?v=1", AssetKind::JavaScript),
            "app.bundle.js"
        );
    }

    #[test]
    fn test_url_to_filename_no_extension() {
        let name = url_to_filename("https://example.com/api/data", AssetKind::JavaScript);
        assert!(name.starts_with("asset_"));
    }
}
