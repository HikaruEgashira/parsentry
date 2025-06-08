# PAR Framework (Principal-Action-Resource)

## Overview

The PAR (Principal-Action-Resource) framework is a security analysis model that categorizes vulnerabilities by examining three key components of any security-relevant operation:

- **Principal**: Who or what is performing the action (user, service, process)
- **Action**: What operation is being performed (read, write, execute, authenticate)
- **Resource**: What is being accessed or modified (file, database, network endpoint, memory)

## Origins and Inspiration

The PAR framework draws inspiration from established security analysis concepts while providing a more structured approach:

### Taint Tracking Model
Traditional static analysis tools use taint tracking to follow data flow:
- **Sources**: Points where untrusted data enters the system
- **Sinks**: Points where data could cause security issues
- **Validation**: Sanitization and checks between sources and sinks

The PAR framework extends this model by:
- **Principal** encompasses both traditional sources and the entities controlling them
- **Action** includes validation/sanitization operations as well as the actual operations performed
- **Resource** generalizes sinks to include all types of targets and system resources

### Cedar Language Influence
Amazon's Cedar authorization language provides a policy framework based on:
- **Principal**: The entity making the request
- **Action**: The operation being requested
- **Resource**: The target of the operation

Parsentry's PAR framework adapts these concepts for vulnerability analysis:
- Cedar focuses on authorization decisions for allowed operations
- PAR focuses on security analysis to identify potential vulnerabilities
- Both provide systematic ways to reason about security relationships

## Why PAR Framework?

Building on these foundational concepts, the PAR framework provides a more systematic approach than traditional vulnerability scanners by:

1. **Comprehensive Coverage**: Ensures all aspects of a security operation are analyzed
2. **Context Awareness**: Considers the relationship between actors, actions, and targets
3. **Scalable Classification**: Works across different languages and technology stacks
4. **Risk Assessment**: Enables better prioritization based on principal privileges and resource sensitivity

## PAR Components in Detail

### Principal
The entity initiating or controlling the action:
- **User accounts** (authenticated, anonymous, privileged)
- **Service accounts** (database users, API keys, service principals)
- **Processes** (system processes, application threads)
- **External systems** (third-party APIs, microservices)

### Action
The operation being performed:
- **Data operations** (read, write, update, delete)
- **Authentication** (login, logout, token generation)
- **Authorization** (permission checks, role assignments)
- **System operations** (file I/O, network calls, process execution)
- **Cryptographic operations** (encryption, signing, hashing)

### Resource
The target of the action:
- **Data stores** (databases, files, memory)
- **Network resources** (endpoints, protocols, certificates)
- **System resources** (processes, services, hardware)
- **Business logic** (workflows, transactions, state)

## PAR-Based Vulnerability Analysis

Parsentry uses the PAR framework to systematically identify security issues:

### 1. Principal Analysis
- Identifies all actors in the code
- Analyzes privilege levels and trust boundaries
- Detects privilege escalation opportunities
- Maps authentication and authorization flows

### 2. Action Analysis
- Catalogs all security-relevant operations
- Identifies unsafe or deprecated functions
- Analyzes input validation and sanitization
- Detects business logic flaws

### 3. Resource Analysis
- Maps all accessed resources
- Identifies sensitive data flows
- Analyzes access control mechanisms
- Detects resource exhaustion vulnerabilities

### 4. PAR Relationship Analysis
- Examines interactions between components
- Identifies trust boundary violations
- Detects authorization bypasses
- Analyzes data flow security

## Example: Vulnerability Analysis

Consider this vulnerable code:
```javascript
function validateEmail(email) {
    // Vulnerable regex - missing anchors
    const emailRegex = /[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/;
    return emailRegex.test(email);
}

function processUser(userData) {
    if (validateEmail(userData.email)) {
        // Bypass: "malicious<script>alert('xss')</script>user@example.com"
        document.getElementById('welcome').innerHTML = `Welcome ${userData.email}`;
    }
}
```

## Analysis Framework Comparison

### Traditional Taint Tracking:
- **Source**: `userData.email` (untrusted input)
- **Sink**: `innerHTML` (DOM injection point)
- **Validation**: Present but insufficient - regex bypass possible

### Cedar-style Policy Analysis:
- **Principal**: Web application service
- **Action**: DOM manipulation operation
- **Resource**: User interface
- **Policy**: "Application can update UI" (authorized but unsafe implementation)

### PAR Framework Analysis:
- **Principal**: User input via web interface (untrusted source)
- **Action**: Email validation with flawed regex (vulnerable validation implementation)
  - Regex lacks `^` and `$` anchors
  - Allows valid email within malicious string
  - Creates false trust boundary
- **Resource**: DOM via `innerHTML` (XSS execution context)

**PAR Advantage**: Captures both the trust relationship (user input vs. application) and analyzes the quality of security actions, providing more comprehensive analysis than traditional approaches alone.

## PAR Patterns Across Contexts

### JavaScript/Python/Ruby Libraries and Applications
**Library Level:**
- **P**: Function arguments, configuration objects, imported modules
- **A**: Data processing, validation functions, crypto operations
- **R**: Return values, file system, network endpoints

**Web Application Level:**
- **P**: User sessions, API clients, service accounts
- **A**: HTTP requests, database queries, authentication
- **R**: User data, configuration files, external APIs

**PAR Advantage**: Unlike traditional scanners that focus on user input flows, PAR can analyze security issues in library code, utility functions, and internal APIs regardless of the application context.

### Infrastructure as Code (Terraform)
- **P**: Cloud services, deployment pipelines, operators
- **A**: Resource provisioning, permission grants, network configuration
- **R**: Cloud resources, security groups, IAM policies

### System Programming (C/C++/Rust)
- **P**: Processes, threads, system users
- **A**: Memory allocation, file I/O, system calls
- **R**: Memory regions, file systems, hardware interfaces

### Framework Agnostic Analysis
Traditional vulnerability scanners typically require:
- Web application context (HTTP requests, form inputs)
- User input as vulnerability sources
- Application-specific sinks (responses, database writes)

PAR framework enables analysis of:
- **Standalone libraries** without web context
- **Internal APIs and utilities** 
- **Framework-independent code**
- **Microservices and serverless functions**
- **CLI tools and system utilities**

This versatility allows PAR-based analysis to identify vulnerabilities in reusable components that might be missed by application-focused scanners.

## Integration with LLM Analysis

The PAR framework guides LLM prompts to ensure comprehensive analysis:

1. **Structured Prompts**: Each analysis request includes PAR context
2. **Systematic Coverage**: Ensures all three dimensions are evaluated
3. **Consistent Classification**: Provides uniform vulnerability categorization
4. **Risk Scoring**: Enables PAR-based severity assessment
