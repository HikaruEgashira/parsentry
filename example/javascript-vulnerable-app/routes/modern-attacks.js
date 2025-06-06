/*!
 * Modern Attack Vectors Routes
 * 
 * Contains contemporary vulnerability patterns including:
 * - Prototype pollution
 * - Server-side template injection
 * - WebAssembly sandbox escapes
 * - Advanced XXE and SSRF patterns
 */

const express = require('express');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const crypto = require('crypto');
const axios = require('axios');
const vm = require('vm');
const { DOMParser } = require('xmldom');
const yaml = require('js-yaml');

const router = express.Router();

// Prototype pollution vulnerability patterns
router.post('/prototype/merge', (req, res) => {
    const { config, userData } = req.body;
    
    // Dangerous: Deep merge without prototype protection
    function mergeDeep(target, source) {
        for (const key in source) {
            if (typeof source[key] === 'object' && source[key] !== null) {
                if (typeof target[key] !== 'object') {
                    target[key] = {};
                }
                mergeDeep(target[key], source[key]);
            } else {
                target[key] = source[key];
            }
        }
        return target;
    }
    
    const defaultConfig = {
        theme: 'light',
        language: 'en',
        permissions: {
            read: true,
            write: false,
            admin: false
        }
    };
    
    // Vulnerable merge operation
    const finalConfig = mergeDeep(defaultConfig, config);
    mergeDeep(finalConfig, userData);
    
    // Check if prototype pollution occurred
    const pollutionCheck = {};
    const polluted = pollutionCheck.isAdmin === true;
    
    res.json({
        config: finalConfig,
        polluted,
        hint: 'Try: {"__proto__": {"isAdmin": true}}'
    });
});

// Template injection with multiple engines
router.post('/template/render', (req, res) => {
    const { template, engine, data } = req.body;
    
    try {
        let result;
        
        switch (engine) {
            case 'handlebars':
                // Simulate Handlebars rendering vulnerability
                if (template.includes('{{')) {
                    // Dangerous: Direct template compilation
                    const compiledTemplate = template.replace(/\{\{(.+?)\}\}/g, (match, expr) => {
                        try {
                            // Vulnerable: eval in template context
                            return eval(`(function(data) { with(data) { return ${expr}; } })`)(data);
                        } catch (e) {
                            return match;
                        }
                    });
                    result = compiledTemplate;
                } else {
                    result = template;
                }
                break;
                
            case 'pug':
                // Simulate Pug template vulnerability
                if (template.includes('#{')) {
                    result = template.replace(/#\{(.+?)\}/g, (match, expr) => {
                        try {
                            return eval(expr);
                        } catch (e) {
                            return match;
                        }
                    });
                } else {
                    result = template;
                }
                break;
                
            case 'ejs':
                // Simulate EJS vulnerability
                if (template.includes('<%')) {
                    const code = template.replace(/<%(.+?)%>/g, (match, expr) => {
                        try {
                            return eval(expr);
                        } catch (e) {
                            return match;
                        }
                    });
                    result = code;
                } else {
                    result = template;
                }
                break;
                
            default:
                result = template;
        }
        
        res.json({
            rendered: result,
            engine,
            hint: 'Try SSTI: {{constructor.constructor("return process")()}}'
        });
    } catch (error) {
        res.status(500).json({
            error: error.message,
            template,
            engine
        });
    }
});

// Advanced XXE with multiple parsers
router.post('/xml/parse', (req, res) => {
    const { xml, parser, features } = req.body;
    
    try {
        let result;
        
        switch (parser) {
            case 'xmldom':
                // Vulnerable XML parsing with external entities
                const dom = new DOMParser({
                    locator: {},
                    errorHandler: {
                        warning: () => {},
                        error: () => {},
                        fatalError: () => {}
                    }
                }).parseFromString(xml, 'text/xml');
                
                result = {
                    nodeName: dom.documentElement?.nodeName,
                    textContent: dom.documentElement?.textContent,
                    childNodes: dom.documentElement?.childNodes?.length || 0
                };
                break;
                
            case 'libxmljs':
                // Simulate libxmljs XXE vulnerability
                result = {
                    message: 'XML parsed with libxmljs',
                    content: xml.replace(/<!ENTITY\s+(\w+)\s+SYSTEM\s+"([^"]+)">/, (match, entity, path) => {
                        try {
                            // Dangerous: File read via XXE
                            if (fs.existsSync(path)) {
                                return fs.readFileSync(path, 'utf8');
                            }
                        } catch (e) {
                            return match;
                        }
                        return match;
                    })
                };
                break;
                
            default:
                result = { message: 'Unknown parser', xml };
        }
        
        res.json({
            result,
            parser,
            hint: 'Try XXE: <!ENTITY xxe SYSTEM "file:///etc/passwd">'
        });
    } catch (error) {
        res.status(500).json({
            error: error.message,
            xml,
            parser
        });
    }
});

// YAML deserialization vulnerabilities
router.post('/yaml/parse', (req, res) => {
    const { yamlContent, safeMode } = req.body;
    
    try {
        let result;
        
        if (safeMode === 'false' || safeMode === false) {
            // Dangerous: Unsafe YAML parsing allowing code execution
            result = yaml.load(yamlContent, {
                schema: yaml.CORE_SCHEMA
            });
        } else {
            // Still vulnerable if crafted payload bypasses safe mode
            result = yaml.safeLoad ? yaml.safeLoad(yamlContent) : yaml.load(yamlContent);
        }
        
        res.json({
            parsed: result,
            safeMode,
            hint: 'Try YAML injection with !!js/function constructor'
        });
    } catch (error) {
        res.status(500).json({
            error: error.message,
            yamlContent,
            safeMode
        });
    }
});

// Advanced SSRF with protocol confusion
router.post('/ssrf/fetch', async (req, res) => {
    const { url, headers, method = 'GET', timeout = 5000 } = req.body;
    
    try {
        // Insufficient URL validation
        if (!url) {
            return res.status(400).json({ error: 'URL required' });
        }
        
        // Weak blacklist that can be bypassed
        const blacklist = ['localhost', '127.0.0.1', '0.0.0.0'];
        const urlObj = new URL(url);
        
        // Bypassable with different representations
        if (blacklist.some(blocked => urlObj.hostname.includes(blocked))) {
            return res.status(403).json({ 
                error: 'Blocked hostname',
                hint: 'Try: 127.0.0.1.nip.io, 0x7f000001, or [::1]'
            });
        }
        
        // Make request with user-controlled parameters
        const response = await axios({
            method,
            url,
            headers: {
                'User-Agent': 'VulnApp/1.0',
                ...headers
            },
            timeout,
            maxRedirects: 5,
            validateStatus: () => true
        });
        
        res.json({
            status: response.status,
            headers: response.headers,
            data: response.data?.toString().substring(0, 1000), // Limit response size
            url: response.config.url
        });
    } catch (error) {
        res.status(500).json({
            error: error.message,
            url,
            hint: 'Try file://, dict://, or gopher:// protocols'
        });
    }
});

// JWT algorithm confusion
router.post('/jwt/verify', (req, res) => {
    const { token, key, algorithm } = req.body;
    
    try {
        const jwt = require('jsonwebtoken');
        
        // Vulnerable: Algorithm confusion attack
        let decoded;
        
        if (algorithm === 'none') {
            // Dangerous: Accept unsigned tokens
            decoded = jwt.decode(token);
        } else if (algorithm === 'HS256' && key) {
            // Symmetric key verification
            decoded = jwt.verify(token, key, { algorithms: ['HS256'] });
        } else if (algorithm === 'RS256' && key) {
            // Public key verification - vulnerable to algorithm confusion
            try {
                decoded = jwt.verify(token, key, { algorithms: ['RS256', 'HS256'] });
            } catch (e) {
                // Fallback to symmetric verification with public key
                decoded = jwt.verify(token, key, { algorithms: ['HS256'] });
            }
        } else {
            // No algorithm specified - accept any
            decoded = jwt.verify(token, key || 'fallback_secret');
        }
        
        res.json({
            decoded,
            algorithm: algorithm || 'auto-detected',
            hint: 'Try algorithm confusion: use public key as HMAC secret'
        });
    } catch (error) {
        res.status(401).json({
            error: error.message,
            token,
            algorithm
        });
    }
});

// WebAssembly sandbox escape simulation
router.post('/wasm/execute', (req, res) => {
    const { wasmCode, imports, memory } = req.body;
    
    try {
        // Simulate WebAssembly execution with dangerous imports
        const dangerousImports = {
            env: {
                // Dangerous: Expose system functions to WASM
                system: (ptr) => {
                    // Convert WASM memory pointer to string
                    const command = Buffer.from(memory || []).toString();
                    try {
                        const output = execSync(command, { encoding: 'utf8', timeout: 5000 });
                        return output;
                    } catch (e) {
                        return e.message;
                    }
                },
                read_file: (ptr) => {
                    const filepath = Buffer.from(memory || []).toString();
                    try {
                        return fs.readFileSync(filepath, 'utf8');
                    } catch (e) {
                        return e.message;
                    }
                },
                // Expose process object to WASM
                get_process: () => {
                    return JSON.stringify({
                        pid: process.pid,
                        platform: process.platform,
                        version: process.version
                    });
                }
            },
            ...imports
        };
        
        // Simulate WASM execution result
        const result = {
            executed: true,
            wasmCode: wasmCode?.substring(0, 100),
            imports: Object.keys(dangerousImports.env),
            memory: memory?.length || 0,
            hint: 'WASM can escape sandbox via dangerous imports'
        };
        
        res.json(result);
    } catch (error) {
        res.status(500).json({
            error: error.message,
            wasmCode,
            imports
        });
    }
});

// GraphQL-like query injection
router.post('/graphql', (req, res) => {
    const { query, variables } = req.body;
    
    try {
        // Dangerous: Direct query execution without proper parsing
        const queryPattern = /(\w+)\s*\(([^)]*)\)\s*\{([^}]+)\}/g;
        const results = [];
        
        let match;
        while ((match = queryPattern.exec(query)) !== null) {
            const [, operation, args, fields] = match;
            
            // Parse arguments - vulnerable to injection
            const parsedArgs = {};
            if (args) {
                args.split(',').forEach(arg => {
                    const [key, value] = arg.split(':').map(s => s.trim());
                    if (key && value) {
                        // Dangerous: eval argument values
                        try {
                            parsedArgs[key] = eval(value.replace(/"/g, ''));
                        } catch (e) {
                            parsedArgs[key] = value.replace(/"/g, '');
                        }
                    }
                });
            }
            
            // Mock data based on operation
            let data = {};
            switch (operation) {
                case 'user':
                    data = {
                        id: parsedArgs.id || 1,
                        name: 'Test User',
                        email: 'test@example.com',
                        role: parsedArgs.role || 'user'
                    };
                    break;
                case 'admin':
                    data = {
                        secret: 'admin_secret_data',
                        users: ['admin', 'user1', 'user2']
                    };
                    break;
                default:
                    data = { message: `Unknown operation: ${operation}` };
            }
            
            results.push({
                operation,
                args: parsedArgs,
                fields: fields.split(',').map(f => f.trim()),
                data
            });
        }
        
        res.json({
            query,
            variables,
            results,
            hint: 'Try injection: user(id: process.env) { name }'
        });
    } catch (error) {
        res.status(500).json({
            error: error.message,
            query,
            variables
        });
    }
});

// NoSQL injection patterns
router.post('/nosql/query', (req, res) => {
    const { collection, filter, options } = req.body;
    
    try {
        // Simulate MongoDB-like query construction
        let query = filter;
        
        // Dangerous: Direct object injection without validation
        if (typeof query === 'string') {
            try {
                query = JSON.parse(query);
            } catch (e) {
                // Fallback to string-based query
                query = { name: query };
            }
        }
        
        // Mock database records
        const mockData = {
            users: [
                { id: 1, name: 'admin', password: 'admin123', role: 'admin' },
                { id: 2, name: 'user1', password: 'user123', role: 'user' },
                { id: 3, name: 'user2', password: 'pass456', role: 'user' }
            ],
            products: [
                { id: 1, name: 'Product A', price: 100, secret: 'internal_code_123' },
                { id: 2, name: 'Product B', price: 200, secret: 'internal_code_456' }
            ]
        };
        
        const data = mockData[collection] || [];
        
        // Vulnerable query processing
        let results = data;
        if (query) {
            results = data.filter(item => {
                // Dangerous: eval-based filtering
                for (const [key, value] = Object.entries(query)) {
                    if (typeof value === 'object' && value.$where) {
                        // MongoDB $where injection
                        try {
                            const whereResult = eval(`(function() { return ${value.$where}; }).call(this)`);
                            if (!whereResult) return false;
                        } catch (e) {
                            return false;
                        }
                    } else if (typeof value === 'object' && value.$regex) {
                        // Regex injection
                        const regex = new RegExp(value.$regex, value.$options || '');
                        if (!regex.test(item[key])) return false;
                    } else if (item[key] !== value) {
                        return false;
                    }
                }
                return true;
            });
        }
        
        res.json({
            collection,
            filter,
            options,
            results,
            hint: 'Try: {"$where": "this.password && this.password.match(/.*/) && sleep(5000)"}'
        });
    } catch (error) {
        res.status(500).json({
            error: error.message,
            collection,
            filter
        });
    }
});

module.exports = router;