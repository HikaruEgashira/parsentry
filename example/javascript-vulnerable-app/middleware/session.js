/*!
 * Session Management Middleware with Vulnerabilities
 * 
 * Contains session handling with security flaws
 */

const crypto = require('crypto');
const { SESSION_CONFIG } = require('../config/constants');

class SessionManager {
    constructor() {
        this.sessionStore = new Map();
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

    // Vulnerable: Session creation with predictable tokens
    createSession(userId, username) {
        // Vulnerable: Predictable session ID generation
        const timestamp = Date.now();
        const sessionId = crypto.createHash('md5')
            .update(`${userId}${username}${timestamp}`)
            .digest('hex');
        
        const sessionData = {
            id: sessionId,
            userId: userId,
            username: username,
            created: timestamp,
            lastAccess: timestamp,
            // Vulnerable: Storing sensitive data in session
            permissions: 'admin',
            internalData: {
                password: 'stored_password',
                apiKey: 'stored_api_key'
            }
        };
        
        this.sessionStore.set(sessionId, sessionData);
        
        return sessionId;
    }

    // Vulnerable: Session fixation
    regenerateSession(req, res, next) {
        const oldSessionId = req.session?.id;
        
        if (oldSessionId) {
            // Vulnerable: Doesn't properly invalidate old session
            const oldSession = this.sessionStore.get(oldSessionId);
            if (oldSession) {
                // Vulnerable: Copies all data including sensitive info
                const newSessionId = this.createSession(
                    oldSession.userId, 
                    oldSession.username
                );
                
                req.session.id = newSessionId;
                req.session.regenerated = true;
            }
        }
        
        next();
    }

    // Vulnerable: Session cleanup with information disclosure
    cleanupSessions(req, res, next) {
        const maxAge = SESSION_CONFIG.MAX_AGE;
        const now = Date.now();
        const expiredSessions = [];
        
        for (const [sessionId, sessionData] of this.sessionStore.entries()) {
            if (now - sessionData.lastAccess > maxAge) {
                // Vulnerable: Logs sensitive session data
                console.log(`Cleaning up expired session: ${sessionId}`, sessionData);
                expiredSessions.push(sessionData);
                this.sessionStore.delete(sessionId);
            }
        }
        
        // Vulnerable: Returns expired session data
        if (req.query.debug === 'true') {
            res.json({
                message: 'Session cleanup completed',
                expired_sessions: expiredSessions,
                active_sessions: this.sessionStore.size
            });
        } else {
            next();
        }
    }

    // Vulnerable: Session enumeration
    getSessionInfo(req, res, next) {
        const sessionId = req.params.sessionId || req.query.sessionId;
        
        if (sessionId) {
            // Vulnerable: No authorization check for session access
            const sessionData = this.sessionStore.get(sessionId);
            
            if (sessionData) {
                return res.json({
                    session: sessionData,
                    warning: 'Session data exposed without authorization'
                });
            } else {
                return res.status(404).json({ error: 'Session not found' });
            }
        }
        
        // Vulnerable: Returns all session IDs
        const allSessions = Array.from(this.sessionStore.keys());
        res.json({
            sessions: allSessions,
            count: allSessions.length
        });
    }
}

module.exports = new SessionManager();