use anyhow::{Result, bail};

use crate::cli::commands::common::{cache_dir_for, write_stdout};
use parsentry_reports::{Acceptance, accept_finding};

/// `parsentry report --surface <id> --finding <json>`: validate one finding
/// and append it to `reports/<id>/result.sarif.json`.
pub fn run_report_command(target: &str, surface: &str, finding: &str) -> Result<()> {
    if surface.is_empty()
        || !surface
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        bail!("invalid surface id: {surface:?}");
    }
    let surface_dir = cache_dir_for(target).join("reports").join(surface);
    match accept_finding(&surface_dir, finding)? {
        Acceptance::Accepted(id) => write_stdout(&format!("accepted {id}\n")),
        Acceptance::Duplicate(id) => write_stdout(&format!("duplicate {id}\n")),
    }
}
