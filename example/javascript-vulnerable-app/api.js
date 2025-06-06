/*!
 * API Routes for Advanced Vulnerable JavaScript Application
 * 
 * Contains various API endpoints with intentional security vulnerabilities
 * FOR TESTING PURPOSES ONLY
 */

const express = require('express');
const sqlite3 = require('sqlite3').verbose();
const axios = require('axios');
const cheerio = require('cheerio');
const _ = require('lodash');
const yaml = require('yaml');
const xml2js = require('xml2js');
const jwt = require('jsonwebtoken');
const handlebars = require('handlebars');
const ejs = require('ejs');
const serialize = require('serialize-javascript');
const { execSync, spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const archiver = require('archiver');
const extract = require('extract-zip');
const winston = require('winston');

const router = express.Router();
const db = new sqlite3.Database('vulnerable_app.db');

// Vulnerable: Hardcoded secrets
const JWT_SECRET = 'super_secret_js_key_123';
const API_KEYS = {
    'sk-js-1234567890abcdef': 'admin',
    'pk-js-0987654321fedcba': 'guest'
};

const logger = winston.createLogger({
    level: 'debug',
    format: winston.format.json(),
    transports: [new winston.transports.Console()]
});

// Authentication APIs
router.post('/auth/login', (req, res) => {
    const { username, password } = req.body;
    
    // Vulnerable: SQL injection in authentication
    const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
    
    logger.info(`API login attempt: ${username}:${password}`, { username, password, ip: req.ip });
    
    db.get(query, (err, user) => {
        if (err) {
            return res.status(500).json({ 
                error: `Authentication failed: ${err.message}`,
                query: query // Vulnerable: Exposing query in error
            });
        }
        
        if (user) {
            // Vulnerable: Weak JWT implementation
            const token = jwt.sign({
                user_id: user.id,
                username: user.username,
                role: user.role
            }, JWT_SECRET, { algorithm: 'HS256' });
            
            // Vulnerable: Log sensitive information
            const logQuery = `INSERT INTO audit_logs (user_id, action, details, ip_address, user_agent) 
                             VALUES (${user.id}, 'API_LOGIN', 'User ${username} logged in with password ${password}', '${req.ip}', '${req.get('User-Agent')}')`;
            db.run(logQuery);
            
            res.json({
                token: token,
                user: user,
                api_key: user.api_key // Vulnerable: Exposing API key
            });
        } else {
            res.status(401).json({ error: `Invalid credentials for user '${username}'` });
        }
    });
});

router.post('/auth/jwt-login', (req, res) => {
    const { token } = req.body;
    
    try {
        // Vulnerable: JWT verification without proper validation
        const decoded = jwt.verify(token, JWT_SECRET);
        res.json({ message: 'JWT valid', user: decoded });
    } catch (error) {
        res.status(401).json({ error: `JWT validation failed: ${error.message}` });
    }
});

// User management APIs with IDOR vulnerabilities
router.get('/user/:id', (req, res) => {
    const userId = req.params.id;
    
    // Vulnerable: No authorization check (IDOR)
    const query = `SELECT * FROM users WHERE id = ${userId}`;
    
    db.get(query, (err, user) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        if (user) {
            res.json(user);
        } else {
            res.status(404).json({ error: 'User not found' });
        }
    });
});

router.get('/users/search', (req, res) => {
    const { q, role, limit } = req.query;
    
    // Vulnerable: SQL injection in search
    let query = `SELECT * FROM users WHERE 1=1`;
    
    if (q) {
        query += ` AND username LIKE '%${q}%'`;
    }
    
    if (role) {
        query += ` AND role = '${role}'`;
    }
    
    if (limit) {
        query += ` LIMIT ${limit}`;
    }
    
    db.all(query, (err, users) => {
        if (err) {
            return res.status(500).json({ 
                error: err.message,
                query: query // Vulnerable: Exposing query
            });
        }
        
        res.json({ users, query });
    });
});

// Document management with vulnerabilities
router.get('/documents/:id', (req, res) => {
    const docId = req.params.id;
    
    // Vulnerable: SQL injection + IDOR
    const query = `SELECT * FROM documents WHERE id = ${docId}`;
    
    db.get(query, (err, document) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        if (document) {
            res.json(document);
        } else {
            res.status(404).json({ error: 'Document not found' });
        }
    });
});

router.get('/documents/:id/content', (req, res) => {
    const docId = req.params.id;
    
    // Vulnerable: Path traversal via database
    const query = `SELECT file_path FROM documents WHERE id = ${docId}`;
    
    db.get(query, (err, document) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        if (document && document.file_path) {
            try {
                // Vulnerable: No path validation (LFI)
                const content = fs.readFileSync(document.file_path, 'utf8');
                res.json({ content, file_path: document.file_path });
            } catch (error) {
                res.status(500).json({ 
                    error: `Failed to read file: ${error.message}`,
                    file_path: document.file_path
                });
            }
        } else {
            res.status(404).json({ error: 'Document not found' });
        }
    });
});

// SSRF vulnerabilities
router.post('/ssrf/fetch', async (req, res) => {
    const { url, timeout = 5000 } = req.body;
    
    try {
        // Vulnerable: No URL validation (SSRF)
        const response = await axios.get(url, { timeout });
        
        res.json({
            status_code: response.status,
            content: response.data.toString().substring(0, 1000),
            headers: response.headers
        });
    } catch (error) {
        res.status(500).json({ error: error.message, url });
    }
});

router.post('/scraper/url', async (req, res) => {
    const { url, selector } = req.body;
    
    try {
        // Vulnerable: SSRF + potential HTML injection
        const response = await axios.get(url);
        const $ = cheerio.load(response.data);
        
        let result;
        if (selector) {
            result = $(selector).text();
        } else {
            result = $('title').text();
        }
        
        res.json({ url, selector, result });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// XXE vulnerability
router.post('/xml/parse', (req, res) => {
    const xmlData = req.body;
    
    // Vulnerable: XXE attack vector
    const parser = new xml2js.Parser({
        explicitChildren: true,
        preserveChildrenOrder: true
    });
    
    parser.parseString(xmlData, (err, result) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        res.json({ parsed_xml: result });
    });
});

// Template injection vulnerabilities
router.post('/template/handlebars', (req, res) => {
    const { template, context } = req.body;
    
    try {
        // Vulnerable: Template injection
        const compiledTemplate = handlebars.compile(template);
        const rendered = compiledTemplate(context || {});
        
        res.json({ rendered });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

router.post('/template/ejs', (req, res) => {
    const { template, context } = req.body;
    
    try {
        // Vulnerable: Template injection
        const rendered = ejs.render(template, context || {});
        
        res.json({ rendered });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Prototype pollution vulnerabilities
router.post('/config/merge', (req, res) => {
    const { config } = req.body;
    
    // Vulnerable: Prototype pollution via lodash merge
    const defaultConfig = {};
    const mergedConfig = _.merge(defaultConfig, config);
    
    res.json({ 
        merged_config: mergedConfig,
        prototype_polluted: Object.prototype.polluted || false
    });
});

router.post('/object/deep-merge', (req, res) => {
    const { target, source } = req.body;
    
    // Vulnerable: Manual deep merge with prototype pollution
    function deepMerge(target, source) {
        for (let key in source) {
            if (source[key] && typeof source[key] === 'object') {
                if (!target[key]) target[key] = {};
                deepMerge(target[key], source[key]);
            } else {
                target[key] = source[key];
            }
        }
        return target;
    }
    
    const result = deepMerge(target || {}, source || {});
    
    res.json({ 
        result,
        prototype_polluted: Object.prototype.polluted || false
    });
});

// Command injection vulnerabilities
router.post('/system/execute', (req, res) => {
    const { command, args } = req.body;
    
    try {
        // Vulnerable: Direct command execution
        const fullCommand = args ? `${command} ${args.join(' ')}` : command;
        const output = execSync(fullCommand, { encoding: 'utf8', timeout: 5000 });
        
        res.json({
            command: fullCommand,
            output: output
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            command: command
        });
    }
});

// File operation vulnerabilities
router.get('/file/read', (req, res) => {
    const { path: filePath } = req.query;
    
    try {
        // Vulnerable: No path validation (LFI)
        const content = fs.readFileSync(filePath, 'utf8');
        res.json({ file_path: filePath, content });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            file_path: filePath
        });
    }
});

router.get('/file/list', (req, res) => {
    const { dir } = req.query;
    
    try {
        // Vulnerable: Directory traversal
        const files = fs.readdirSync(dir || '.');
        res.json({ directory: dir, files });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

router.post('/file/upload', (req, res) => {
    // This will be handled by multer middleware in main app
    if (!req.file) {
        return res.status(400).json({ error: 'No file uploaded' });
    }
    
    // Vulnerable: No file type validation, predictable paths
    const uploadPath = `/tmp/uploads/${req.file.originalname}`;
    
    try {
        fs.renameSync(req.file.path, uploadPath);
        res.json({
            message: 'File uploaded successfully',
            filename: req.file.originalname,
            path: uploadPath
        });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

router.post('/archive/extract', async (req, res) => {
    if (!req.file) {
        return res.status(400).json({ error: 'No archive uploaded' });
    }
    
    const extractDir = `/tmp/extracted_${Date.now()}`;
    
    try {
        // Vulnerable: Zip slip attack - no path validation
        await extract(req.file.path, { dir: extractDir });
        
        const extractedFiles = fs.readdirSync(extractDir);
        
        res.json({
            message: 'Archive extracted successfully',
            extracted_files: extractedFiles,
            extract_dir: extractDir
        });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Serialization vulnerabilities
router.post('/serialize/js', (req, res) => {
    const { data } = req.body;
    
    try {
        // Vulnerable: Unsafe serialization
        const serialized = serialize(data, { unsafe: true });
        res.json({ serialized });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

router.post('/deserialize/eval', (req, res) => {
    const { serialized } = req.body;
    
    try {
        // Vulnerable: Using eval for deserialization
        const deserialized = eval(`(${serialized})`);
        res.json({ deserialized });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// YAML deserialization
router.post('/yaml/parse', (req, res) => {
    const yamlData = req.body.toString();
    
    try {
        // Vulnerable: YAML deserialization
        const parsed = yaml.parse(yamlData);
        res.json({ parsed_yaml: parsed });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Audit log access (IDOR)
router.get('/logs/:user_id', (req, res) => {
    const userId = req.params.user_id;
    
    // Vulnerable: No authorization check + SQL injection
    const query = `SELECT * FROM audit_logs WHERE user_id = ${userId} ORDER BY timestamp DESC`;
    
    db.all(query, (err, logs) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        res.json({ logs });
    });
});

// Comments API for stored XSS
router.post('/comments/create', (req, res) => {
    const { content, author } = req.body;
    
    // Vulnerable: Stored XSS - no sanitization
    const query = `INSERT INTO comments (content, author, created_at) VALUES ('${content}', '${author}', datetime('now'))`;
    
    db.run(query, function(err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        res.json({ 
            id: this.lastID,
            message: 'Comment created successfully'
        });
    });
});

router.get('/comments', (req, res) => {
    db.all('SELECT * FROM comments ORDER BY created_at DESC', (err, comments) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        res.json({ comments });
    });
});

// API documentation endpoint
router.get('/docs', (req, res) => {
    res.json({
        title: 'Vulnerable JavaScript API Documentation',
        version: '1.0.0',
        description: 'API endpoints with intentional security vulnerabilities',
        endpoints: {
            authentication: [
                'POST /api/auth/login',
                'POST /api/auth/jwt-login'
            ],
            users: [
                'GET /api/user/:id',
                'GET /api/users/search'
            ],
            documents: [
                'GET /api/documents/:id',
                'GET /api/documents/:id/content'
            ],
            ssrf: [
                'POST /api/ssrf/fetch',
                'POST /api/scraper/url'
            ],
            injection: [
                'POST /api/xml/parse',
                'POST /api/template/handlebars',
                'POST /api/template/ejs',
                'POST /api/system/execute'
            ],
            prototype_pollution: [
                'POST /api/config/merge',
                'POST /api/object/deep-merge'
            ],
            file_operations: [
                'GET /api/file/read',
                'GET /api/file/list',
                'POST /api/file/upload',
                'POST /api/archive/extract'
            ],
            serialization: [
                'POST /api/serialize/js',
                'POST /api/deserialize/eval',
                'POST /api/yaml/parse'
            ],
            comments: [
                'POST /api/comments/create',
                'GET /api/comments'
            ],
            logs: [
                'GET /api/logs/:user_id'
            ]
        },
        warning: 'All endpoints contain intentional security vulnerabilities. Use only for testing purposes.'
    });
});

module.exports = router;