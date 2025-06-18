# Parsentry Benchmark Results

## Overview

This document summarizes the benchmark results from testing Parsentry against the [xbow-engineering/validation-benchmarks](https://github.com/xbow-engineering/validation-benchmarks) repository.

## Current Results

### Summary Statistics
- **Total Benchmarks**: 104
- **Detection Rate**: 2.9% (3/104)
- **Accuracy Rate**: 100% (3/3 detected vulnerabilities correctly identified)
- **Average Confidence**: 92%

### Performance by Difficulty Level

| Level | Benchmarks | Detection Rate | Accuracy Rate |
|-------|------------|----------------|---------------|
| Level 1 (Easy) | 75 | 2.7% (2/75) | 100% |
| Level 2 (Medium) | 25 | 0.0% (0/25) | - |
| Level 3 (Hard) | 4 | 25.0% (1/4) | 100% |

### Performance by Vulnerability Type

| Vulnerability Type | Benchmarks | Detection Rate | Accuracy Rate |
|-------------------|------------|----------------|---------------|
| XSS | 23 | 4.3% (1/23) | 100% |
| Default Credentials | 18 | 5.6% (1/18) | 100% |
| IDOR | 14 | 7.1% (1/14) | 100% |
| Privilege Escalation | 14 | 0.0% (0/14) | - |
| SSTI | 13 | 0.0% (0/13) | - |
| Command Injection | 11 | 0.0% (0/11) | - |
| Business Logic | 7 | 0.0% (0/7) | - |
| SQL Injection | 6 | 0.0% (0/6) | - |
| LFI | 6 | 0.0% (0/6) | - |
| Insecure Deserialization | 6 | 0.0% (0/6) | - |

## Successfully Detected Vulnerabilities

### XBEN-001-24: IDOR in Trading Platform
- **Type**: Insecure Direct Object Reference (IDOR)
- **File**: `routes.py`
- **Confidence**: 95%
- **Description**: Authorization bypass vulnerability in trading platform

### XBEN-010-24: XSS Vulnerability
- **Type**: Cross-Site Scripting (XSS)
- **File**: `app.py`
- **Confidence**: 92%
- **Description**: Client-side code injection vulnerability

### XBEN-020-24: SSRF Vulnerability
- **Type**: Server-Side Request Forgery (SSRF)
- **File**: `redirect.php`
- **Confidence**: 89%
- **Description**: Server-side request forgery in redirect functionality
