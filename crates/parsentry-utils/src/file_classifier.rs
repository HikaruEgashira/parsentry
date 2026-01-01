use parsentry_core::Language;

pub struct FileClassifier;

impl FileClassifier {
    /// Classifies a file based on filename and content
    pub fn classify(filename: &str, content: &str) -> Language {
        // CI/CD platform detection
        if Self::is_gitlab_ci(filename, content) {
            return Language::Yaml;
        }
        if Self::is_circleci(filename, content) {
            return Language::Yaml;
        }
        if Self::is_travis_ci(filename, content) {
            return Language::Yaml;
        }
        if Self::is_jenkinsfile(filename, content) {
            return Language::Yaml;
        }

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

    fn is_gitlab_ci(filename: &str, content: &str) -> bool {
        // Path-based detection
        if !filename.ends_with(".gitlab-ci.yml") {
            return false;
        }

        // Content-based validation
        let gitlab_patterns = ["stages:", "script:", "image:", "stage:"];
        gitlab_patterns.iter().any(|&p| content.contains(p))
    }

    fn is_circleci(filename: &str, content: &str) -> bool {
        // Path-based detection
        if !filename.contains(".circleci/config.yml") {
            return false;
        }

        // Content-based validation
        let circleci_patterns = ["version:", "jobs:", "workflows:"];
        circleci_patterns.iter().all(|&p| content.contains(p))
    }

    fn is_travis_ci(filename: &str, content: &str) -> bool {
        // Path-based detection
        if !filename.ends_with(".travis.yml") {
            return false;
        }

        // Content-based validation
        let travis_patterns = ["language:", "script:"];
        travis_patterns.iter().any(|&p| content.contains(p))
    }

    fn is_jenkinsfile(filename: &str, content: &str) -> bool {
        // Path-based detection
        if !filename.ends_with("Jenkinsfile") && !filename.ends_with(".groovy") {
            return false;
        }

        // Content-based validation for Jenkinsfile
        let jenkins_patterns = ["pipeline {", "stage(", "steps {", "agent "];
        jenkins_patterns.iter().any(|&p| content.contains(p))
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

    #[test]
    fn test_gitlab_ci_detection() {
        let content = r#"
image: "python:3.9"
stages:
  - test
  - deploy
test-job:
  stage: test
  script:
    - pytest
        "#;
        assert!(FileClassifier::is_gitlab_ci(".gitlab-ci.yml", content));

        // Should not match non-GitLab CI YAML
        assert!(!FileClassifier::is_gitlab_ci("config.yml", content));
    }

    #[test]
    fn test_circleci_detection() {
        let content = r#"
version: 2.1
jobs:
  ci:
    steps:
      - checkout
workflows:
  version: 2
        "#;
        assert!(FileClassifier::is_circleci(".circleci/config.yml", content));

        // Should not match non-CircleCI YAML
        assert!(!FileClassifier::is_circleci("config.yml", content));
    }

    #[test]
    fn test_travis_detection() {
        let content = r#"
language: python
install:
  - pip install .
script:
  - pytest
        "#;
        assert!(FileClassifier::is_travis_ci(".travis.yml", content));

        // Should not match non-Travis YAML
        assert!(!FileClassifier::is_travis_ci("config.yml", content));
    }

    #[test]
    fn test_jenkinsfile_detection() {
        let content = r#"
pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh 'make'
            }
        }
    }
}
        "#;
        assert!(FileClassifier::is_jenkinsfile("Jenkinsfile", content));
        assert!(FileClassifier::is_jenkinsfile("awesome_app.groovy", content));

        // Should not match non-Jenkins files
        assert!(!FileClassifier::is_jenkinsfile("script.sh", content));
    }
}
