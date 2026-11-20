//! `parsentry scan` — a single, state-driven entry point for the pipeline.
//!
//! Each invocation inspects the per-target cache and runs (or emits the
//! prompt for) exactly one phase:
//!
//! 1. **Model** — `model.json` is missing, invalid, or was built from repo
//!    metadata that has since changed. Emits the threat model prompt.
//! 2. **Surfaces** — some surfaces of the current model have no valid,
//!    up-to-date SARIF result. Emits per-surface prompts + orchestrator.
//! 3. **Report** — every surface is done. Merges SARIF and writes the report.
//!
//! Every emitted prompt ends with an instruction to re-run `parsentry scan`,
//! so an agent runtime re-running it drives the pipeline to completion.
//! Piping (`parsentry scan <target> | <agent>`) still works but is deprecated.
//! The scan mode falls out of the cache state: a cold cache is a full scan,
//! changed sources only re-run their surfaces (incremental), and a changed
//! model input snapshots the previous model and its results so the report
//! diffs the findings of both models.

use anyhow::{Context, Result};
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use crate::cli::args::RuntimeKind;
use crate::cli::ui::StatusPrinter;
use crate::prompt::{
    SCAN_COMPLETE_MARKER, SurfacePrompt, build_all_surface_prompts, build_orchestrator_prompt,
    continuation_instructions,
};
use crate::runtime::{AgentRuntime, SidecarRuntime};

use parsentry_core::{RepoMetadata, ThreatModel};
use parsentry_reports::{SarifReport, merge_sarif_files};
use sha2::{Digest, Sha256};

use super::common::{
    build_threat_model_cli_prompt, cache_base, cache_dir_for, is_network_target, locate_repository,
    repo_name_from_target, write_stdout,
};
use super::report::{render_pdf, render_report_markdown};

/// Printed to stderr when stdout is piped (e.g. `parsentry scan | claude -p`).
pub const PIPE_MODE_DEPRECATION: &str = "piping `parsentry scan` into an agent (`parsentry scan | claude -p`) is deprecated; \
let the agent runtime drive the scan loop instead (install the skill: `npx skills add HikaruEgashira/parsentry`)";

/// Files in the per-target cache directory that carry scan state.
struct CacheLayout {
    root: PathBuf,
}

impl CacheLayout {
    fn new(root: PathBuf) -> Self {
        Self { root }
    }
    fn model(&self) -> PathBuf {
        self.root.join("model.json")
    }
    /// Hash of the repo metadata the current `model.json` was built from.
    fn model_key(&self) -> PathBuf {
        self.root.join("model.key")
    }
    /// Hash of the repo metadata the last emitted model prompt was built from.
    /// Promoted to `model.key` once the agent writes `model.json`.
    fn model_pending_key(&self) -> PathBuf {
        self.root.join("model.pending_key")
    }
    fn previous_dir(&self) -> PathBuf {
        self.root.join("previous")
    }
    fn previous_model(&self) -> PathBuf {
        self.previous_dir().join("model.json")
    }
    fn previous_merged(&self) -> PathBuf {
        self.previous_dir().join("merged.sarif.json")
    }
    fn reports(&self) -> PathBuf {
        self.root.join("reports")
    }
    fn merged(&self) -> PathBuf {
        self.root.join("merged.sarif.json")
    }
    fn report_md(&self) -> PathBuf {
        self.root.join("report.md")
    }
}

/// Why the threat model has to be (re)generated.
#[derive(Debug, PartialEq, Eq)]
enum ModelReason {
    Missing,
    Invalid(String),
    InputChanged,
}

/// The phase a scan invocation runs, derived purely from cache state.
#[derive(Debug)]
enum Phase {
    Model(ModelReason),
    Surfaces {
        model: ThreatModel,
        pending: Vec<SurfacePrompt>,
        cached: usize,
    },
    Report {
        model: ThreatModel,
    },
}

fn hex_sha256(data: &str) -> String {
    format!("{:x}", Sha256::digest(data.as_bytes()))
}

/// Promote the pending model key once the agent has written `model.json`,
/// and adopt caches created before model keys existed.
fn reconcile_model_key(layout: &CacheLayout, current_key: &str) -> Result<()> {
    if !layout.model().exists() {
        return Ok(());
    }
    if layout.model_pending_key().exists() {
        std::fs::rename(layout.model_pending_key(), layout.model_key())?;
    } else if !layout.model_key().exists() {
        std::fs::write(layout.model_key(), current_key)?;
    }
    Ok(())
}

fn load_model(
    layout: &CacheLayout,
    current_key: &str,
) -> std::result::Result<ThreatModel, ModelReason> {
    let json = std::fs::read_to_string(layout.model()).map_err(|_| ModelReason::Missing)?;
    let model: ThreatModel =
        serde_json::from_str(&json).map_err(|e| ModelReason::Invalid(e.to_string()))?;
    let stored_key = std::fs::read_to_string(layout.model_key()).unwrap_or_default();
    if stored_key.trim() != current_key {
        return Err(ModelReason::InputChanged);
    }
    Ok(model)
}

/// A surface is done when its SARIF parses and was produced for the same
/// cache key (surface definition + source contents). Invalid SARIF is
/// treated as pending so the surface is re-analyzed instead of breaking
/// the merge.
fn is_cached(reports_dir: &Path, sp: &SurfacePrompt) -> bool {
    let surface_dir = reports_dir.join(&sp.surface_id);
    let key_matches = std::fs::read_to_string(surface_dir.join(".cache_key"))
        .is_ok_and(|k| k.trim() == sp.cache_key);
    key_matches
        && std::fs::read_to_string(surface_dir.join("result.sarif.json"))
            .ok()
            .and_then(|s| serde_json::from_str::<SarifReport>(&s).ok())
            .is_some()
}

fn determine_phase(layout: &CacheLayout, root_dir: &Path, current_key: &str) -> Phase {
    let model = match load_model(layout, current_key) {
        Ok(model) => model,
        Err(reason) => return Phase::Model(reason),
    };
    let (cached, pending): (Vec<_>, Vec<_>) = build_all_surface_prompts(&model, root_dir)
        .into_iter()
        .partition(|sp| is_cached(&layout.reports(), sp));
    if pending.is_empty() {
        Phase::Report { model }
    } else {
        Phase::Surfaces {
            model,
            pending,
            cached: cached.len(),
        }
    }
}

/// Move the current model and its merged results to `previous/` so the next
/// report can diff the findings of the old and new models.
fn snapshot_previous(layout: &CacheLayout) -> Result<()> {
    let previous = layout.previous_dir();
    if previous.exists() {
        std::fs::remove_dir_all(&previous)?;
    }
    std::fs::create_dir_all(&previous)?;
    std::fs::rename(layout.model(), layout.previous_model())?;
    if layout.merged().exists() {
        std::fs::rename(layout.merged(), layout.previous_merged())?;
    }
    std::fs::remove_file(layout.model_key()).ok();
    Ok(())
}

/// SARIF files of surfaces in the current model. Results of surfaces that
/// the model no longer contains are deliberately excluded.
pub fn current_sarif_files(reports_dir: &Path, model: &ThreatModel) -> Vec<PathBuf> {
    model
        .surfaces
        .iter()
        .map(|s| reports_dir.join(&s.id).join("result.sarif.json"))
        .filter(|p| p.exists())
        .collect()
}

pub async fn run_scan_command(
    target: &str,
    output: Option<&str>,
    runtime: RuntimeKind,
) -> Result<()> {
    let printer = StatusPrinter::with_service(repo_name_from_target(target));
    if !std::io::stdout().is_terminal() {
        printer.warning("Deprecated", PIPE_MODE_DEPRECATION);
    }
    let (root_dir, _repo_name) = locate_repository(target, &printer).await?;

    let mut repo_metadata = RepoMetadata::collect(&root_dir)?;
    if is_network_target(target) {
        repo_metadata.source_url = Some(target.to_string());
    }
    printer.status(
        "Collected",
        &format!(
            "{} source files across {} languages",
            repo_metadata.total_files,
            repo_metadata.languages.len()
        ),
    );

    let layout = CacheLayout::new(cache_dir_for(target));
    std::fs::create_dir_all(&layout.root)?;
    let model_input_key = hex_sha256(&repo_metadata.to_prompt_context());
    reconcile_model_key(&layout, &model_input_key)?;

    if runtime == RuntimeKind::Claude {
        let mut sidecar = SidecarRuntime::from_env()?;
        return drive_runtime(
            &layout,
            &root_dir,
            &repo_metadata,
            &model_input_key,
            &mut sidecar,
            output,
            &printer,
        );
    }

    let parsentry_bin = std::env::current_exe()?;
    let continuation = continuation_instructions(target, &cache_base(), &parsentry_bin);

    match determine_phase(&layout, &root_dir, &model_input_key) {
        Phase::Model(reason) => {
            prepare_model_phase(&layout, &reason, &printer)?;
            std::fs::write(layout.model_pending_key(), &model_input_key)?;

            let mut prompt = model_prompt(&layout, &repo_metadata);
            prompt.push_str(&continuation);
            write_stdout(&prompt)?;
            printer.success(
                "Prompt",
                &format!(
                    "threat model prompt emitted (output → {})",
                    layout.model().display()
                ),
            );
        }
        Phase::Surfaces {
            model,
            pending,
            cached,
        } => {
            // A previous model's results mean we are re-scanning for a model
            // diff; a previous report means only changed surfaces re-run.
            let mode = if layout.previous_merged().exists() {
                "model diff"
            } else if layout.merged().exists() || cached > 0 {
                "incremental"
            } else {
                "full"
            };
            printer.status(
                "Phase",
                &format!(
                    "surfaces ({mode}: {} pending, {cached} cached of {} surfaces)",
                    pending.len(),
                    model.total_surfaces()
                ),
            );
            let reports_dir = layout.reports();
            printer.section("Prompts");
            for sp in &pending {
                let (prompt_path, _) = prepare_surface(&reports_dir, sp)?;
                printer.bullet(&format!("{} → {}", sp.surface_id, prompt_path.display()));
            }

            let orchestrator = build_orchestrator_prompt(
                &pending,
                &reports_dir,
                target,
                &cache_base(),
                &parsentry_bin,
            );
            let orchestrator_path = reports_dir.join("orchestrator.prompt.md");
            std::fs::write(&orchestrator_path, &orchestrator)?;
            printer.bullet(&format!("orchestrator → {}", orchestrator_path.display()));
            write_stdout(&format!("{}\n", orchestrator))?;
        }
        Phase::Report { model } => {
            run_report_phase(&layout, &model, output, &printer)?;
        }
    }
    Ok(())
}

/// The threat model prompt, pointing at the previous model when diffing.
fn model_prompt(layout: &CacheLayout, repo_metadata: &RepoMetadata) -> String {
    let mut prompt = build_threat_model_cli_prompt(repo_metadata, &layout.model());
    if layout.previous_model().exists() {
        prompt.push_str(&format!(
            "\nA previous threat model exists at {}. Read it first. Keep the same \
             surface `id` for every surface that still exists so its cached results \
             stay comparable; assign new ids only to new surfaces.\n",
            layout.previous_model().display()
        ));
    }
    prompt
}

/// Handle the cache side effects of a model phase before (re)generating it.
fn prepare_model_phase(
    layout: &CacheLayout,
    reason: &ModelReason,
    printer: &StatusPrinter,
) -> Result<()> {
    match reason {
        ModelReason::Missing if layout.previous_model().exists() => {
            printer.status("Phase", "model (model diff: awaiting new threat model)")
        }
        ModelReason::Missing => printer.status("Phase", "model (full scan)"),
        ModelReason::Invalid(e) => {
            printer.warning(
                "Model",
                &format!("model.json is invalid ({e}), regenerating"),
            );
            std::fs::remove_file(layout.model())?;
        }
        ModelReason::InputChanged => {
            snapshot_previous(layout)?;
            printer.status(
                "Phase",
                "model (model diff: repository changed, previous model snapshotted)",
            );
        }
    }
    Ok(())
}

/// Write a pending surface's prompt and cache key, dropping any stale SARIF.
/// Returns the prompt path and the full prompt text.
fn prepare_surface(reports_dir: &Path, sp: &SurfacePrompt) -> Result<(PathBuf, String)> {
    let surface_dir = reports_dir.join(&sp.surface_id);
    std::fs::create_dir_all(&surface_dir)?;
    let sarif_path = surface_dir.join("result.sarif.json");
    // Drop stale results so a failed worker cannot leave an old
    // SARIF that looks current.
    std::fs::remove_file(&sarif_path).ok();
    let prompt_path = surface_dir.join("prompt.md");
    let full_prompt = format!(
        "{}\n\nWrite the SARIF JSON output to: {}\n\
         Write ONLY valid JSON. No markdown, no code fences, no explanation.\n",
        sp.prompt,
        sarif_path.display()
    );
    std::fs::write(&prompt_path, &full_prompt)?;
    std::fs::write(surface_dir.join(".cache_key"), &sp.cache_key)?;
    Ok((prompt_path, full_prompt))
}

/// Surface ids become cache directory names; reject anything path-like.
fn is_safe_surface_id(id: &str) -> bool {
    !id.is_empty()
        && id != "."
        && id != ".."
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

/// Parse and validate threat model JSON returned by a runtime.
fn validate_model(json: &str) -> Result<ThreatModel> {
    let model: ThreatModel = serde_json::from_str(json.trim())
        .context("agent runtime returned an invalid threat model")?;
    if let Some(bad) = model.surfaces.iter().find(|s| !is_safe_surface_id(&s.id)) {
        anyhow::bail!("agent runtime returned an invalid surface id {:?}", bad.id);
    }
    Ok(model)
}

/// Parse and validate SARIF returned by a runtime; returns it pretty-printed.
fn validate_sarif(surface_id: &str, json: &str) -> Result<String> {
    let report: SarifReport = serde_json::from_str(json.trim())
        .with_context(|| format!("agent runtime returned invalid SARIF for {surface_id}"))?;
    Ok(serde_json::to_string_pretty(&report)?)
}

/// Drive every phase through `runtime` until the report is written.
/// Rust owns validation and all cache writes; the runtime only returns text.
fn drive_runtime(
    layout: &CacheLayout,
    root_dir: &Path,
    repo_metadata: &RepoMetadata,
    model_input_key: &str,
    runtime: &mut dyn AgentRuntime,
    output: Option<&str>,
    printer: &StatusPrinter,
) -> Result<()> {
    // model → surfaces → report, plus one retry for an invalidated model.
    for _ in 0..4 {
        reconcile_model_key(layout, model_input_key)?;
        match determine_phase(layout, root_dir, model_input_key) {
            Phase::Model(reason) => {
                prepare_model_phase(layout, &reason, printer)?;
                std::fs::write(layout.model_pending_key(), model_input_key)?;
                let model =
                    validate_model(&runtime.run_model(&model_prompt(layout, repo_metadata))?)?;
                std::fs::write(layout.model(), serde_json::to_string_pretty(&model)?)?;
                printer.success(
                    "Model",
                    &format!(
                        "{} surfaces → {}",
                        model.total_surfaces(),
                        layout.model().display()
                    ),
                );
            }
            Phase::Surfaces {
                pending, cached, ..
            } => {
                printer.status(
                    "Phase",
                    &format!("surfaces ({} pending, {cached} cached)", pending.len()),
                );
                let reports_dir = layout.reports();
                for sp in &pending {
                    let (_, prompt) = prepare_surface(&reports_dir, sp)?;
                    let sarif = validate_sarif(
                        &sp.surface_id,
                        &runtime.run_surface(&sp.surface_id, &prompt)?,
                    )?;
                    let sarif = validate_sarif(
                        &sp.surface_id,
                        &runtime.run_verify(&sp.surface_id, &sarif)?,
                    )?;
                    let sarif_path = reports_dir.join(&sp.surface_id).join("result.sarif.json");
                    std::fs::write(&sarif_path, sarif)?;
                    printer.bullet(&format!("{} → {}", sp.surface_id, sarif_path.display()));
                }
            }
            Phase::Report { model } => return run_report_phase(layout, &model, output, printer),
        }
    }
    anyhow::bail!("agent runtime did not converge to a report")
}

fn run_report_phase(
    layout: &CacheLayout,
    model: &ThreatModel,
    output: Option<&str>,
    printer: &StatusPrinter,
) -> Result<()> {
    let files = current_sarif_files(&layout.reports(), model);
    let baseline = layout.previous_merged();
    let baseline = baseline.exists().then_some(baseline.as_path());
    printer.status(
        "Phase",
        if baseline.is_some() {
            "report (model diff against previous model results)"
        } else {
            "report"
        },
    );

    // Never leave a stale merged report behind if the merge fails.
    std::fs::remove_file(layout.merged()).ok();
    if files.is_empty() {
        printer.warning("Report", "threat model has no analyzable surfaces");
    }
    let merged = merge_sarif_files(&files, baseline)?;

    let tmp = layout.root.join("merged.sarif.json.tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(&merged)?)
        .context("failed to write merged.sarif.json")?;
    std::fs::rename(&tmp, layout.merged())?;
    std::fs::write(
        layout.report_md(),
        render_report_markdown(&merged, baseline.is_some()),
    )
    .context("failed to write report.md")?;

    let count = |state: Option<&str>| {
        merged
            .runs
            .iter()
            .flat_map(|r| &r.results)
            .filter(|r| state.is_none_or(|s| r.baseline_state.as_deref() == Some(s)))
            .count()
    };
    let findings = count(None) - count(Some("absent"));
    printer.success(
        "Merged",
        &format!("{findings} findings → {}", layout.merged().display()),
    );

    let pdf = output
        .map(|o| render_pdf(&layout.root, o, printer))
        .transpose()?;

    let mut summary = format!(
        "All phases are complete.\n\n\
         - Findings: {findings}\n\
         - Merged SARIF: {}\n\
         - Report: {}\n",
        layout.merged().display(),
        layout.report_md().display(),
    );
    if let Some(pdf) = &pdf {
        summary.push_str(&format!("- PDF: {}\n", pdf.display()));
    }
    if baseline.is_some() {
        summary.push_str(&format!(
            "- Model diff: {} new, {} absent compared with the previous threat model ({})\n\n\
             Read the `Model Diff` section of the report together with both threat models \
             ({} and {}) and explain why each model detected what the other did not.\n",
            count(Some("new")),
            count(Some("absent")),
            layout.previous_model().display(),
            layout.previous_model().display(),
            layout.model().display(),
        ));
    }
    summary.push_str(&format!(
        "\nSummarize the report for the user.\n{}\n",
        SCAN_COMPLETE_MARKER
    ));
    write_stdout(&summary)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use parsentry_core::AttackSurface;
    use tempfile::TempDir;

    const VALID_SARIF: &str = r#"{"$schema":"s","version":"2.1.0","runs":[{"tool":{"driver":{"name":"t","version":"1"}},"results":[]}]}"#;

    fn model_json(ids: &[&str]) -> String {
        let model = ThreatModel {
            repository: "r".into(),
            generated_at: String::new(),
            app_type: "web_application".into(),
            summary: "s".into(),
            surfaces: ids
                .iter()
                .map(|id| AttackSurface {
                    id: id.to_string(),
                    kind: "endpoint".into(),
                    identifier: format!("GET /{id}"),
                    locations: vec!["app.py".into()],
                    description: "d".into(),
                })
                .collect(),
        };
        serde_json::to_string(&model).unwrap()
    }

    fn setup() -> (TempDir, TempDir, CacheLayout) {
        let repo = TempDir::new().unwrap();
        std::fs::write(repo.path().join("app.py"), "print(1)\n").unwrap();
        let cache = TempDir::new().unwrap();
        let layout = CacheLayout::new(cache.path().to_path_buf());
        (repo, cache, layout)
    }

    fn complete_surfaces(layout: &CacheLayout, repo: &Path) {
        let model: ThreatModel =
            serde_json::from_str(&std::fs::read_to_string(layout.model()).unwrap()).unwrap();
        for sp in build_all_surface_prompts(&model, repo) {
            let dir = layout.reports().join(&sp.surface_id);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join(".cache_key"), &sp.cache_key).unwrap();
            std::fs::write(dir.join("result.sarif.json"), VALID_SARIF).unwrap();
        }
    }

    #[test]
    fn pipe_mode_deprecation_mentions_pipe_and_runtime() {
        assert!(PIPE_MODE_DEPRECATION.contains("deprecated"));
        assert!(PIPE_MODE_DEPRECATION.contains("claude -p"));
        assert!(PIPE_MODE_DEPRECATION.contains("runtime"));
    }

    #[test]
    fn cold_cache_starts_with_model_phase() {
        let (repo, _c, layout) = setup();
        assert!(matches!(
            determine_phase(&layout, repo.path(), "k1"),
            Phase::Model(ModelReason::Missing)
        ));
    }

    #[test]
    fn written_model_promotes_pending_key_and_moves_to_surfaces() {
        let (repo, _c, layout) = setup();
        std::fs::write(layout.model_pending_key(), "k1").unwrap();
        std::fs::write(layout.model(), model_json(&["S-1", "S-2"])).unwrap();
        reconcile_model_key(&layout, "k1").unwrap();
        assert!(!layout.model_pending_key().exists());
        match determine_phase(&layout, repo.path(), "k1") {
            Phase::Surfaces {
                pending, cached, ..
            } => {
                assert_eq!(pending.len(), 2);
                assert_eq!(cached, 0);
            }
            other => panic!("unexpected phase {other:?}"),
        }
    }

    #[test]
    fn completed_surfaces_move_to_report_and_source_change_is_incremental() {
        let (repo, _c, layout) = setup();
        std::fs::write(layout.model(), model_json(&["S-1"])).unwrap();
        reconcile_model_key(&layout, "k1").unwrap();
        complete_surfaces(&layout, repo.path());
        assert!(matches!(
            determine_phase(&layout, repo.path(), "k1"),
            Phase::Report { .. }
        ));

        std::fs::write(repo.path().join("app.py"), "print(2)\n").unwrap();
        assert!(matches!(
            determine_phase(&layout, repo.path(), "k1"),
            Phase::Surfaces { .. }
        ));
    }

    #[test]
    fn invalid_sarif_is_pending() {
        let (repo, _c, layout) = setup();
        std::fs::write(layout.model(), model_json(&["S-1"])).unwrap();
        reconcile_model_key(&layout, "k1").unwrap();
        complete_surfaces(&layout, repo.path());
        std::fs::write(layout.reports().join("S-1/result.sarif.json"), "{oops").unwrap();
        assert!(matches!(
            determine_phase(&layout, repo.path(), "k1"),
            Phase::Surfaces { .. }
        ));
    }

    #[test]
    fn invalid_model_is_regenerated() {
        let (repo, _c, layout) = setup();
        std::fs::write(layout.model(), "not json").unwrap();
        assert!(matches!(
            determine_phase(&layout, repo.path(), "k1"),
            Phase::Model(ModelReason::Invalid(_))
        ));
    }

    #[test]
    fn changed_model_input_snapshots_previous_results() {
        let (repo, _c, layout) = setup();
        std::fs::write(layout.model(), model_json(&["S-1"])).unwrap();
        reconcile_model_key(&layout, "k1").unwrap();
        std::fs::write(layout.merged(), VALID_SARIF).unwrap();

        assert!(matches!(
            determine_phase(&layout, repo.path(), "k2"),
            Phase::Model(ModelReason::InputChanged)
        ));
        snapshot_previous(&layout).unwrap();
        assert!(!layout.model().exists());
        assert!(layout.previous_model().exists());
        assert!(layout.previous_merged().exists());
        assert!(matches!(
            determine_phase(&layout, repo.path(), "k2"),
            Phase::Model(ModelReason::Missing)
        ));
    }

    #[test]
    fn legacy_cache_without_key_is_adopted() {
        let (repo, _c, layout) = setup();
        std::fs::write(layout.model(), model_json(&["S-1"])).unwrap();
        reconcile_model_key(&layout, "k1").unwrap();
        assert!(matches!(
            determine_phase(&layout, repo.path(), "k1"),
            Phase::Surfaces { .. }
        ));
    }

    #[test]
    fn report_ignores_surfaces_removed_from_model() {
        let (_repo, _c, layout) = setup();
        let model: ThreatModel = serde_json::from_str(&model_json(&["S-1"])).unwrap();
        for id in ["S-1", "S-OLD"] {
            let dir = layout.reports().join(id);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("result.sarif.json"), VALID_SARIF).unwrap();
        }
        let files = current_sarif_files(&layout.reports(), &model);
        assert_eq!(files, vec![layout.reports().join("S-1/result.sarif.json")]);
    }

    struct MockRuntime {
        calls: Vec<String>,
    }

    impl AgentRuntime for MockRuntime {
        fn run_model(&mut self, _prompt: &str) -> Result<String> {
            self.calls.push("model".into());
            Ok(model_json(&["S-1", "S-2"]))
        }
        fn run_surface(&mut self, surface_id: &str, _prompt: &str) -> Result<String> {
            self.calls.push(format!("surface:{surface_id}"));
            Ok(VALID_SARIF.into())
        }
        fn run_verify(&mut self, surface_id: &str, sarif: &str) -> Result<String> {
            self.calls.push(format!("verify:{surface_id}"));
            Ok(sarif.into())
        }
    }

    #[test]
    fn mock_runtime_drives_cold_cache_to_merged_report() {
        let (repo, _c, layout) = setup();
        let metadata = RepoMetadata::collect(repo.path()).unwrap();
        let mut rt = MockRuntime { calls: vec![] };
        drive_runtime(
            &layout,
            repo.path(),
            &metadata,
            "k1",
            &mut rt,
            None,
            &StatusPrinter::new(),
        )
        .unwrap();
        assert_eq!(
            rt.calls,
            [
                "model",
                "surface:S-1",
                "verify:S-1",
                "surface:S-2",
                "verify:S-2"
            ]
        );
        assert_eq!(std::fs::read_to_string(layout.model_key()).unwrap(), "k1");
        assert!(layout.reports().join("S-2/result.sarif.json").exists());
        let merged: SarifReport =
            serde_json::from_str(&std::fs::read_to_string(layout.merged()).unwrap()).unwrap();
        assert_eq!(merged.runs.len(), 1);
        assert!(layout.report_md().exists());

        // A warm cache goes straight to the report without runtime calls.
        rt.calls.clear();
        drive_runtime(
            &layout,
            repo.path(),
            &metadata,
            "k1",
            &mut rt,
            None,
            &StatusPrinter::new(),
        )
        .unwrap();
        assert!(rt.calls.is_empty());
    }

    #[test]
    fn runtime_output_is_validated() {
        assert!(validate_model("not json").is_err());
        assert!(validate_model(&model_json(&["../evil"])).is_err());
        assert!(validate_model(&model_json(&["S-1"])).is_ok());
        assert!(validate_sarif("S-1", "{oops").is_err());
        assert!(validate_sarif("S-1", VALID_SARIF).is_ok());
    }
}
