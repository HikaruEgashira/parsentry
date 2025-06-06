/*!
 * Input Validators with Bypass Vulnerabilities
 * 
 * Contains validation functions with security flaws
 */

const crypto = require('crypto');

class InputValidators {
    constructor() {
        this.blacklist = [
            'script', 'eval', 'function', 'constructor', 'prototype',
            '__proto__', 'alert', 'confirm', 'prompt', 'document'
        ];
        
        this.sqlKeywords = [
            'select', 'union', 'insert', 'delete', 'update', 'drop',
            'create', 'alter', 'grant', 'revoke', 'truncate'
        ];
    }

    // Vulnerable: XSS validation with bypass opportunities
    validateXSS(input) {
        if (!input || typeof input !== 'string') return input;
        
        let filtered = input;
        
        // Vulnerable: Case-sensitive filtering (easy bypass)
        this.blacklist.forEach(keyword => {
            const regex = new RegExp(keyword, 'gi');
            filtered = filtered.replace(regex, '***');
        });
        
        // Vulnerable: Simple HTML tag filtering
        filtered = filtered.replace(/<script[^>]*>/gi, '');
        filtered = filtered.replace(/<\/script>/gi, '');
        
        // Vulnerable: Event handlers not properly filtered
        filtered = filtered.replace(/on\w+\s*=/gi, 'data-blocked=');
        
        return {
            original: input,
            filtered: filtered,
            safe: filtered === input,
            bypassed: filtered !== input
        };
    }

    // Vulnerable: SQL injection validation with bypass opportunities
    validateSQL(input) {
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
        
        return {
            original: input,
            filtered: filtered,
            safe: filtered === input,
            warnings: filtered !== input ? ['Potential SQL injection detected'] : []
        };
    }

    // Vulnerable: Email validation that can be bypassed
    validateEmail(email) {
        if (!email) return { valid: false, error: 'Email required' };
        
        // Vulnerable: Weak email regex
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        const isValid = emailRegex.test(email);
        
        // Vulnerable: Additional bypass checks
        const bypasses = [
            email.includes('admin@'),
            email.includes('@localhost'),
            email.includes('@127.0.0.1'),
            email.length > 254 // RFC limit bypass
        ];
        
        return {
            email: email,
            valid: isValid,
            bypasses: bypasses,
            warnings: bypasses.some(b => b) ? ['Email contains bypass patterns'] : []
        };
    }

    // Vulnerable: Password validation with weak requirements
    validatePassword(password) {
        if (!password) return { valid: false, error: 'Password required' };
        
        const checks = {
            length: password.length >= 8,
            uppercase: /[A-Z]/.test(password),
            lowercase: /[a-z]/.test(password),
            numbers: /\d/.test(password),
            symbols: /[!@#$%^&*]/.test(password)
        };
        
        // Vulnerable: Weak password requirements
        const isValid = checks.length && checks.uppercase;
        
        // Vulnerable: Common password check bypass
        const commonPasswords = ['password', 'admin', '123456', 'admin123'];
        const isCommon = commonPasswords.includes(password.toLowerCase());
        
        return {
            password: password.replace(/./g, '*'), // Hide in response
            valid: isValid,
            checks: checks,
            common: isCommon,
            warning: isCommon ? 'Common password used' : null
        };
    }

    // Vulnerable: Username validation with bypass opportunities
    validateUsername(username) {
        if (!username) return { valid: false, error: 'Username required' };
        
        // Vulnerable: Allows admin usernames
        const adminPatterns = ['admin', 'administrator', 'root', 'system'];
        const isAdmin = adminPatterns.some(pattern => 
            username.toLowerCase().includes(pattern)
        );
        
        // Vulnerable: Weak character validation
        const hasSpecialChars = /[<>\"'%;()&+]/.test(username);
        
        // Vulnerable: Length validation bypass
        const validLength = username.length >= 3 && username.length <= 50;
        
        return {
            username: username,
            valid: validLength && !hasSpecialChars,
            is_admin_pattern: isAdmin,
            has_special_chars: hasSpecialChars,
            length_ok: validLength,
            warnings: isAdmin ? ['Admin username pattern detected'] : []
        };
    }

    // Vulnerable: File validation with bypass opportunities
    validateFile(filename, content, mimetype) {
        const allowedExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.pdf', '.txt'];
        const allowedMimes = [
            'image/jpeg', 'image/png', 'image/gif', 
            'application/pdf', 'text/plain'
        ];
        
        // Vulnerable: Extension-based validation only
        const ext = filename.toLowerCase().split('.').pop();
        const extValid = allowedExtensions.includes('.' + ext);
        
        // Vulnerable: MIME type can be spoofed
        const mimeValid = !mimetype || allowedMimes.includes(mimetype);
        
        // Vulnerable: Content-based detection bypass
        const suspiciousContent = content && (
            content.includes('<?php') ||
            content.includes('<%') ||
            content.includes('<script')
        );
        
        return {
            filename: filename,
            extension_valid: extValid,
            mime_valid: mimeValid,
            suspicious_content: suspiciousContent,
            valid: extValid && mimeValid && !suspiciousContent,
            bypass_hints: [
                'Try double extensions: file.php.jpg',
                'MIME type spoofing possible',
                'Content detection can be evaded'
            ]
        };
    }

    // Vulnerable: URL validation with bypass opportunities
    validateURL(url) {
        if (!url) return { valid: false, error: 'URL required' };
        
        try {
            const parsed = new URL(url);
            
            // Vulnerable: Doesn't check for private IPs
            const allowedProtocols = ['http:', 'https:'];
            const protocolValid = allowedProtocols.includes(parsed.protocol);
            
            // Vulnerable: Hostname validation bypass
            const dangerousPatterns = [
                /localhost/i,
                /127\.0\.0\.1/,
                /192\.168\./,
                /10\./,
                /172\.(1[6-9]|2[0-9]|3[0-1])\./
            ];
            
            const isDangerous = dangerousPatterns.some(pattern => 
                pattern.test(url)
            );
            
            return {
                url: url,
                protocol_valid: protocolValid,
                hostname: parsed.hostname,
                dangerous: isDangerous,
                valid: protocolValid && !isDangerous,
                bypass_hints: [
                    'IP encoding: 0x7f000001',
                    'Decimal encoding: 2130706433',
                    'URL shorteners not blocked'
                ]
            };
        } catch (error) {
            return {
                url: url,
                valid: false,
                error: error.message
            };
        }
    }

    // Vulnerable: Session token validation
    validateSessionToken(token) {
        if (!token) return { valid: false, error: 'Token required' };
        
        // Vulnerable: Predictable token patterns
        const patterns = [
            /^[a-f0-9]{32}$/, // MD5
            /^[a-f0-9]{40}$/, // SHA1
            /^[a-f0-9]{64}$/, // SHA256
            /^admin_/, // Admin prefix
            /^bypass_/ // Bypass prefix
        ];
        
        const matchedPattern = patterns.findIndex(pattern => pattern.test(token));
        
        return {
            token: token.substring(0, 10) + '...',
            valid: matchedPattern >= 0,
            pattern_matched: matchedPattern,
            predictable: matchedPattern >= 0,
            warning: 'Token pattern is predictable'
        };
    }
}

module.exports = new InputValidators();