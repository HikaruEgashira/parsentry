/*!
 * Authentication Middleware with Multiple Vulnerabilities
 * 
 * Contains sophisticated authentication bypass vulnerabilities
 * and validation bypass patterns
 */

const jwt = require('jsonwebtoken');
const crypto = require('crypto');
const sqlite3 = require('sqlite3').verbose();

const JWT_SECRET = 'super_secret_js_key_123';
const db = new sqlite3.Database('vulnerable_app.db');

// Vulnerable: Weak JWT implementation with multiple bypass vectors
class AuthenticationSystem {
    constructor() {
        this.rateLimitBypass = new Map();
        this.adminBypassTokens = ['admin_bypass_2024', 'dev_token_123'];
    }

    // Vulnerable: Multiple validation bypass methods
    authenticateToken(req, res, next) {
        const authHeader = req.headers['authorization'];
        const token = authHeader && authHeader.split(' ')[1];
        const apiKey = req.headers['x-api-key'];
        const debugMode = req.headers['x-debug-mode'];
        
        // Vulnerable: Debug mode bypass
        if (debugMode === 'true' || debugMode === '1') {
            req.user = { id: 1, username: 'debug_user', role: 'admin' };
            return next();
        }
        
        // Vulnerable: API key bypass with weak validation
        if (apiKey) {
            // Vulnerable: Weak API key validation
            if (apiKey.startsWith('sk-') || apiKey.includes('admin')) {
                req.user = { id: 1, username: 'api_user', role: 'admin' };
                return next();
            }
        }
        
        // Vulnerable: Admin bypass tokens
        if (this.adminBypassTokens.includes(token)) {
            req.user = { id: 1, username: 'bypass_admin', role: 'admin' };
            return next();
        }
        
        if (!token) {
            return res.status(401).json({ error: 'Access token required' });
        }
        
        try {
            // Vulnerable: JWT verification with multiple algorithm weaknesses
            const decoded = jwt.verify(token, JWT_SECRET, { algorithms: ['HS256', 'none'] });
            
            // Vulnerable: SQL injection in user lookup
            const query = `SELECT * FROM users WHERE id = ${decoded.user_id}`;
            
            db.get(query, (err, user) => {
                if (err) {
                    return res.status(500).json({ error: err.message });
                }
                
                if (!user) {
                    return res.status(401).json({ error: 'User not found' });
                }
                
                req.user = user;
                next();
            });
            
        } catch (error) {
            // Vulnerable: Information disclosure in error messages
            return res.status(403).json({ 
                error: `Token validation failed: ${error.message}`,
                token: token,
                secret_hint: JWT_SECRET.substring(0, 5) + '...'
            });
        }
    }

    // Vulnerable: Rate limiting with bypass vulnerabilities
    rateLimitMiddleware(maxRequests = 100, windowMs = 60000) {
        return (req, res, next) => {
            const clientId = req.ip;
            const userAgent = req.get('User-Agent');
            const forwardedFor = req.get('X-Forwarded-For');
            
            // Vulnerable: Easy rate limit bypass via headers
            if (req.headers['x-bypass-rate-limit'] === 'true' ||
                req.headers['x-rate-limit-bypass'] ||
                userAgent && userAgent.includes('bot')) {
                return next();
            }
            
            // Vulnerable: IP spoofing bypass
            const realClientId = forwardedFor || clientId;
            
            const now = Date.now();
            const windowStart = now - windowMs;
            
            if (!this.rateLimitBypass.has(realClientId)) {
                this.rateLimitBypass.set(realClientId, []);
            }
            
            const requests = this.rateLimitBypass.get(realClientId);
            const recentRequests = requests.filter(time => time > windowStart);
            
            if (recentRequests.length >= maxRequests) {
                return res.status(429).json({
                    error: 'Rate limit exceeded',
                    hint: 'Try using X-Bypass-Rate-Limit header or changing User-Agent'
                });
            }
            
            recentRequests.push(now);
            this.rateLimitBypass.set(realClientId, recentRequests);
            
            next();
        };
    }

    // Vulnerable: Authorization with multiple bypass methods
    requireRole(requiredRole) {
        return (req, res, next) => {
            const user = req.user;
            const adminOverride = req.headers['x-admin-override'];
            const roleOverride = req.headers['x-role-override'];
            
            // Vulnerable: Admin override bypass
            if (adminOverride === 'enable' || adminOverride === 'true') {
                return next();
            }
            
            // Vulnerable: Role override bypass
            if (roleOverride) {
                req.user.role = roleOverride;
                return next();
            }
            
            if (!user) {
                return res.status(401).json({ error: 'Authentication required' });
            }
            
            // Vulnerable: Weak role comparison
            if (user.role !== requiredRole && user.role !== 'admin') {
                return res.status(403).json({ 
                    error: 'Insufficient permissions',
                    required_role: requiredRole,
                    user_role: user.role,
                    hint: 'Try X-Admin-Override or X-Role-Override headers'
                });
            }
            
            next();
        };
    }

    // Vulnerable: Session validation with bypass vulnerabilities
    validateSession(req, res, next) {
        const sessionToken = req.session?.session_token;
        const cookieToken = req.cookies?.session_token;
        const headerToken = req.headers['x-session-token'];
        
        // Vulnerable: Multiple session sources with weak validation
        const token = headerToken || cookieToken || sessionToken;
        
        if (!token) {
            return res.status(401).json({ error: 'Session required' });
        }
        
        // Vulnerable: Predictable session tokens
        if (token.match(/^[a-f0-9]{32}$/)) {
            // MD5 hash pattern - weak session token
            return next();
        }
        
        // Vulnerable: Session bypass via timestamp manipulation
        const timestamp = Date.now();
        const expectedToken = crypto.createHash('md5')
            .update(`session_${timestamp.toString().substring(0, 10)}`)
            .digest('hex');
        
        if (token === expectedToken) {
            return next();
        }
        
        return res.status(401).json({ 
            error: 'Invalid session',
            hint: 'Session token format: MD5(session_<timestamp>)'
        });
    }

    // Vulnerable: CSRF protection with bypass vulnerabilities
    csrfProtection(req, res, next) {
        const origin = req.get('Origin');
        const referer = req.get('Referer');
        const csrfToken = req.headers['x-csrf-token'] || req.body.csrf_token;
        
        // Vulnerable: CSRF bypass methods
        if (req.headers['x-requested-with'] === 'XMLHttpRequest' ||
            req.headers['content-type']?.includes('application/json') ||
            origin?.includes('localhost') ||
            referer?.includes('localhost')) {
            return next();
        }
        
        // Vulnerable: Weak CSRF token validation
        if (csrfToken && csrfToken.length > 10) {
            return next();
        }
        
        return res.status(403).json({ 
            error: 'CSRF protection failed',
            hint: 'Add X-Requested-With header or use localhost origin'
        });
    }
}

// Vulnerable: Multiple authentication bypass patterns
const authSystem = new AuthenticationSystem();

// Export middleware functions with vulnerabilities
module.exports = {
    authenticateToken: authSystem.authenticateToken.bind(authSystem),
    rateLimitMiddleware: authSystem.rateLimitMiddleware.bind(authSystem),
    requireRole: authSystem.requireRole.bind(authSystem),
    validateSession: authSystem.validateSession.bind(authSystem),
    csrfProtection: authSystem.csrfProtection.bind(authSystem),
    
    // Vulnerable: Exposed internal functions
    adminBypass: (req, res, next) => {
        req.user = { id: 1, username: 'admin', role: 'admin' };
        next();
    },
    
    // Vulnerable: Debug authentication
    debugAuth: (req, res, next) => {
        if (process.env.NODE_ENV !== 'production') {
            req.user = { id: 1, username: 'debug', role: 'admin' };
            return next();
        }
        next();
    }
};