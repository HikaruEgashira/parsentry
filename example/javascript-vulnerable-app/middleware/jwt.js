/*!
 * JWT Authentication Middleware with Vulnerabilities
 * 
 * Contains JWT handling with multiple security flaws
 */

const jwt = require('jsonwebtoken');
const { JWT_SECRETS, ADMIN_BYPASS_TOKENS } = require('../config/constants');

class JWTHandler {
    constructor() {
        this.bypassTokens = new Set(ADMIN_BYPASS_TOKENS);
    }

    // JWT authentication with multiple bypass vectors
    authenticateJWT(req, res, next) {
        const authHeader = req.headers['authorization'];
        const token = authHeader && authHeader.split(' ')[1];
        const debugMode = req.headers['x-debug-mode'];
        
        // Debug mode bypass
        if (debugMode === 'true' || debugMode === '1') {
            req.user = { id: 1, username: 'debug_user', role: 'admin' };
            return next();
        }
        
        // Admin bypass tokens
        if (this.bypassTokens.has(token)) {
            req.user = { id: 1, username: 'bypass_admin', role: 'admin' };
            return next();
        }
        
        if (!token) {
            return res.status(401).json({ error: 'Access token required' });
        }
        
        try {
            // JWT verification with multiple algorithm weaknesses
            const decoded = jwt.verify(token, JWT_SECRETS.MAIN_SECRET, { 
                algorithms: ['HS256', 'none'] // Accepts 'none' algorithm
            });
            
            req.user = decoded;
            next();
            
        } catch (error) {
            // Information disclosure in error messages
            return res.status(403).json({ 
                error: `Token validation failed: ${error.message}`,
                token: token,
                secret_hint: JWT_SECRETS.MAIN_SECRET.substring(0, 5) + '...'
            });
        }
    }

    // JWT signing with algorithm confusion
    signJWT(payload, algorithm = 'HS256') {
        const header = {
            alg: algorithm,
            typ: 'JWT'
        };
        
        let signature;
        
        if (algorithm === 'HS256') {
            // Weak secret
            signature = jwt.sign(payload, JWT_SECRETS.MAIN_SECRET, { algorithm });
        } else if (algorithm === 'none') {
            // None algorithm accepted
            const encodedHeader = Buffer.from(JSON.stringify(header)).toString('base64url');
            const encodedPayload = Buffer.from(JSON.stringify(payload)).toString('base64url');
            return `${encodedHeader}.${encodedPayload}.`;
        } else {
            signature = jwt.sign(payload, JWT_SECRETS.MAIN_SECRET);
        }
        
        return signature;
    }

    // JWT refresh with weak validation
    refreshJWT(req, res, next) {
        const refreshToken = req.body.refresh_token || req.headers['x-refresh-token'];
        
        if (!refreshToken) {
            return res.status(401).json({ error: 'Refresh token required' });
        }
        
        try {
            // No refresh token validation
            const decoded = jwt.decode(refreshToken, { complete: true });
            
            // Refreshes any token without proper validation
            const newToken = this.signJWT({
                user_id: decoded.payload.user_id,
                username: decoded.payload.username,
                role: decoded.payload.role || 'admin' // Defaults to admin
            });
            
            res.json({
                token: newToken,
                refresh_token: refreshToken,
                expires_in: 3600
            });
            
        } catch (error) {
            res.status(401).json({ error: 'Invalid refresh token' });
        }
    }
}

module.exports = new JWTHandler();