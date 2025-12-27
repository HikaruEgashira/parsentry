# parsentry-core

Core types and traits for Parsentry security scanner.

## Overview

This crate provides fundamental types used across all Parsentry components:

- **`VulnType`**: Vulnerability types (SQLi, XSS, RCE, LFI, SSRF, AFO, IDOR)
- **`Language`**: Supported programming languages for analysis
- **`Response`**: Analysis response structure
- **`ParAnalysis`**: PAR (Principal-Action-Resource) analysis types

## Usage

```rust
use parsentry_core::{Response, VulnType, Language, ParAnalysis};

// Parse vulnerability type from string
let vuln: VulnType = "SQLI".parse().unwrap();
assert_eq!(vuln, VulnType::SQLI);

// Get CWE IDs for a vulnerability
let cwe_ids = VulnType::RCE.cwe_ids();
assert!(cwe_ids.contains(&"CWE-78".to_string()));

// Detect language from file extension
let lang = Language::from_extension("py");
assert_eq!(lang, Language::Python);

// Check if language is IaC
assert!(Language::Terraform.is_iac());
assert!(!Language::Python.is_iac());

// Create and use Response
let mut response = Response::default();
response.confidence_score = 85;
response.vulnerability_types.push(VulnType::SQLI);
assert_eq!(response.severity_level(), "high");
```

## Types

### `VulnType`

Represents security vulnerability types with mappings to:
- CWE (Common Weakness Enumeration)
- MITRE ATT&CK techniques
- OWASP Top 10 categories

### `Language`

Supported programming languages including:
- General: Python, JavaScript, TypeScript, Rust, Java, Go, Ruby, C, C++, PHP
- IaC: Terraform, CloudFormation, Kubernetes, YAML

### `ParAnalysis`

PAR (Principal-Action-Resource) security model:
- **Principal**: Data sources with trust levels
- **Action**: Security controls with quality assessment
- **Resource**: Protected resources with sensitivity levels

### `Response`

Complete analysis response including:
- Vulnerability findings
- Confidence scoring
- PAR analysis results
- Remediation guidance

## License

MIT
