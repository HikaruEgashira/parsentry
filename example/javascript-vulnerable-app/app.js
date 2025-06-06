/*!
 * Advanced Vulnerable JavaScript/Node.js Application
 * 
 * A sophisticated, intentionally vulnerable Node.js web application designed for testing
 * advanced security analysis tools. Features enterprise-level complexity with
 * multi-layered architecture and complex vulnerability patterns.
 * 
 * ⚠️ FOR TESTING PURPOSES ONLY - Contains severe security vulnerabilities
 */

const express = require('express');
const sqlite3 = require('sqlite3').verbose();
const bodyParser = require('body-parser');
const cookieParser = require('cookie-parser');
const session = require('express-session');
const multer = require('multer');
const path = require('path');
const fs = require('fs');
const crypto = require('crypto');
const axios = require('axios');
const cheerio = require('cheerio');
const _ = require('lodash');
const yaml = require('yaml');
const xml2js = require('xml2js');
const jwt = require('jsonwebtoken');
const bcrypt = require('bcrypt');
const winston = require('winston');
const handlebars = require('handlebars');
const ejs = require('ejs');
const serialize = require('serialize-javascript');
const forge = require('node-forge');
const { execSync, spawn } = require('child_process');
const { promisify } = require('util');
const archiver = require('archiver');
const extract = require('extract-zip');

// Initialize Express app
const app = express();
const PORT = process.env.PORT || 3000;

// Vulnerable: Hardcoded secrets and configuration
const JWT_SECRET = 'super_secret_js_key_123';
const SESSION_SECRET = 'vulnerable_session_secret';
const API_KEYS = {
    'sk-js-1234567890abcdef': 'admin',
    'pk-js-0987654321fedcba': 'guest'
};

// Configure logging with potential security issues
const logger = winston.createLogger({
    level: 'debug',
    format: winston.format.combine(
        winston.format.timestamp(),
        winston.format.json()
    ),
    transports: [
        new winston.transports.Console(),
        new winston.transports.File({ filename: 'app.log' })
    ]
});

// Vulnerable middleware configuration
app.use(bodyParser.json({ limit: '50mb' })); // Vulnerable: Large payload limit
app.use(bodyParser.urlencoded({ extended: true, limit: '50mb' }));
app.use(cookieParser());
app.use(session({
    secret: SESSION_SECRET, // Vulnerable: Weak session secret
    resave: false,
    saveUninitialized: true,
    cookie: { 
        secure: false, // Vulnerable: No HTTPS requirement
        httpOnly: false, // Vulnerable: Accessible via JavaScript
        maxAge: 24 * 60 * 60 * 1000 // 24 hours
    }
}));

// Configure file upload with vulnerabilities
const upload = multer({ 
    dest: '/tmp/uploads/', // Vulnerable: Predictable upload directory
    limits: { fileSize: 100 * 1024 * 1024 }, // 100MB limit
    fileFilter: (req, file, cb) => {
        // Vulnerable: No file type validation
        cb(null, true);
    }
});

// Initialize SQLite database with vulnerable schema
const db = new sqlite3.Database('vulnerable_app.db');

// Vulnerable database initialization
db.serialize(() => {
    // Users table with plain text passwords
    db.run(`CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT UNIQUE NOT NULL,
        password TEXT NOT NULL,
        email TEXT,
        role TEXT DEFAULT 'user',
        api_key TEXT,
        session_token TEXT,
        metadata TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )`);

    // Documents table
    db.run(`CREATE TABLE IF NOT EXISTS documents (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        content TEXT,
        owner_id INTEGER,
        file_path TEXT,
        metadata TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (owner_id) REFERENCES users (id)
    )`);

    // Audit logs table
    db.run(`CREATE TABLE IF NOT EXISTS audit_logs (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER,
        action TEXT,
        details TEXT,
        ip_address TEXT,
        user_agent TEXT,
        timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
    )`);

    // Insert vulnerable default data
    db.run(`INSERT OR IGNORE INTO users (username, password, email, role, api_key) 
            VALUES ('admin', 'admin123', 'admin@example.com', 'admin', 'sk-js-1234567890abcdef')`);
    
    db.run(`INSERT OR IGNORE INTO users (username, password, email, role, api_key) 
            VALUES ('guest', 'guest', 'guest@example.com', 'user', 'pk-js-0987654321fedcba')`);

    // Sample documents with vulnerable file paths
    db.run(`INSERT OR IGNORE INTO documents (title, content, owner_id, file_path) 
            VALUES ('Secret Config', 'database_password=super_secret_123', 1, '/etc/passwd')`);
    
    db.run(`INSERT OR IGNORE INTO documents (title, content, owner_id, file_path) 
            VALUES ('User Data', 'Sensitive user information', 2, '../../etc/shadow')`);
});

// Enhanced main page with comprehensive vulnerability showcase
app.get('/', (req, res) => {
    const user = req.session.user;
    
    res.send(`
    <!DOCTYPE html>
    <html>
    <head>
        <title>🔓 Advanced Vulnerable JavaScript Application</title>
        <style>
            body { 
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
                margin: 0; 
                padding: 20px; 
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                min-height: 100vh;
            }
            .container { 
                max-width: 1200px; 
                margin: 0 auto; 
                background: white; 
                padding: 40px; 
                border-radius: 12px; 
                box-shadow: 0 10px 30px rgba(0,0,0,0.2); 
            }
            .header { 
                text-align: center; 
                margin-bottom: 40px; 
                padding-bottom: 20px; 
                border-bottom: 3px solid #667eea; 
            }
            .header h1 { 
                color: #333; 
                margin: 0 0 10px 0; 
                font-size: 2.5em; 
            }
            .header p { 
                color: #666; 
                font-size: 1.2em; 
                margin: 0; 
            }
            .section { 
                margin: 30px 0; 
                padding: 30px; 
                border: 2px solid #e0e0e0; 
                border-radius: 12px; 
                background: #fafafa; 
            }
            .section h2 { 
                color: #333; 
                margin-top: 0; 
                border-bottom: 2px solid #667eea; 
                padding-bottom: 10px; 
                font-size: 1.8em; 
            }
            .vuln-grid { 
                display: grid; 
                grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); 
                gap: 25px; 
                margin-top: 20px; 
            }
            .vuln-card { 
                background: white; 
                padding: 25px; 
                border-radius: 10px; 
                border-left: 5px solid #e74c3c; 
                box-shadow: 0 4px 6px rgba(0,0,0,0.1); 
                transition: transform 0.2s; 
            }
            .vuln-card:hover { 
                transform: translateY(-2px); 
                box-shadow: 0 6px 12px rgba(0,0,0,0.15); 
            }
            .vuln-title { 
                font-weight: bold; 
                color: #e74c3c; 
                margin-bottom: 10px; 
                font-size: 1.1em; 
            }
            .vuln-desc { 
                color: #666; 
                margin-bottom: 15px; 
                line-height: 1.4; 
            }
            .endpoint { 
                background: #f8f9fa; 
                padding: 12px; 
                margin: 8px 0; 
                border-radius: 6px; 
                border-left: 3px solid #28a745; 
                font-family: 'Courier New', monospace; 
                font-size: 0.9em; 
            }
            .method { 
                display: inline-block; 
                color: white; 
                padding: 4px 8px; 
                border-radius: 4px; 
                font-size: 0.8em; 
                margin-right: 10px; 
                font-weight: bold; 
            }
            .get { background: #007bff; }
            .post { background: #28a745; }
            .put { background: #ffc107; color: #000; }
            .delete { background: #dc3545; }
            .cwe { 
                background: #dc3545; 
                color: white; 
                padding: 2px 6px; 
                border-radius: 3px; 
                font-size: 0.75em; 
                margin-left: 8px; 
            }
            .warning { 
                background: #fff3cd; 
                border: 2px solid #ffeaa7; 
                color: #856404; 
                padding: 20px; 
                border-radius: 8px; 
                margin: 25px 0; 
                font-weight: 500; 
            }
            .nav-links { 
                display: grid; 
                grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); 
                gap: 15px; 
                margin-top: 20px; 
            }
            .nav-link { 
                display: block; 
                padding: 15px; 
                background: #667eea; 
                color: white; 
                text-decoration: none; 
                border-radius: 8px; 
                text-align: center; 
                transition: background 0.2s; 
                font-weight: 500; 
            }
            .nav-link:hover { 
                background: #5a6fd8; 
                text-decoration: none; 
                color: white; 
            }
            a { color: #667eea; text-decoration: none; font-weight: 500; }
            a:hover { text-decoration: underline; }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <h1>🔓 Advanced Vulnerable JavaScript Application</h1>
                <p>Enterprise-level Node.js security testing platform with complex vulnerability patterns</p>
                ${user ? `<p style="color: #28a745;">Welcome, ${user.username}! | <a href="/logout">Logout</a></p>` : 
                          '<p><a href="/login">Login</a> to access more features</p>'}
            </div>
            
            <div class="warning">
                <strong>⚠️ SECURITY WARNING:</strong> This application contains severe security vulnerabilities by design.
                Use only in isolated testing environments. DO NOT expose to public networks.
            </div>
            
            <div class="section">
                <h2>🌐 Classic Web Vulnerabilities</h2>
                <div class="vuln-grid">
                    <div class="vuln-card">
                        <div class="vuln-title">SQL Injection <span class="cwe">CWE-89</span></div>
                        <div class="vuln-desc">Multiple injection points with complex queries and NoSQL variants</div>
                        <div class="endpoint">
                            <span class="method get">GET</span>
                            <a href="/sqli?username=admin&order=id">/sqli</a>
                        </div>
                        <div class="endpoint">
                            <span class="method get">GET</span>
                            <a href="/api/users/search?q=admin">/api/users/search</a>
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">XSS Attacks <span class="cwe">CWE-79</span></div>
                        <div class="vuln-desc">Multiple XSS contexts including DOM-based and stored XSS</div>
                        <div class="endpoint">
                            <span class="method get">GET</span>
                            <a href="/xss?name=test&comment=hello">/xss</a>
                        </div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/comments/create
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">Command Injection <span class="cwe">CWE-78</span></div>
                        <div class="vuln-desc">System command execution via multiple vectors</div>
                        <div class="endpoint">
                            <span class="method get">GET</span>
                            <a href="/cmdi?cmd=ls&args=-la">/cmdi</a>
                        </div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/system/execute
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>🔐 Authentication & Authorization</h2>
                <div class="vuln-grid">
                    <div class="vuln-card">
                        <div class="vuln-title">Authentication Bypass</div>
                        <div class="vuln-desc">Weak authentication mechanisms and session handling</div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/auth/login
                        </div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/auth/jwt-login
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">IDOR <span class="cwe">CWE-639</span></div>
                        <div class="vuln-desc">Insecure Direct Object References</div>
                        <div class="endpoint">
                            <span class="method get">GET</span>
                            <a href="/api/user/1">/api/user/*</a>
                        </div>
                        <div class="endpoint">
                            <span class="method get">GET</span>
                            <a href="/api/documents/1">/api/documents/*</a>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>🌍 Network & Injection Attacks</h2>
                <div class="vuln-grid">
                    <div class="vuln-card">
                        <div class="vuln-title">SSRF <span class="cwe">CWE-918</span></div>
                        <div class="vuln-desc">Server-Side Request Forgery with URL manipulation</div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/ssrf/fetch
                        </div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/scraper/url
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">XXE <span class="cwe">CWE-611</span></div>
                        <div class="vuln-desc">XML External Entity injection</div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/xml/parse
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">Template Injection <span class="cwe">CWE-94</span></div>
                        <div class="vuln-desc">Server-side template injection in multiple engines</div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/template/handlebars
                        </div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/template/ejs
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">Prototype Pollution <span class="cwe">CWE-1321</span></div>
                        <div class="vuln-desc">JavaScript prototype pollution attacks</div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/config/merge
                        </div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/object/deep-merge
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>📁 File Operation Vulnerabilities</h2>
                <div class="vuln-grid">
                    <div class="vuln-card">
                        <div class="vuln-title">Unrestricted Upload</div>
                        <div class="vuln-desc">File upload without validation</div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/file/upload
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">Path Traversal <span class="cwe">CWE-22</span></div>
                        <div class="vuln-desc">Directory traversal and file inclusion</div>
                        <div class="endpoint">
                            <span class="method get">GET</span>
                            <a href="/api/file/read?path=package.json">/api/file/read</a>
                        </div>
                        <div class="endpoint">
                            <span class="method get">GET</span>
                            <a href="/api/file/list?dir=.">/api/file/list</a>
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">Archive Extraction</div>
                        <div class="vuln-desc">Zip slip and archive extraction vulnerabilities</div>
                        <div class="endpoint">
                            <span class="method post">POST</span>
                            /api/archive/extract
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>🧪 Testing & Documentation</h2>
                <div class="nav-links">
                    <a href="/api/docs" class="nav-link">📚 API Documentation</a>
                    <a href="/test" class="nav-link">🧪 Vulnerability Test Suite</a>
                    <a href="/logs" class="nav-link">📊 Audit Logs</a>
                    <a href="/metrics" class="nav-link">📈 Security Metrics</a>
                </div>
            </div>
        </div>
        
        <script>
            // Vulnerable: XSS in JavaScript context
            const userName = '${user ? user.username : 'anonymous'}';
            console.log('Current user: ' + userName);
            
            // Vulnerable: DOM manipulation without sanitization
            if (location.hash) {
                document.body.innerHTML += '<div>Hash: ' + location.hash + '</div>';
            }
        </script>
    </body>
    </html>
    `);
});

// Authentication endpoints
app.get('/login', (req, res) => {
    res.send(`
    <!DOCTYPE html>
    <html>
    <head>
        <title>Login - Vulnerable JS App</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }
            .container { max-width: 400px; margin: 0 auto; background: white; padding: 40px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
            .form-group { margin: 20px 0; }
            .form-group label { display: block; margin-bottom: 5px; font-weight: bold; }
            .form-group input { width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; box-sizing: border-box; }
            .btn { background: #667eea; color: white; padding: 12px 20px; border: none; border-radius: 4px; cursor: pointer; width: 100%; }
            .btn:hover { background: #5a6fd8; }
        </style>
    </head>
    <body>
        <div class="container">
            <h2>Login</h2>
            <form method="post" action="/login">
                <div class="form-group">
                    <label>Username:</label>
                    <input type="text" name="username" required>
                </div>
                <div class="form-group">
                    <label>Password:</label>
                    <input type="password" name="password" required>
                </div>
                <button type="submit" class="btn">Login</button>
            </form>
            <p style="margin-top: 20px; color: #666;">Default credentials: admin/admin123 or guest/guest</p>
            <p><a href="/">Back to Home</a></p>
        </div>
    </body>
    </html>
    `);
});

app.post('/login', (req, res) => {
    const { username, password } = req.body;
    
    // Vulnerable: SQL injection in authentication
    const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
    
    // Vulnerable: Log sensitive information
    logger.info(`Login attempt: ${username}:${password} from ${req.ip}`, {
        username,
        password,
        ip: req.ip,
        userAgent: req.get('User-Agent')
    });
    
    db.get(query, (err, user) => {
        if (err) {
            // Vulnerable: Information disclosure in error messages
            return res.status(500).send(`Database error: ${err.message}<br>Query: ${query}`);
        }
        
        if (user) {
            // Vulnerable: Storing sensitive data in session
            req.session.user = user;
            req.session.logged_in = true;
            
            // Vulnerable: Predictable session tokens
            req.session.session_token = crypto.createHash('md5').update(username + password).digest('hex');
            
            // Log successful authentication
            const logQuery = `INSERT INTO audit_logs (user_id, action, details, ip_address, user_agent) 
                             VALUES (${user.id}, 'WEB_LOGIN', 'User ${username} logged in with password ${password}', '${req.ip}', '${req.get('User-Agent')}')`;
            db.run(logQuery);
            
            res.redirect('/');
        } else {
            res.status(401).send(`
                <h2>Login Failed</h2>
                <p style="color: red;">Invalid credentials for user '${username}'</p>
                <a href="/login">Try Again</a>
            `);
        }
    });
});

app.get('/logout', (req, res) => {
    // Vulnerable: Session fixation - doesn't regenerate session ID
    req.session.user = null;
    req.session.logged_in = false;
    // Vulnerable: Leaves session_token in session
    res.redirect('/');
});

// Import advanced modules and routes
const authMiddleware = require('./middleware/auth');
const validationMiddleware = require('./middleware/validation');
const advancedRoutes = require('./routes/advanced');
const bypassRoutes = require('./routes/bypass');
const cryptoService = require('./services/crypto');
const dbService = require('./services/database');

// Import API routes
const apiRoutes = require('./api');

// Configure advanced middleware with vulnerabilities
app.use(validationMiddleware.lengthValidation(5000)); // Vulnerable: High limit
app.use(validationMiddleware.jsonValidation);

// Mount API routes with vulnerable middleware
app.use('/api', [
    authMiddleware.rateLimitMiddleware(1000, 60000), // Vulnerable: High rate limit
    validationMiddleware.xssFilter,
    validationMiddleware.sqlInjectionFilter,
    validationMiddleware.commandInjectionFilter
], apiRoutes);

// Mount advanced vulnerability routes
app.use('/advanced', [
    authMiddleware.debugAuth, // Vulnerable: Debug authentication
    validationMiddleware.pathTraversalFilter,
    validationMiddleware.ssrfProtection
], advancedRoutes);

// Mount bypass demonstration routes
app.use('/bypass', bypassRoutes);

// Vulnerable: Direct service exposure endpoints
app.get('/crypto/demo', (req, res) => {
    const { action, data, password } = req.query;
    
    switch (action) {
        case 'hash':
            res.json(cryptoService.hashPassword(password || 'test', 'demo_user'));
            break;
        case 'encrypt':
            res.json(cryptoService.encrypt(data || 'secret data'));
            break;
        case 'token':
            res.json(cryptoService.generateRandomToken(32));
            break;
        case 'jwt':
            res.json(cryptoService.signJWT({ user: 'demo', role: 'user' }));
            break;
        default:
            res.json({
                message: 'Crypto service demo',
                actions: ['hash', 'encrypt', 'token', 'jwt'],
                example: '/crypto/demo?action=hash&password=test'
            });
    }
});

app.post('/database/query', async (req, res) => {
    try {
        const result = await dbService.searchUsers(req.body);
        res.json(result);
    } catch (error) {
        res.status(500).json(error);
    }
});

app.post('/database/procedure', async (req, res) => {
    try {
        const result = await dbService.executeStoredProcedure(req.body.procedure, req.body.parameters);
        res.json(result);
    } catch (error) {
        res.status(500).json(error);
    }
});

// Vulnerable: Admin endpoints with weak protection
app.get('/admin/config', [
    authMiddleware.authenticateToken,
    authMiddleware.requireRole('admin')
], async (req, res) => {
    try {
        const backup = await dbService.createBackup();
        res.json(backup);
    } catch (error) {
        res.status(500).json(error);
    }
});

app.post('/admin/elevate', async (req, res) => {
    const { userId, targetRole, justification } = req.body;
    
    try {
        const result = await dbService.elevatePrivileges(userId, targetRole, justification);
        res.json(result);
    } catch (error) {
        res.status(500).json(error);
    }
});

// Vulnerable: File operations with multiple attack vectors
app.use('/files', upload.single('file'));

app.post('/files/upload', [
    validationMiddleware.fileUploadValidation
], (req, res) => {
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
            path: uploadPath,
            size: req.file.size
        });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Vulnerable: SQL injection and XSS demonstration endpoints
app.get('/sqli', (req, res) => {
    const { username, order } = req.query;
    
    if (!username) {
        return res.send(`
            <h2>SQL Injection Demo</h2>
            <form>
                <input type="text" name="username" placeholder="Enter username" value="admin">
                <select name="order">
                    <option value="id">Order by ID</option>
                    <option value="username">Order by Username</option>
                    <option value="created_at">Order by Date</option>
                </select>
                <button type="submit">Search</button>
            </form>
            <p>Try: <code>admin' OR '1'='1</code></p>
        `);
    }
    
    // Vulnerable: Direct SQL injection
    const query = `SELECT * FROM users WHERE username LIKE '%${username}%' ORDER BY ${order || 'id'}`;
    
    db.all(query, (err, users) => {
        if (err) {
            return res.send(`<h2>Error:</h2><p>${err.message}</p><p>Query: ${query}</p>`);
        }
        
        let html = `<h2>Search Results for: ${username}</h2>`;
        html += `<p>Query executed: <code>${query}</code></p>`;
        
        if (users.length > 0) {
            html += '<table border="1"><tr><th>ID</th><th>Username</th><th>Email</th><th>Role</th></tr>';
            users.forEach(user => {
                html += `<tr><td>${user.id}</td><td>${user.username}</td><td>${user.email}</td><td>${user.role}</td></tr>`;
            });
            html += '</table>';
        } else {
            html += '<p>No users found</p>';
        }
        
        res.send(html);
    });
});

app.get('/xss', (req, res) => {
    const { name, comment } = req.query;
    
    res.send(`
        <html>
        <head><title>XSS Demo</title></head>
        <body>
            <h2>XSS Demonstration</h2>
            <form>
                <input type="text" name="name" placeholder="Your name" value="${name || ''}">
                <textarea name="comment" placeholder="Your comment">${comment || ''}</textarea>
                <button type="submit">Submit</button>
            </form>
            ${name ? `<p>Hello, ${name}!</p>` : ''}
            ${comment ? `<div>Comment: ${comment}</div>` : ''}
            <p>Try: <code>&lt;script&gt;alert('XSS')&lt;/script&gt;</code></p>
        </body>
        </html>
    `);
});

app.get('/cmdi', (req, res) => {
    const { cmd, args } = req.query;
    
    if (!cmd) {
        return res.send(`
            <h2>Command Injection Demo</h2>
            <form>
                <input type="text" name="cmd" placeholder="Command" value="ls">
                <input type="text" name="args" placeholder="Arguments" value="-la">
                <button type="submit">Execute</button>
            </form>
            <p>Try: <code>ls; cat /etc/passwd</code></p>
        `);
    }
    
    try {
        // Vulnerable: Direct command execution
        const fullCommand = args ? `${cmd} ${args}` : cmd;
        const output = execSync(fullCommand, { encoding: 'utf8', timeout: 5000 });
        
        res.send(`
            <h2>Command Output</h2>
            <p>Executed: <code>${fullCommand}</code></p>
            <pre>${output}</pre>
        `);
    } catch (error) {
        res.send(`
            <h2>Command Error</h2>
            <p>Command: <code>${cmd} ${args || ''}</code></p>
            <p>Error: ${error.message}</p>
        `);
    }
});

// Enhanced test suite endpoint
app.get('/test', (req, res) => {
    res.send(`
    <!DOCTYPE html>
    <html>
    <head>
        <title>Vulnerability Test Suite</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .section { margin: 30px 0; padding: 20px; border: 1px solid #ddd; }
            .test-link { display: block; margin: 10px 0; padding: 10px; background: #f5f5f5; text-decoration: none; }
            .test-link:hover { background: #eee; }
            pre { background: #f8f8f8; padding: 10px; overflow-x: auto; }
        </style>
    </head>
    <body>
        <h1>🧪 Advanced Vulnerability Test Suite</h1>
        
        <div class="section">
            <h2>🔓 Authentication & Authorization Bypasses</h2>
            <a href="/advanced/auth/multi-step" class="test-link">Multi-Step Authentication Bypass</a>
            <a href="/bypass/auth/bypass-demo" class="test-link">Authentication Bypass Demonstrations</a>
            <a href="/api/auth/login" class="test-link">SQL Injection in Login</a>
        </div>
        
        <div class="section">
            <h2>💉 Injection Vulnerabilities</h2>
            <a href="/sqli?username=admin" class="test-link">SQL Injection Demo</a>
            <a href="/bypass/sql/filter-test" class="test-link">SQL Injection Filter Bypass</a>
            <a href="/cmdi?cmd=ls" class="test-link">Command Injection Demo</a>
            <a href="/bypass/command/bypass-test" class="test-link">Command Injection Bypass</a>
            <a href="/xss?name=<script>alert(1)</script>" class="test-link">XSS Demo</a>
            <a href="/bypass/xss/filter-test" class="test-link">XSS Filter Bypass</a>
        </div>
        
        <div class="section">
            <h2>🌐 Network & Server-Side Attacks</h2>
            <a href="/api/ssrf/fetch" class="test-link">SSRF Vulnerability</a>
            <a href="/api/scraper/url" class="test-link">SSRF via URL Scraping</a>
            <a href="/bypass/url/validation-bypass" class="test-link">URL Validation Bypass</a>
        </div>
        
        <div class="section">
            <h2>📁 File Operation Vulnerabilities</h2>
            <a href="/api/file/read?path=package.json" class="test-link">Path Traversal</a>
            <a href="/bypass/path/traversal-test" class="test-link">Path Traversal Bypass</a>
            <a href="/files/upload" class="test-link">File Upload</a>
            <a href="/bypass/file/type-bypass" class="test-link">File Type Validation Bypass</a>
        </div>
        
        <div class="section">
            <h2>🔐 Cryptographic Vulnerabilities</h2>
            <a href="/crypto/demo?action=hash" class="test-link">Weak Password Hashing</a>
            <a href="/crypto/demo?action=encrypt" class="test-link">Insecure Encryption</a>
            <a href="/crypto/demo?action=jwt" class="test-link">Vulnerable JWT</a>
        </div>
        
        <div class="section">
            <h2>🏢 Business Logic Vulnerabilities</h2>
            <a href="/advanced/payment/process" class="test-link">Race Condition in Payments</a>
            <a href="/advanced/pricing/calculate" class="test-link">Price Manipulation</a>
            <a href="/advanced/distributed/coordinate" class="test-link">Timing Attack Demo</a>
        </div>
        
        <div class="section">
            <h2>📊 Database Vulnerabilities</h2>
            <a href="/database/query" class="test-link">Advanced SQL Injection</a>
            <a href="/database/procedure" class="test-link">Stored Procedure Injection</a>
            <a href="/admin/config" class="test-link">Database Backup Exposure</a>
        </div>
        
        <div class="section">
            <h2>📚 Documentation</h2>
            <a href="/api/docs" class="test-link">API Documentation</a>
            <a href="/bypass/documentation" class="test-link">Bypass Techniques Documentation</a>
        </div>
    </body>
    </html>
    `);
});

// Logs viewer with IDOR vulnerability
app.get('/logs', (req, res) => {
    const userId = req.query.user_id || 1;
    
    // Vulnerable: No authorization check
    db.all(`SELECT * FROM audit_logs WHERE user_id = ${userId} ORDER BY timestamp DESC LIMIT 50`, (err, logs) => {
        if (err) {
            return res.status(500).send(`Error: ${err.message}`);
        }
        
        let html = `
        <h2>Audit Logs for User ${userId}</h2>
        <form>
            <input type="number" name="user_id" value="${userId}" placeholder="User ID">
            <button type="submit">View Logs</button>
        </form>
        <table border="1">
            <tr><th>ID</th><th>Action</th><th>Details</th><th>IP</th><th>Timestamp</th></tr>
        `;
        
        logs.forEach(log => {
            html += `<tr>
                <td>${log.id}</td>
                <td>${log.action}</td>
                <td>${log.details}</td>
                <td>${log.ip_address}</td>
                <td>${log.timestamp}</td>
            </tr>`;
        });
        
        html += '</table>';
        res.send(html);
    });
});

// Metrics endpoint with information disclosure
app.get('/metrics', (req, res) => {
    const stats = {
        database: {
            total_users: 0,
            total_documents: 0,
            total_logs: 0
        },
        system: {
            uptime: process.uptime(),
            memory: process.memoryUsage(),
            version: process.version,
            platform: process.platform
        },
        security: {
            failed_logins: Math.floor(Math.random() * 100),
            blocked_ips: ['192.168.1.100', '10.0.0.50'],
            suspicious_activity: true
        },
        environment: process.env // Vulnerable: Environment variable exposure
    };
    
    // Get database statistics
    db.get('SELECT COUNT(*) as count FROM users', (err, result) => {
        if (!err) stats.database.total_users = result.count;
        
        db.get('SELECT COUNT(*) as count FROM documents', (err, result) => {
            if (!err) stats.database.total_documents = result.count;
            
            db.get('SELECT COUNT(*) as count FROM audit_logs', (err, result) => {
                if (!err) stats.database.total_logs = result.count;
                res.json(stats);
            });
        });
    });
});

module.exports = app;

if (require.main === module) {
    app.listen(PORT, () => {
        console.log(`🔓 Advanced Enterprise-Level Vulnerable JavaScript Application`);
        console.log(`⚠️  This application contains intentional security vulnerabilities!`);
        console.log(`🌐 Server running at http://localhost:${PORT}`);
        console.log(`📚 Visit http://localhost:${PORT} for the vulnerability showcase`);
        console.log(`🧪 Test suite available at http://localhost:${PORT}/test`);
        console.log(`📖 API docs at http://localhost:${PORT}/api/docs`);
        console.log(`🔓 Bypass demos at http://localhost:${PORT}/bypass/documentation`);
    });
}