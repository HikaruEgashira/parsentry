use crate::threat_model::ThreatModel;

/// Render a threat model as a Markdown document for explainability.
pub fn render_threat_model_md(model: &ThreatModel) -> String {
    let mut md = String::new();

    md.push_str(&format!("# Threat Model: {}\n\n", model.repository));
    md.push_str(&format!("Generated: {}\n\n", model.generated_at));
    md.push_str(&format!("**Application type**: {}\n\n", model.app_type));
    md.push_str(&format!("## Summary\n\n{}\n\n", model.summary));

    md.push_str(&format!(
        "## Overview\n\n- **Attack surfaces identified**: {}\n\n",
        model.surfaces.len(),
    ));

    md.push_str("## Attack Surfaces\n\n");

    for surface in &model.surfaces {
        md.push_str(&format!(
            "### {} — {} `{}`\n\n",
            surface.id, surface.kind, surface.identifier
        ));
        md.push_str(&format!("{}\n\n", surface.description));

        if !surface.locations.is_empty() {
            md.push_str("**Locations**:\n");
            for loc in &surface.locations {
                md.push_str(&format!("- `{}`\n", loc));
            }
            md.push('\n');
        }

        md.push_str("---\n\n");
    }

    md
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::threat_model::AttackSurface;

    #[test]
    fn test_render_basic() {
        let model = ThreatModel {
            repository: "test/repo".to_string(),
            generated_at: "2025-01-01T00:00:00Z".to_string(),
            app_type: "web_application".to_string(),
            summary: "A Flask web app with user-facing endpoints.".to_string(),
            surfaces: vec![AttackSurface {
                id: "SURFACE-001".to_string(),
                kind: "endpoint".to_string(),
                identifier: "POST /api/users".to_string(),
                locations: vec!["src/routes/users.py".to_string()],
                description: "User registration endpoint".to_string(),
            }],
        };

        let md = render_threat_model_md(&model);

        assert!(md.contains("# Threat Model: test/repo"));
        assert!(md.contains("SURFACE-001"));
        assert!(md.contains("POST /api/users"));
        assert!(md.contains("web_application"));
    }

    #[test]
    fn test_render_locations_section() {
        // Kills `!` deletion in `if !surface.locations.is_empty()`
        let with_locations = ThreatModel {
            repository: "r".to_string(),
            generated_at: "t".to_string(),
            app_type: "web".to_string(),
            summary: "s".to_string(),
            surfaces: vec![AttackSurface {
                id: "S-1".to_string(),
                kind: "endpoint".to_string(),
                identifier: "GET /".to_string(),
                locations: vec!["src/app.py".to_string()],
                description: "d".to_string(),
            }],
        };
        let md = render_threat_model_md(&with_locations);
        assert!(md.contains("**Locations**:"));
        assert!(md.contains("- `src/app.py`"));

        // Empty locations → no Locations section
        let no_locations = ThreatModel {
            repository: "r".to_string(),
            generated_at: "t".to_string(),
            app_type: "web".to_string(),
            summary: "s".to_string(),
            surfaces: vec![AttackSurface {
                id: "S-2".to_string(),
                kind: "endpoint".to_string(),
                identifier: "GET /".to_string(),
                locations: vec![],
                description: "d".to_string(),
            }],
        };
        let md2 = render_threat_model_md(&no_locations);
        assert!(!md2.contains("**Locations**:"));
    }
}
