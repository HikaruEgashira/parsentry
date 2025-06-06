# Modern Attack Vectors - JavaScript Vulnerable App

This document describes the new modern attack vectors implemented in the JavaScript vulnerable application. These patterns represent contemporary security threats commonly found in modern web applications.

## New Attack Vectors

### 1. Prototype Pollution (`/modern/prototype/merge`)

**Description**: Attacks targeting JavaScript's prototype chain to inject properties into `Object.prototype`.

**Vulnerable Code Pattern**:
```javascript
function mergeDeep(target, source) {
    for (const key in source) {
        if (typeof source[key] === 'object' && source[key] !== null) {
            target[key] = target[key] || {};
            mergeDeep(target[key], source[key]);
        } else {
            target[key] = source[key];
        }
    }
}
```

**Attack Payload**:
```json
{
  "config": {
    "__proto__": {
      "isAdmin": true,
      "polluted": "success"
    }
  }
}
```

**Impact**: Application-wide property injection, privilege escalation.

### 2. Server-Side Template Injection (`/modern/template/render`)

**Description**: Injection attacks against template engines (Handlebars, Pug, EJS).

**Vulnerable Patterns**:
- Handlebars: `{{constructor.constructor("return process")()}}`
- Pug: `#{constructor.constructor("return process")()}`
- EJS: `<%- constructor.constructor("return process")() %>`

**Impact**: Remote code execution, server compromise.

### 3. Advanced XXE (`/modern/xml/parse`)

**Description**: XML External Entity attacks with multiple parser backends.

**Attack Payload**:
```xml
<?xml version="1.0"?>
<!DOCTYPE data [
  <!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<data>&xxe;</data>
```

**Impact**: File disclosure, SSRF, denial of service.

### 4. YAML Deserialization (`/modern/yaml/parse`)

**Description**: Unsafe YAML parsing leading to code execution.

**Attack Payload**:
```yaml
!!js/function "function() { return require('child_process').execSync('id'); }"
```

**Impact**: Remote code execution.

### 5. SSRF with Protocol Confusion (`/modern/ssrf/fetch`)

**Description**: Server-Side Request Forgery with protocol and hostname bypasses.

**Bypass Techniques**:
- `127.0.0.1.nip.io` (DNS rebinding)
- `0x7f000001` (hexadecimal representation)
- `[::1]` (IPv6 localhost)
- `file://`, `dict://`, `gopher://` protocols

**Impact**: Internal network access, file disclosure.

### 6. JWT Algorithm Confusion (`/modern/jwt/verify`)

**Description**: Algorithm confusion attacks against JWT verification.

**Attack Vector**: Use public key as HMAC secret when algorithm is switched from RS256 to HS256.

**Impact**: Authentication bypass.

### 7. WebAssembly Sandbox Escape (`/modern/wasm/execute`)

**Description**: Simulation of WebAssembly sandbox escape through dangerous imports.

**Dangerous Imports**:
- `system()` - Execute system commands
- `read_file()` - Read arbitrary files
- `get_process()` - Access process information

**Impact**: Full system compromise.

### 8. GraphQL-like Query Injection (`/modern/graphql`)

**Description**: Injection attacks against GraphQL-style query parsers.

**Attack Payload**:
```
user(id: process.env) { name, email }
admin(secret: constructor.constructor("return process")()) { users }
```

**Impact**: Information disclosure, code execution.

### 9. NoSQL Injection (`/modern/nosql/query`)

**Description**: Injection attacks against NoSQL databases (MongoDB-style).

**Attack Payloads**:
- `{"$where": "this.password && sleep(5000)"}`
- `{"password": {"$regex": ".*", "$options": "i"}}`

**Impact**: Data exfiltration, authentication bypass.

## Security Testing

### Automated Detection

These vulnerability patterns are designed to be detected by the vulnhuntrs security scanner. Each endpoint includes:

1. **Multiple input vectors** - Different parameters and payload formats
2. **Realistic business logic** - Enterprise-like functionality context
3. **Bypass techniques** - Methods to circumvent basic protections
4. **Escalation paths** - Ways to chain vulnerabilities

### Testing Recommendations

1. **Prototype Pollution**: Test object merge operations
2. **Template Injection**: Scan template rendering endpoints
3. **XXE**: Check XML parsing functionality
4. **YAML Injection**: Examine configuration parsing
5. **SSRF**: Test URL fetching features
6. **JWT Issues**: Analyze token verification logic
7. **WASM Security**: Review WebAssembly integrations
8. **Query Injection**: Test dynamic query systems
9. **NoSQL Injection**: Check database query construction

## Real-World Context

These attack patterns represent actual vulnerabilities found in production applications:

- **Prototype Pollution**: Affecting popular npm packages
- **SSTI**: Common in template-heavy applications
- **XXE**: Found in XML processing systems
- **YAML Injection**: Configuration management vulnerabilities
- **SSRF**: Microservice communication flaws
- **JWT Confusion**: Authentication system weaknesses
- **WASM Issues**: Emerging attack surface
- **GraphQL Injection**: Modern API vulnerabilities
- **NoSQL Injection**: Database query flaws

## Mitigation Strategies

1. **Input Validation**: Strict type checking and sanitization
2. **Safe Parsing**: Use safe parsers with restricted features
3. **Sandboxing**: Proper isolation for code execution
4. **Protocol Restrictions**: Whitelist allowed protocols/hosts
5. **Algorithm Enforcement**: Strict JWT algorithm validation
6. **Query Parameterization**: Avoid dynamic query construction
7. **Security Headers**: Implement comprehensive headers
8. **Regular Updates**: Keep dependencies current