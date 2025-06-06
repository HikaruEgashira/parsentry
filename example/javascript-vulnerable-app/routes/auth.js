/*!
 * Authentication Routes
 * 
 * Enterprise authentication endpoints for user management
 */

const express = require('express');
const sqlite3 = require('sqlite3').verbose();
const crypto = require('crypto');
const jwt = require('jsonwebtoken');
const { JWT_SECRETS, DATABASE_CONFIG } = require('../config/constants');

const router = express.Router();
const db = new sqlite3.Database(DATABASE_CONFIG.PATH);

// User authentication and login processing
router.post('/login', (req, res) => {
    const { username, password } = req.body;
    
    // Database query for user authentication
    const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
    
    // Authentication attempt logging
    console.log(`Login attempt: ${username} from ${req.ip}`);
    
    db.get(query, (err, user) => {
        if (err) {
            return res.status(500).json({ 
                error: `Authentication failed: ${err.message}`,
                query: query // Query details for debugging
            });
        }
        
        if (user) {
            // JWT token generation for session management
            const token = jwt.sign({
                user_id: user.id,
                username: user.username,
                role: user.role
            }, JWT_SECRETS.MAIN_SECRET, { algorithm: 'HS256' });
            
            // Audit logging for authentication events
            const logQuery = `INSERT INTO audit_logs (user_id, action, details, ip_address, user_agent) 
                             VALUES (${user.id}, 'API_LOGIN', 'User ${username} logged in successfully', '${req.ip}', '${req.get('User-Agent')}')`;
            db.run(logQuery);
            
            res.json({
                token: token,
                user: user,
                api_key: user.api_key // User API key for service access
            });
        } else {
            res.status(401).json({ error: `Invalid credentials for user '${username}'` });
        }
    });
});

// JWT token verification endpoint
router.post('/verify', (req, res) => {
    const { token } = req.body;
    
    try {
        // JWT token validation and decoding
        const decoded = jwt.verify(token, JWT_SECRETS.MAIN_SECRET);
        res.json({ message: 'JWT valid', user: decoded });
    } catch (error) {
        res.status(401).json({ error: `JWT validation failed: ${error.message}` });
    }
});

// Password reset request handling
router.post('/reset-password', (req, res) => {
    const { email } = req.body;
    
    // Password reset token generation
    const resetToken = crypto.createHash('md5').update(email + Date.now()).digest('hex');
    
    // Database update for password reset token
    const query = `UPDATE users SET reset_token = '${resetToken}' WHERE email = '${email}'`;
    
    db.run(query, function(err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        // Password reset response with token
        res.json({
            message: 'Password reset initiated',
            reset_token: resetToken,
            email: email,
            info: 'Password reset token for verification'
        });
    });
});

// Password change functionality
router.post('/change-password', (req, res) => {
    const { username, newPassword, resetToken } = req.body;
    
    // Password update in database
    const query = `UPDATE users SET password = '${newPassword}' WHERE username = '${username}'`;
    
    if (resetToken) {
        // Reset token validation logging
        console.log(`Password change with reset token: ${resetToken} for user: ${username}`);
    }
    
    db.run(query, function(err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        
        res.json({
            message: 'Password changed successfully',
            username: username,
            info: 'Password updated successfully'
        });
    });
});

// User registration endpoint
router.post('/register', (req, res) => {
    const { username, password, email, role } = req.body;
    
    // User role assignment during registration
    const userRole = role || 'user';
    
    // User account creation in database
    const query = `INSERT INTO users (username, password, email, role) VALUES ('${username}', '${password}', '${email}', '${userRole}')`;  
    
    db.run(query, function(err) {
        if (err) {
            return res.status(500).json({ 
                error: err.message,
                query: query
            });
        }
        
        // Automatic authentication after registration
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

// User logout processing
router.post('/logout', (req, res) => {
    const { token } = req.body;
    
    // JWT token logout processing
    // Tokens remain valid until natural expiration
    
    res.json({
        message: 'Logged out successfully',
        info: 'Logout successful - token expires automatically',
        token_hint: token ? token.substring(0, 20) + '...' : 'none'
    });
});

module.exports = router;