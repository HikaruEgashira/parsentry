use crate::model::ThreatModel;

/// Render a threat model as a Markdown document for explainability.
pub fn render_threat_model_md(model: &ThreatModel) -> String {
    let mut md = String::new();

    md.push_str(&format!("# Threat Model: {}\n\n", model.repository));
    md.push_str(&format!("Generated: {}\n\n", model.generated_at));
    md.push_str(&format!("## Summary\n\n{}\n\n", model.summary));

    md.push_str(&format!(
        "## Overview\n\n- **Threats identified**: {}\n- **Detection queries**: {}\n\n",
        model.threats.len(),
        model.total_queries(),
    ));

    md.push_str("## Threats\n\n");

    for threat in &model.threats {
        md.push_str(&format!("### {} — {}\n\n", threat.id, threat.category));
        md.push_str(&format!("**Language**: {:?}\n\n", threat.language));
        md.push_str(&format!("**Rationale**: {}\n\n", threat.rationale));

        if !threat.attack_vectors.is_empty() {
            md.push_str(&format!(
                "**MITRE ATT&CK**: {}\n\n",
                threat.attack_vectors.join(", ")
            ));
        }

        if !threat.attack_surface.is_empty() {
            md.push_str("**Attack Surface**:\n");
            for surface in &threat.attack_surface {
                md.push_str(&format!("- `{}`\n", surface));
            }
            md.push('\n');
        }

        if !threat.queries.is_empty() {
            md.push_str("**Detection Queries**:\n\n");
            for query in &threat.queries {
                md.push_str(&format!(
                    "- **[{}]** {} ({})\n  ```scheme\n  {}\n  ```\n\n",
                    query.par_type, query.description, query.query_type, query.query
                ));
            }
        }

        md.push_str("---\n\n");
    }

    md
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ThreatEntry, ThreatQuery};
    use parsentry_core::Language;

    #[test]
    fn test_render_basic() {
        let model = ThreatModel {
            repository: "test/repo".to_string(),
            generated_at: "2025-01-01T00:00:00Z".to_string(),
            summary: "A Flask web app with SQL injection risks.".to_string(),
            threats: vec![ThreatEntry {
                id: "THREAT-001".to_string(),
                category: "SQL Injection".to_string(),
                rationale: "Uses raw SQL queries with string formatting.".to_string(),
                attack_vectors: vec!["T1190".to_string()],
                attack_surface: vec!["src/db.py".to_string()],
                language: Language::Python,
                queries: vec![ThreatQuery {
                    par_type: "resource".to_string(),
                    query_type: "reference".to_string(),
                    query: r#"(call function: (attribute attribute: (identifier) @attr (#eq? @attr "execute"))) @call"#.to_string(),
                    description: "SQL execute calls".to_string(),
                }],
            }],
        };

        let md = render_threat_model_md(&model);

        assert!(md.contains("# Threat Model: test/repo"));
        assert!(md.contains("THREAT-001"));
        assert!(md.contains("SQL Injection"));
        assert!(md.contains("execute"));
    }
}
