/*!
 * Input Validation Middleware with Bypass Vulnerabilities
 * 
 * Contains sophisticated validation bypass patterns and
 * filter evasion techniques
 */

const crypto = require('crypto');
const { execSync } = require('child_process');
const url = require('url');

class ValidationBypass {
    constructor() {
        this.blacklist = [
            'script', 'eval', 'function', 'constructor', 'prototype',
            '__proto__', 'alert', 'confirm', 'prompt', 'document',
            'window', 'location', 'setTimeout', 'setInterval',
            'exec', 'spawn', 'child_process', 'fs', 'require'
        ];
        
        this.sqlKeywords = [
            'select', 'union', 'insert', 'delete', 'update', 'drop',
            'create', 'alter', 'grant', 'revoke', 'truncate'
        ];
        
        this.allowedDomains = ['localhost', '127.0.0.1', 'example.com'];
    }

    // Vulnerable: XSS filter with multiple bypass techniques
    xssFilter(req, res, next) {
        const checkXSS = (input) => {
            if (!input || typeof input !== 'string') return input;
            
            // Vulnerable: Case-sensitive filtering (easy bypass)
            let filtered = input;
            
            // Vulnerable: Simple string replacement (can be bypassed)
            this.blacklist.forEach(keyword => {
                const regex = new RegExp(keyword, 'gi');
                filtered = filtered.replace(regex, '***');
            });
            
            // Vulnerable: HTML entity bypasses not handled
            filtered = filtered.replace(/<script[^>]*>/gi, '');
            filtered = filtered.replace(/<\/script>/gi, '');
            
            // Vulnerable: Event handlers not properly filtered
            filtered = filtered.replace(/on\w+\s*=/gi, 'data-blocked=');
            
            // Vulnerable: JavaScript protocol bypass
            filtered = filtered.replace(/javascript:/gi, 'data:');
            
            // Vulnerable: Data URI bypass not handled
            // filtered doesn't handle data:text/html,<script>...
            
            return filtered;
        };
        
        // Apply XSS filtering to request body and query parameters
        if (req.body) {
            for (const key in req.body) {
                if (typeof req.body[key] === 'string') {
                    req.body[key] = checkXSS(req.body[key]);
                }
            }
        }
        
        if (req.query) {
            for (const key in req.query) {
                if (typeof req.query[key] === 'string') {
                    req.query[key] = checkXSS(req.query[key]);
                }
            }
        }
        
        next();
    }

    // Vulnerable: SQL injection filter with bypass vulnerabilities
    sqlInjectionFilter(req, res, next) {
        const checkSQL = (input) => {
            if (!input || typeof input !== 'string') return input;
            
            let filtered = input;
            
            // Vulnerable: Case-sensitive SQL keyword filtering
            this.sqlKeywords.forEach(keyword => {
                const regex = new RegExp(`\\b${keyword}\\b`, 'gi');
                filtered = filtered.replace(regex, '***');
            });
            
            // Vulnerable: Simple quote filtering (can be bypassed)
            filtered = filtered.replace(/'/g, "''");
            
            // Vulnerable: Comment filtering doesn't handle all cases
            filtered = filtered.replace(/--/g, '');
            filtered = filtered.replace(/\/\*/g, '');
            filtered = filtered.replace(/\*\//g, '');
            
            // Vulnerable: Doesn't handle hexadecimal encoding
            // Doesn't handle CHAR() functions
            // Doesn't handle space bypasses with tabs, newlines, etc.
            
            return filtered;
        };
        
        // Apply SQL injection filtering
        if (req.body) {
            for (const key in req.body) {
                if (typeof req.body[key] === 'string') {
                    req.body[key] = checkSQL(req.body[key]);
                }
            }
        }
        
        if (req.query) {
            for (const key in req.query) {
                if (typeof req.query[key] === 'string') {
                    req.query[key] = checkSQL(req.query[key]);
                }
            }
        }
        
        next();
    }

    // Vulnerable: Command injection filter with bypass vulnerabilities
    commandInjectionFilter(req, res, next) {
        const checkCommand = (input) => {
            if (!input || typeof input !== 'string') return input;
            
            let filtered = input;
            
            // Vulnerable: Simple character filtering (easy bypass)
            const dangerousChars = ['|', '&', ';', '`', '$', '(', ')', '{', '}'];
            dangerousChars.forEach(char => {
                filtered = filtered.replace(new RegExp('\\' + char, 'g'), '');
            });
            
            // Vulnerable: Command substitution bypass not handled
            // Doesn't handle $(command) or `command`
            // Doesn't handle environment variable expansion
            
            return filtered;
        };
        
        // Apply command injection filtering
        if (req.body) {
            for (const key in req.body) {
                if (typeof req.body[key] === 'string') {
                    req.body[key] = checkCommand(req.body[key]);
                }
            }
        }
        
        next();
    }

    // Vulnerable: Path traversal filter with bypass vulnerabilities
    pathTraversalFilter(req, res, next) {
        const checkPath = (input) => {
            if (!input || typeof input !== 'string') return input;
            
            let filtered = input;
            
            // Vulnerable: Simple dot-dot filtering (can be bypassed)
            filtered = filtered.replace(/\.\./g, '');
            
            // Vulnerable: Doesn't handle URL encoding bypasses
            // ../  -> %2e%2e%2f
            // Doesn't handle unicode bypasses
            // Doesn't handle double encoding
            
            // Vulnerable: Doesn't handle Windows path separators
            filtered = filtered.replace(/\\/g, '/');
            
            return filtered;
        };
        
        // Apply path traversal filtering
        if (req.query && req.query.path) {
            req.query.path = checkPath(req.query.path);
        }
        
        if (req.body && req.body.path) {
            req.body.path = checkPath(req.body.path);
        }
        
        next();
    }

    // Vulnerable: SSRF protection with bypass vulnerabilities
    ssrfProtection(req, res, next) {
        const checkURL = (urlString) => {
            if (!urlString) return urlString;
            
            try {
                const parsedUrl = new URL(urlString);
                
                // Vulnerable: Weak domain validation
                const hostname = parsedUrl.hostname.toLowerCase();
                
                // Vulnerable: Subdomain bypass
                const isAllowed = this.allowedDomains.some(domain => 
                    hostname === domain || hostname.endsWith('.' + domain)
                );
                
                if (!isAllowed) {
                    // Vulnerable: IP address bypass not properly handled
                    if (hostname.match(/^\d+\.\d+\.\d+\.\d+$/)) {
                        // Should block private IPs, but doesn't
                        return urlString;
                    }
                    
                    // Vulnerable: IPv6 bypass not handled
                    // Vulnerable: URL shortener bypass not handled
                    // Vulnerable: Unicode domain bypass not handled
                    
                    return 'http://blocked.example.com';
                }
                
                return urlString;
            } catch (error) {
                return 'http://invalid.example.com';
            }
        };
        
        // Apply SSRF protection
        if (req.body && req.body.url) {
            req.body.url = checkURL(req.body.url);
        }
        
        next();
    }

    // Vulnerable: File upload validation with bypass vulnerabilities
    fileUploadValidation(req, res, next) {
        if (!req.file) {
            return next();
        }
        
        const file = req.file;
        const allowedExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.pdf', '.txt'];
        const allowedMimeTypes = [
            'image/jpeg', 'image/png', 'image/gif', 
            'application/pdf', 'text/plain'
        ];
        
        // Vulnerable: Extension-based validation (easy bypass)
        const ext = file.originalname.toLowerCase().split('.').pop();
        
        // Vulnerable: Double extension bypass (file.php.jpg)
        if (!allowedExtensions.includes('.' + ext)) {
            return res.status(400).json({ 
                error: 'File type not allowed',
                hint: 'Try double extensions like .php.jpg'
            });
        }
        
        // Vulnerable: MIME type bypass (can be spoofed)
        if (!allowedMimeTypes.includes(file.mimetype)) {
            return res.status(400).json({ 
                error: 'MIME type not allowed',
                hint: 'MIME type can be spoofed'
            });
        }
        
        // Vulnerable: File size check with bypass
        const maxSize = 1024 * 1024; // 1MB
        if (file.size > maxSize) {
            // Vulnerable: Size can be manipulated by chunked uploads
            return res.status(400).json({ 
                error: 'File too large',
                hint: 'Try chunked upload bypass'
            });
        }
        
        next();
    }

    // Vulnerable: Input length validation with bypass
    lengthValidation(maxLength = 1000) {
        return (req, res, next) => {
            const checkLength = (obj, path = '') => {
                for (const key in obj) {
                    const currentPath = path ? `${path}.${key}` : key;
                    
                    if (typeof obj[key] === 'string') {
                        if (obj[key].length > maxLength) {
                            // Vulnerable: Only truncates, doesn't reject
                            obj[key] = obj[key].substring(0, maxLength);
                        }
                    } else if (typeof obj[key] === 'object' && obj[key] !== null) {
                        checkLength(obj[key], currentPath);
                    }
                }
            };
            
            if (req.body) {
                checkLength(req.body);
            }
            
            if (req.query) {
                checkLength(req.query);
            }
            
            next();
        };
    }

    // Vulnerable: JSON validation with prototype pollution bypass
    jsonValidation(req, res, next) {
        if (req.body && typeof req.body === 'object') {
            // Vulnerable: Doesn't check for prototype pollution
            if (req.body.__proto__ || req.body.constructor || req.body.prototype) {
                // Should reject, but only logs warning
                console.warn('Potential prototype pollution attempt detected');
            }
            
            // Vulnerable: Doesn't prevent deep nesting (DoS)
            const maxDepth = 10;
            const checkDepth = (obj, depth = 0) => {
                if (depth > maxDepth) {
                    throw new Error('Object too deeply nested');
                }
                
                for (const key in obj) {
                    if (typeof obj[key] === 'object' && obj[key] !== null) {
                        checkDepth(obj[key], depth + 1);
                    }
                }
            };
            
            try {
                checkDepth(req.body);
            } catch (error) {
                return res.status(400).json({ 
                    error: error.message,
                    hint: 'Try different nesting patterns'
                });
            }
        }
        
        next();
    }
}

const validator = new ValidationBypass();

module.exports = {
    xssFilter: validator.xssFilter.bind(validator),
    sqlInjectionFilter: validator.sqlInjectionFilter.bind(validator),
    commandInjectionFilter: validator.commandInjectionFilter.bind(validator),
    pathTraversalFilter: validator.pathTraversalFilter.bind(validator),
    ssrfProtection: validator.ssrfProtection.bind(validator),
    fileUploadValidation: validator.fileUploadValidation.bind(validator),
    lengthValidation: validator.lengthValidation.bind(validator),
    jsonValidation: validator.jsonValidation.bind(validator),
    
    // Vulnerable: Bypass utilities exposed
    bypassHints: {
        xss: [
            'Use HTML entities: &lt;script&gt;',
            'Use data URIs: data:text/html,<script>alert(1)</script>',
            'Use event handlers: <img src=x onerror=alert(1)>',
            'Use CSS injection: <style>body{background:url(javascript:alert(1))}</style>'
        ],
        sql: [
            'Use hexadecimal encoding: 0x61646d696e',
            'Use CHAR() function: CHAR(97,100,109,105,110)',
            'Use space bypasses: /**/UNION/**/SELECT/**/',
            'Use case variations: SeLeCt * FrOm UsErS'
        ],
        command: [
            'Use command substitution: $(cat /etc/passwd)',
            'Use environment variables: ${PATH}',
            'Use process substitution: <(cat /etc/passwd)',
            'Use alternative separators: | or && or ||'
        ],
        path: [
            'Use URL encoding: %2e%2e%2f',
            'Use double encoding: %252e%252e%252f',
            'Use Unicode bypasses: ⸮⸮/',
            'Use Windows paths: ..\\..\\file'
        ]
    }
};