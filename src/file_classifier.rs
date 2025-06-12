use crate::security_patterns::Language;

pub struct FileClassifier;

impl FileClassifier {
    /// Classifies a file based on filename and content
    pub fn classify(filename: &str, content: &str) -> Language {
        // GitHub Actions workflows
        if Self::is_github_actions_workflow(filename, content) {
            return Language::Yaml;
        }

        // Kubernetes manifests
        if Self::is_kubernetes_manifest(filename, content) {
            return Language::Kubernetes;
        }

        // Docker Compose files
        if Self::is_docker_compose(filename, content) {
            return Language::Yaml;
        }

        // Terraform files
        if Self::is_terraform(filename, content) {
            return Language::Terraform;
        }

        // Fall back to extension-based detection
        Language::from_filename(filename)
    }

    fn is_github_actions_workflow(filename: &str, content: &str) -> bool {
        // Path-based detection
        if !filename.contains(".github/workflows/") {
            return false;
        }

        // File extension check
        if !(filename.ends_with(".yml") || filename.ends_with(".yaml")) {
            return false;
        }

        // Content-based validation
        let github_actions_patterns = ["on:", "jobs:", "runs-on:", "uses:", "steps:"];

        let content_lower = content.to_lowercase();
        github_actions_patterns
            .iter()
            .any(|&pattern| content_lower.contains(pattern))
    }

    fn is_kubernetes_manifest(filename: &str, content: &str) -> bool {
        // File extension check
        if !(filename.ends_with(".yml") || filename.ends_with(".yaml")) {
            return false;
        }

        // Kubernetes manifests must have these fields
        let required_k8s_patterns = ["apiVersion:", "kind:", "metadata:"];

        // At least one of these should be present
        let k8s_spec_patterns = ["spec:", "data:", "stringData:"];

        let has_required = required_k8s_patterns
            .iter()
            .all(|&pattern| content.contains(pattern));

        let has_spec = k8s_spec_patterns
            .iter()
            .any(|&pattern| content.contains(pattern));

        has_required && has_spec
    }

    fn is_docker_compose(filename: &str, content: &str) -> bool {
        // Filename-based detection
        if filename.ends_with("docker-compose.yml")
            || filename.ends_with("docker-compose.yaml")
            || filename.contains("compose.")
        {
            return true;
        }

        // Content-based detection for generic YAML files
        if !(filename.ends_with(".yml") || filename.ends_with(".yaml")) {
            return false;
        }

        // Docker Compose specific patterns
        let compose_patterns = ["version:", "services:"];

        compose_patterns
            .iter()
            .all(|&pattern| content.contains(pattern))
    }

    fn is_terraform(filename: &str, content: &str) -> bool {
        // File extension check
        if !(filename.ends_with(".tf") || filename.ends_with(".hcl")) {
            return false;
        }

        // Terraform-specific patterns
        let terraform_patterns = [
            "resource \"",
            "provider \"",
            "variable \"",
            "data \"",
            "module \"",
            "locals {",
            "output \"",
        ];

        terraform_patterns
            .iter()
            .any(|&pattern| content.contains(pattern))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_actions_detection() {
        let content = r#"
name: CI
on:
  push:
    branches: [ main ]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        "#;

        assert!(FileClassifier::is_github_actions_workflow(
            ".github/workflows/ci.yml",
            content
        ));

        // Should not match non-workflow YAML
        assert!(!FileClassifier::is_github_actions_workflow(
            "config.yml",
            content
        ));
    }

    #[test]
    fn test_kubernetes_detection() {
        let content = r#"
apiVersion: v1
kind: Pod
metadata:
  name: test-pod
spec:
  containers:
  - name: test
    image: nginx
        "#;

        assert!(FileClassifier::is_kubernetes_manifest("pod.yaml", content));
    }

    #[test]
    fn test_docker_compose_detection() {
        let content = r#"
version: '3.8'
services:
  web:
    image: nginx
    ports:
      - "80:80"
        "#;

        assert!(FileClassifier::is_docker_compose(
            "docker-compose.yml",
            content
        ));

        assert!(FileClassifier::is_docker_compose("services.yml", content));
    }
}
