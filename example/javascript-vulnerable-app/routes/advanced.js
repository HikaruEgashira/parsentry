/*!
 * Advanced Vulnerability Routes
 * 
 * Contains sophisticated multi-layer vulnerability patterns
 * and complex exploitation scenarios
 */

const express = require('express');
const crypto = require('crypto');
const fs = require('fs');
const path = require('path');
const { execSync, spawn } = require('child_process');
const sqlite3 = require('sqlite3').verbose();
const jwt = require('jsonwebtoken');
const axios = require('axios');
const vm = require('vm');
const worker_threads = require('worker_threads');

const router = express.Router();
const db = new sqlite3.Database('vulnerable_app.db');

// Vulnerable: Sophisticated business logic bypass
class BusinessLogicBypass {
    constructor() {
        this.transactions = new Map();
        this.userBalances = new Map();
        this.adminTokens = new Set(['admin_token_2024', 'bypass_all_checks']);
    }

    // Vulnerable: Race condition in financial transaction
    async processPayment(req, res) {
        const { userId, amount, recipient, adminToken } = req.body;
        const transactionId = crypto.randomBytes(16).toString('hex');
        
        // Vulnerable: Admin bypass in business logic
        if (this.adminTokens.has(adminToken)) {
            return res.json({
                success: true,
                message: 'Admin bypass successful',
                transactionId,
                amount: amount * 1000 // Admin multiplier
            });
        }
        
        // Vulnerable: Race condition window
        const currentBalance = this.userBalances.get(userId) || 1000;
        
        // Simulate processing delay (race condition window)
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Vulnerable: TOCTOU (Time of Check Time of Use)
        if (currentBalance >= amount) {
            // Another request could modify balance here
            this.userBalances.set(userId, currentBalance - amount);
            
            // Vulnerable: Integer overflow not checked
            const recipientBalance = this.userBalances.get(recipient) || 0;
            this.userBalances.set(recipient, recipientBalance + parseInt(amount));
            
            res.json({
                success: true,
                transactionId,
                newBalance: this.userBalances.get(userId)
            });
        } else {
            res.status(400).json({ error: 'Insufficient funds' });
        }
    }

    // Vulnerable: Price manipulation through parameter pollution
    calculatePrice(req, res) {
        const { items, discount, coupon, vip } = req.body;
        
        let totalPrice = 0;
        
        // Vulnerable: Array manipulation
        if (Array.isArray(items)) {
            items.forEach(item => {
                // Vulnerable: Type confusion
                totalPrice += parseFloat(item.price) * parseInt(item.quantity);
            });
        }
        
        // Vulnerable: Discount stacking without limits
        if (discount) {
            totalPrice *= (1 - parseFloat(discount));
        }
        
        if (coupon) {
            // Vulnerable: Coupon bypass via negative values
            totalPrice -= parseFloat(coupon);
        }
        
        // Vulnerable: VIP bypass via truthy values
        if (vip) {
            totalPrice *= 0.5; // 50% VIP discount
        }
        
        res.json({
            originalPrice: totalPrice / (vip ? 0.5 : 1),
            finalPrice: Math.max(0, totalPrice),
            savings: totalPrice < 0 ? Math.abs(totalPrice) : 0
        });
    }
}

const businessLogic = new BusinessLogicBypass();

// Vulnerable: Complex multi-step authentication bypass
router.post('/auth/multi-step', async (req, res) => {
    const { step, username, password, mfaCode, recoveryCode, adminOverride } = req.body;
    
    // Vulnerable: Step manipulation
    switch (parseInt(step)) {
        case 1:
            // Step 1: Username/password
            if (username === 'admin' && password === 'admin123') {
                res.json({
                    step: 2,
                    tempToken: jwt.sign({ username, step: 2 }, 'temp_secret', { expiresIn: '5m' })
                });
            } else {
                res.status(401).json({ error: 'Invalid credentials' });
            }
            break;
            
        case 2:
            // Step 2: MFA
            // Vulnerable: MFA bypass via overflow
            if (mfaCode === '123456' || parseInt(mfaCode) > 999999) {
                res.json({
                    step: 3,
                    tempToken: jwt.sign({ username, step: 3 }, 'temp_secret', { expiresIn: '5m' })
                });
            } else {
                res.status(401).json({ error: 'Invalid MFA code' });
            }
            break;
            
        case 3:
            // Step 3: Final verification
            // Vulnerable: Recovery code bypass
            if (recoveryCode === 'RECOVERY2024' || adminOverride === 'true') {
                const finalToken = jwt.sign(
                    { username, role: 'admin' }, 
                    'super_secret_js_key_123', 
                    { expiresIn: '1h' }
                );
                res.json({
                    success: true,
                    token: finalToken,
                    message: 'Multi-step authentication completed'
                });
            } else {
                res.status(401).json({ error: 'Invalid recovery code' });
            }
            break;
            
        default:
            // Vulnerable: Direct step bypass
            if (adminOverride === 'bypass_all_steps') {
                const bypassToken = jwt.sign(
                    { username: 'admin', role: 'admin' }, 
                    'super_secret_js_key_123', 
                    { expiresIn: '1h' }
                );
                res.json({
                    success: true,
                    token: bypassToken,
                    message: 'All steps bypassed'
                });
            } else {
                res.status(400).json({ error: 'Invalid step' });
            }
    }
});

// Vulnerable: Advanced file operations with multiple vulnerabilities
router.post('/file/advanced-ops', (req, res) => {
    const { operation, source, destination, content, encoding } = req.body;
    
    try {
        switch (operation) {
            case 'copy':
                // Vulnerable: Path traversal + symlink attacks
                if (fs.existsSync(source)) {
                    const data = fs.readFileSync(source);
                    fs.writeFileSync(destination, data);
                    res.json({ message: 'File copied successfully' });
                } else {
                    res.status(404).json({ error: 'Source file not found' });
                }
                break;
                
            case 'write':
                // Vulnerable: Arbitrary file write
                const decodedContent = encoding === 'base64' 
                    ? Buffer.from(content, 'base64').toString()
                    : content;
                fs.writeFileSync(destination, decodedContent);
                res.json({ message: 'File written successfully' });
                break;
                
            case 'exec':
                // Vulnerable: Command injection via file operations
                const output = execSync(`cat ${source} | head -10`, { encoding: 'utf8' });
                res.json({ output });
                break;
                
            case 'compress':
                // Vulnerable: Zip bomb creation
                const command = `tar -czf ${destination} ${source}`;
                execSync(command);
                res.json({ message: 'File compressed successfully' });
                break;
                
            default:
                res.status(400).json({ error: 'Invalid operation' });
        }
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            operation,
            source,
            destination
        });
    }
});

// Vulnerable: Code execution via VM sandbox escape
router.post('/vm/execute', (req, res) => {
    const { code, timeout = 5000, context } = req.body;
    
    try {
        // Vulnerable: VM sandbox that can be escaped
        const vmContext = {
            result: null,
            console: {
                log: (...args) => console.log('[VM]', ...args)
            },
            Buffer,
            // Vulnerable: Exposing require indirectly
            global: global,
            process: {
                version: process.version,
                platform: process.platform
            },
            ...context
        };
        
        // Vulnerable: Timeout can be bypassed
        const script = new vm.Script(`
            try {
                result = (function() {
                    ${code}
                })();
            } catch (e) {
                result = { error: e.message };
            }
        `);
        
        script.runInNewContext(vmContext, { timeout });
        
        res.json({
            result: vmContext.result,
            context: Object.keys(vmContext)
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            hint: 'Try escaping the sandbox with constructor.constructor'
        });
    }
});

// Vulnerable: Complex LDAP injection
router.post('/ldap/search', (req, res) => {
    const { username, attribute, filter } = req.body;
    
    // Vulnerable: LDAP injection in multiple parameters
    let ldapQuery = `(&(objectClass=person)(uid=${username})`;
    
    if (attribute) {
        ldapQuery += `(${attribute}=*)`;
    }
    
    if (filter) {
        ldapQuery += filter;
    }
    
    ldapQuery += ')';
    
    // Simulate LDAP response
    const mockUsers = [
        { uid: 'admin', cn: 'Administrator', mail: 'admin@company.com' },
        { uid: 'user1', cn: 'Regular User', mail: 'user1@company.com' }
    ];
    
    res.json({
        query: ldapQuery,
        results: mockUsers,
        hint: 'Try LDAP injection: *)(uid=*'
    });
});

// Vulnerable: Business logic routes
router.post('/payment/process', businessLogic.processPayment.bind(businessLogic));
router.post('/pricing/calculate', businessLogic.calculatePrice.bind(businessLogic));

// Vulnerable: GraphQL-like query injection
router.post('/query/graph', (req, res) => {
    const { query, variables } = req.body;
    
    // Vulnerable: Query injection in graph-like syntax
    try {
        // Simulate query parsing
        const queryPattern = /\{([^}]+)\}/g;
        const matches = [];
        let match;
        
        while ((match = queryPattern.exec(query)) !== null) {
            matches.push(match[1]);
        }
        
        // Vulnerable: Code injection in query resolution
        const resolveQuery = (field) => {
            // Dangerous: eval in query resolution
            if (field.includes('()')) {
                return eval(field);
            }
            return `Resolved: ${field}`;
        };
        
        const results = matches.map(resolveQuery);
        
        res.json({
            query,
            variables,
            results
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            hint: 'Try injecting code: {constructor.constructor("return process")()}'
        });
    }
});

// Vulnerable: Distributed system simulation with timing attacks
router.post('/distributed/coordinate', async (req, res) => {
    const { nodes, operation, secret } = req.body;
    
    const startTime = Date.now();
    
    // Vulnerable: Timing attack in secret validation
    const expectedSecret = 'distributed_secret_2024';
    
    for (let i = 0; i < secret.length && i < expectedSecret.length; i++) {
        // Vulnerable: Character-by-character timing
        if (secret[i] !== expectedSecret[i]) {
            break;
        }
        // Simulate processing delay proportional to correct characters
        await new Promise(resolve => setTimeout(resolve, 10));
    }
    
    const endTime = Date.now();
    const processingTime = endTime - startTime;
    
    if (secret === expectedSecret) {
        res.json({
            success: true,
            coordinatedNodes: nodes?.length || 0,
            operation,
            processingTime
        });
    } else {
        res.status(401).json({
            error: 'Invalid secret',
            processingTime,
            hint: 'Use timing attacks to discover the secret character by character'
        });
    }
});

// Vulnerable: Cache poisoning
const cache = new Map();

router.get('/cache/:key', (req, res) => {
    const { key } = req.params;
    const { host } = req.headers;
    
    // Vulnerable: Cache key poisoning via headers
    const cacheKey = `${host}:${key}`;
    
    if (cache.has(cacheKey)) {
        res.json({
            cached: true,
            data: cache.get(cacheKey),
            cacheKey
        });
    } else {
        const data = `Data for ${key} from ${host}`;
        cache.set(cacheKey, data);
        
        res.json({
            cached: false,
            data,
            cacheKey
        });
    }
});

// Vulnerable: HTTP request smuggling simulation
router.all('/smuggling/test', (req, res) => {
    const contentLength = req.headers['content-length'];
    const transferEncoding = req.headers['transfer-encoding'];
    
    // Vulnerable: Conflicting Content-Length and Transfer-Encoding
    if (transferEncoding && contentLength) {
        res.json({
            warning: 'Conflicting headers detected',
            contentLength,
            transferEncoding,
            hint: 'This could enable request smuggling attacks'
        });
    } else {
        res.json({
            method: req.method,
            headers: req.headers,
            body: req.body
        });
    }
});

module.exports = router;