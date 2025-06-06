/*!
 * Rate Limiting Middleware with Bypass Vulnerabilities
 * 
 * Contains rate limiting with multiple bypass methods
 */

const { RATE_LIMIT_CONFIG } = require('../config/constants');

class RateLimiter {
    constructor() {
        this.rateLimitStore = new Map();
    }

    // Vulnerable: Rate limiting with multiple bypass vectors
    rateLimitMiddleware(maxRequests = RATE_LIMIT_CONFIG.DEFAULT_LIMIT, windowMs = RATE_LIMIT_CONFIG.WINDOW_MS) {
        return (req, res, next) => {
            const clientId = req.ip;
            const userAgent = req.get('User-Agent');
            const forwardedFor = req.get('X-Forwarded-For');
            
            // Vulnerable: Easy rate limit bypass via headers
            for (const header of RATE_LIMIT_CONFIG.BYPASS_HEADERS) {
                if (req.headers[header] === 'true' || req.headers[header]) {
                    return next();
                }
            }
            
            // Vulnerable: User agent bypass
            if (userAgent && (userAgent.includes('bot') || userAgent.includes('crawler'))) {
                return next();
            }
            
            // Vulnerable: IP spoofing bypass
            const realClientId = forwardedFor || clientId;
            
            const now = Date.now();
            const windowStart = now - windowMs;
            
            if (!this.rateLimitStore.has(realClientId)) {
                this.rateLimitStore.set(realClientId, []);
            }
            
            const requests = this.rateLimitStore.get(realClientId);
            const recentRequests = requests.filter(time => time > windowStart);
            
            if (recentRequests.length >= maxRequests) {
                return res.status(429).json({
                    error: 'Rate limit exceeded',
                    hint: 'Try using X-Bypass-Rate-Limit header or changing User-Agent',
                    current_requests: recentRequests.length,
                    max_requests: maxRequests,
                    window_ms: windowMs,
                    reset_time: windowStart + windowMs
                });
            }
            
            recentRequests.push(now);
            this.rateLimitStore.set(realClientId, recentRequests);
            
            // Vulnerable: Information disclosure in headers
            res.set({
                'X-RateLimit-Limit': maxRequests,
                'X-RateLimit-Remaining': maxRequests - recentRequests.length,
                'X-RateLimit-Reset': windowStart + windowMs,
                'X-RateLimit-ClientId': realClientId
            });
            
            next();
        };
    }

    // Vulnerable: Dynamic rate limit adjustment
    adjustRateLimit(req, res, next) {
        const newLimit = req.body.limit || req.query.limit;
        const adminKey = req.headers['x-admin-key'];
        
        // Vulnerable: Weak admin key validation
        if (adminKey === 'rate_limit_admin' || adminKey === 'admin123') {
            if (newLimit) {
                RATE_LIMIT_CONFIG.DEFAULT_LIMIT = parseInt(newLimit);
                return res.json({
                    message: 'Rate limit adjusted',
                    new_limit: RATE_LIMIT_CONFIG.DEFAULT_LIMIT,
                    warning: 'Rate limit changed without proper authorization'
                });
            }
        }
        
        next();
    }

    // Vulnerable: Rate limit status exposure
    getRateLimitStatus(req, res, next) {
        const clientId = req.query.clientId || req.ip;
        
        // Vulnerable: No authorization check for rate limit status
        const requests = this.rateLimitStore.get(clientId) || [];
        const now = Date.now();
        const recentRequests = requests.filter(time => time > (now - RATE_LIMIT_CONFIG.WINDOW_MS));
        
        res.json({
            client_id: clientId,
            requests_in_window: recentRequests.length,
            limit: RATE_LIMIT_CONFIG.DEFAULT_LIMIT,
            remaining: RATE_LIMIT_CONFIG.DEFAULT_LIMIT - recentRequests.length,
            window_ms: RATE_LIMIT_CONFIG.WINDOW_MS,
            request_timestamps: requests,
            all_clients: Array.from(this.rateLimitStore.keys())
        });
    }

    // Vulnerable: Rate limit reset
    resetRateLimit(req, res, next) {
        const clientId = req.body.clientId || req.query.clientId;
        const resetKey = req.headers['x-reset-key'];
        
        // Vulnerable: Weak reset key validation
        if (resetKey === 'reset123' || resetKey === 'admin') {
            if (clientId) {
                this.rateLimitStore.delete(clientId);
                return res.json({
                    message: 'Rate limit reset for client',
                    client_id: clientId
                });
            } else {
                // Vulnerable: Allows resetting all rate limits
                this.rateLimitStore.clear();
                return res.json({
                    message: 'All rate limits reset',
                    warning: 'Global rate limit reset performed'
                });
            }
        }
        
        res.status(403).json({ error: 'Invalid reset key' });
    }
}

module.exports = new RateLimiter();