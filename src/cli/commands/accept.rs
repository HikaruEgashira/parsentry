//! `parsentry report` — the acceptance gate surface workers submit findings
//! through instead of writing SARIF themselves.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

use parsentry_reports::{accept_finding, complete_surface};

use super::common::{cache_dir_for, write_stdout};

/// Resolve the surface directory, refusing ids that escape `reports/` and
/// surfaces that no scan has prompted for.
fn surface_dir(reports_dir: &Path, surface: &str) -> Result<PathBuf> {
    if surface.is_empty() || surface == "." || surface == ".." || surface.contains(['/', '\\']) {
        bail!("invalid surface id: {surface:?}");
    }
    let dir = reports_dir.join(surface);
    if !dir.is_dir() {
        bail!(
            "unknown surface {surface:?}: run `parsentry scan` first ({} missing)",
            dir.display()
        );
    }
    Ok(dir)
}

pub fn run_report_command(target: &str, surface: &str, finding: Option<&str>) -> Result<()> {
    let dir = surface_dir(&cache_dir_for(target).join("reports"), surface)?;
    match finding {
        Some(json) => {
            let id = accept_finding(&dir, json)?;
            write_stdout(&format!("accepted {id}\n"))
        }
        None => {
            complete_surface(&dir)?;
            write_stdout(&format!("surface {surface} complete\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn surface_ids_cannot_escape_reports_dir() {
        let tmp = TempDir::new().unwrap();
        for id in ["", ".", "..", "../x", "a/b", "a\\b"] {
            assert!(surface_dir(tmp.path(), id).is_err(), "{id:?}");
        }
    }

    #[test]
    fn unknown_surface_is_rejected() {
        let tmp = TempDir::new().unwrap();
        assert!(surface_dir(tmp.path(), "S-1").is_err());
        std::fs::create_dir(tmp.path().join("S-1")).unwrap();
        assert_eq!(
            surface_dir(tmp.path(), "S-1").unwrap(),
            tmp.path().join("S-1")
        );
    }
}
