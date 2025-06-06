/*!
 * Authentication Routes with Multiple Vulnerabilities
 * 
 * Contains login, logout, and authentication-related endpoints
 */

const express = require('express');
const sqlite3 = require('sqlite3').verbose();
const crypto = require('crypto');
const jwt = require('jsonwebtoken');
const { JWT_SECRETS, DATABASE_CONFIG } = require('../config/constants');

const router = express.Router();
const db = new sqlite3.Database(DATABASE_CONFIG.PATH);

// Vulnerable: Login with SQL injection and credential exposure
router.post('/login', (req, res) => {
    const { username, password } = req.body;
    
    // Vulnerable: SQL injection in authentication
    const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
    
    // Vulnerable: Log sensitive information
    console.log(`Login attempt: ${username}:${password} from ${req.ip}`);
    
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
            }, JWT_SECRETS.MAIN_SECRET, { algorithm: 'HS256' });
            
            // Vulnerable: Log sensitive information in audit
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

// Vulnerable: JWT verification bypass
router.post('/verify', (req, res) => {
    const { token } = req.body;
    
    try {
        // Vulnerable: JWT verification without proper validation
        const decoded = jwt.verify(token, JWT_SECRETS.MAIN_SECRET);
        res.json({ message: 'JWT valid', user: decoded });
    } catch (error) {
        res.status(401).json({ error: `JWT validation failed: ${error.message}` });
    }
});

// Vulnerable: Password reset with weak token generation
router.post('/reset-password', (req, res) => {
    const { email } = req.body;
    
    // Vulnerable: Predictable reset token
    const resetToken = crypto.createHash('md5').update(email + Date.now()).digest('hex');
    
    // Vulnerable: No rate limiting on password reset
    const query = `UPDATE users SET reset_token = '${resetToken}' WHERE email = '${email}'`;
    
    db.run(query, function(err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        // Vulnerable: Returns reset token in response
        res.json({
            message: 'Password reset initiated',
            reset_token: resetToken,
            email: email,
            hint: 'Reset token is predictable MD5 hash'
        });
    });
});

// Vulnerable: Password change without current password verification
router.post('/change-password', (req, res) => {
    const { username, newPassword, resetToken } = req.body;
    
    // Vulnerable: No verification of current password
    const query = `UPDATE users SET password = '${newPassword}' WHERE username = '${username}'`;
    
    if (resetToken) {
        // Vulnerable: Weak reset token validation
        console.log(`Password change with reset token: ${resetToken} for user: ${username}`);
    }
    
    db.run(query, function(err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        res.json({
            message: 'Password changed successfully',
            username: username,
            warning: 'Password changed without current password verification'
        });
    });
});

// Vulnerable: User registration with weak validation
router.post('/register', (req, res) => {
    const { username, password, email, role } = req.body;
    
    // Vulnerable: Allows setting admin role during registration
    const userRole = role || 'user';
    
    // Vulnerable: No password strength validation
    const query = `INSERT INTO users (username, password, email, role) VALUES ('${username}', '${password}', '${email}', '${userRole}')`;
    
    db.run(query, function(err) {
        if (err) {
            return res.status(500).json({ 
                error: err.message,
                query: query
            });
        }
        
        // Vulnerable: Auto-login after registration
        const token = jwt.sign({
            user_id: this.lastID,
            username: username,
            role: userRole
        }, JWT_SECRETS.MAIN_SECRET);
        
        res.json({
            message: 'User registered successfully',
            user_id: this.lastID,
            token: token,
            role: userRole
        });
    });
});

// Vulnerable: Logout that doesn't invalidate tokens
router.post('/logout', (req, res) => {
    const { token } = req.body;
    
    // Vulnerable: Doesn't actually invalidate the JWT token
    // JWT tokens remain valid until expiration
    
    res.json({
        message: 'Logged out successfully',
        warning: 'JWT token not invalidated - still valid until expiration',
        token_hint: token ? token.substring(0, 20) + '...' : 'none'
    });
});

module.exports = router;