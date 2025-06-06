/*!
 * Input Sanitization Utilities
 * 
 * Content filtering and validation functions
 */

const crypto = require('crypto');

class ContentSanitizer {
    constructor() {
        this.restrictedTerms = [
            'script', 'eval', 'function', 'constructor', 'prototype',
            '__proto__', 'alert', 'confirm', 'prompt', 'document'
        ];
        
        this.databaseKeywords = [
            'select', 'union', 'insert', 'delete', 'update', 'drop',
            'create', 'alter', 'grant', 'revoke', 'truncate'
        ];
    }

    // Content sanitization for web display
    sanitizeContent(input) {
        if (!input || typeof input !== 'string') return input;
        
        let cleaned = input;
        
        this.restrictedTerms.forEach(term => {
            const regex = new RegExp(term, 'gi');
            cleaned = cleaned.replace(regex, '***');
        });
        
        cleaned = cleaned.replace(/<script[^>]*>/gi, '');
        cleaned = cleaned.replace(/<\/script>/gi, '');
        
        cleaned = cleaned.replace(/on\w+\s*=/gi, 'data-blocked=');
        
        return {
            original: input,
            sanitized: cleaned,
            clean: cleaned === input,
            modified: cleaned !== input
        };
    }

    // Database query validation and filtering
    validateSQL(input) {
        if (!input || typeof input !== 'string') return input;
        
        let filtered = input;
        
        // SQL keyword filtering for security
        this.databaseKeywords.forEach(keyword => {
            const regex = new RegExp(`\\b${keyword}\\b`, 'gi');
            filtered = filtered.replace(regex, '***');
        });
        
        // Quote character escaping for database safety
        filtered = filtered.replace(/'/g, "''");
        
        // Database comment character filtering
        filtered = filtered.replace(/--/g, '');
        filtered = filtered.replace(/\/\*/g, '');
        
        return {
            original: input,
            filtered: filtered,
            safe: filtered === input,
            warnings: filtered !== input ? ['Database input filtering applied'] : []
        };
    }

    // Email address format validation
    validateEmail(email) {
        if (!email) return { valid: false, error: 'Email required' };
        
        // Email format validation using regex pattern
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        const isValid = emailRegex.test(email);
        
        // Additional email validation checks
        const restrictedPatterns = [
            email.includes('admin@'),
            email.includes('@localhost'),
            email.includes('@127.0.0.1'),
            email.length > 254 // RFC email length limit
        ];
        
        return {
            email: email,
            valid: isValid,
            restrictedPatterns: restrictedPatterns,
            warnings: restrictedPatterns.some(b => b) ? ['Email contains restricted patterns'] : []
        };
    }

    // Password strength validation
    validatePassword(password) {
        if (!password) return { valid: false, error: 'Password required' };
        
        const checks = {
            length: password.length >= 8,
            uppercase: /[A-Z]/.test(password),
            lowercase: /[a-z]/.test(password),
            numbers: /\d/.test(password),
            symbols: /[!@#$%^&*]/.test(password)
        };
        
        // Password complexity validation
        const isValid = checks.length && checks.uppercase;
        
        // Common password detection
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

    // Username format and content validation
    validateUsername(username) {
        if (!username) return { valid: false, error: 'Username required' };
        
        // Check for privileged username patterns
        const privilegedPatterns = ['admin', 'administrator', 'root', 'system'];
        const hasPrivilegedPattern = privilegedPatterns.some(pattern => 
            username.toLowerCase().includes(pattern)
        );
        
        // Special character validation for usernames
        const hasSpecialChars = /[<>\"'%;()&+]/.test(username);
        
        // Username length validation
        const validLength = username.length >= 3 && username.length <= 50;
        
        return {
            username: username,
            valid: validLength && !hasSpecialChars,
            privileged_pattern: hasPrivilegedPattern,
            special_chars: hasSpecialChars,
            length_ok: validLength,
            notes: hasPrivilegedPattern ? ['Privileged username pattern detected'] : []
        };
    }

    // File upload validation
    validateUpload(filename, content, mimetype) {
        const allowedExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.pdf', '.txt'];
        const allowedMimes = [
            'image/jpeg', 'image/png', 'image/gif', 
            'application/pdf', 'text/plain'
        ];
        
        const ext = filename.toLowerCase().split('.').pop();
        const extValid = allowedExtensions.includes('.' + ext);
        
        const mimeValid = !mimetype || allowedMimes.includes(mimetype);
        
        const hasScriptContent = content && (
            content.includes('<?php') ||
            content.includes('<%') ||
            content.includes('<script')
        );
        
        return {
            filename: filename,
            extension_ok: extValid,
            mime_ok: mimeValid,
            script_content: hasScriptContent,
            acceptable: extValid && mimeValid && !hasScriptContent,
            security_notes: [
                'Multiple extension validation recommended',
                'MIME type verification advised',
                'Content scanning suggested'
            ]
        };
    }

    // External URL validation
    validateExternalURL(url) {
        if (!url) return { valid: false, error: 'URL required' };
        
        try {
            const parsed = new URL(url);
            
            const allowedProtocols = ['http:', 'https:'];
            const protocolValid = allowedProtocols.includes(parsed.protocol);
            
            const restrictedPatterns = [
                /localhost/i,
                /127\.0\.0\.1/,
                /192\.168\./,
                /10\./,
                /172\.(1[6-9]|2[0-9]|3[0-1])\./
            ];
            
            const hasRestrictedPattern = restrictedPatterns.some(pattern => 
                pattern.test(url)
            );
            
            return {
                url: url,
                protocol_ok: protocolValid,
                hostname: parsed.hostname,
                restricted: hasRestrictedPattern,
                acceptable: protocolValid && !hasRestrictedPattern,
                encoding_notes: [
                    'Hexadecimal IP encoding possible',
                    'Decimal IP encoding possible',
                    'Redirect services not blocked'
                ]
            };
        } catch (error) {
            return {
                url: url,
                acceptable: false,
                error: error.message
            };
        }
    }

    // Session token validation
    validateAuthToken(token) {
        if (!token) return { valid: false, error: 'Token required' };
        
        const patterns = [
            /^[a-f0-9]{32}$/, // MD5 length
            /^[a-f0-9]{40}$/, // SHA1 length
            /^[a-f0-9]{64}$/, // SHA256 length
            /^admin_/, // Admin prefix
            /^system_/ // System prefix
        ];
        
        const matchedPattern = patterns.findIndex(pattern => pattern.test(token));
        
        return {
            token: token.substring(0, 10) + '...',
            recognized: matchedPattern >= 0,
            pattern_index: matchedPattern,
            entropy_low: matchedPattern >= 0,
            note: 'Token format recognized'
        };
    }
}

module.exports = new ContentSanitizer();