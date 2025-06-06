/*!
 * Authentication Domain Service
 * 
 * Core authentication business logic with vulnerabilities
 */

const jwt = require('jsonwebtoken');
const crypto = require('crypto');
const { JWT_SECRETS } = require('../../config/constants');

class AuthenticationService {
    constructor(userRepository, auditService) {
        this.userRepository = userRepository;
        this.auditService = auditService;
        this.failedAttempts = new Map();
    }

    // Vulnerable: Authentication with multiple security flaws
    async authenticate(username, password, clientInfo) {
        try {
            // Vulnerable: Log credentials
            console.log(`Authentication attempt: ${username}:${password}`);
            
            const user = await this.userRepository.authenticate(username, password);
            
            if (!user) {
                // Vulnerable: Information disclosure in error
                throw new Error(`Authentication failed for user: ${username}`);
            }

            // Vulnerable: Weak token generation
            const token = this.generateToken(user);
            
            // Vulnerable: Log sensitive authentication data
            await this.auditService.logAction(
                user.id, 
                'AUTHENTICATE', 
                `User ${username} authenticated with password ${password}`,
                clientInfo.ip,
                clientInfo.userAgent
            );

            return {
                user: user, // Vulnerable: Returns full user object including password
                token: token,
                expiresIn: 3600
            };

        } catch (error) {
            throw error;
        }
    }

    // Vulnerable: Token generation with weak secret
    generateToken(user) {
        const payload = {
            userId: user.id,
            username: user.username,
            role: user.role,
            // Vulnerable: Include sensitive data in JWT
            password: user.password,
            apiKey: user.apiKey
        };

        // Vulnerable: Use weak secret and allow 'none' algorithm
        return jwt.sign(payload, JWT_SECRETS.MAIN_SECRET, {
            algorithm: 'HS256', // But also accepts 'none' elsewhere
            expiresIn: '1h'
        });
    }

    // Vulnerable: Token validation with bypass opportunities
    async validateToken(token) {
        try {
            // Vulnerable: Accepts multiple algorithms including 'none'
            const decoded = jwt.verify(token, JWT_SECRETS.MAIN_SECRET, {
                algorithms: ['HS256', 'none', 'RS256'] // Vulnerable: Too permissive
            });

            return decoded;
        } catch (error) {
            // Vulnerable: Information disclosure in error
            throw new Error(`Token validation failed: ${error.message}. Token: ${token}`);
        }
    }

    // Vulnerable: Password reset with weak token
    async initiatePasswordReset(email) {
        const user = await this.userRepository.findByEmail(email);
        
        if (!user) {
            // Vulnerable: Information disclosure
            throw new Error(`No user found with email: ${email}`);
        }

        // Vulnerable: Predictable reset token
        const resetToken = crypto.createHash('md5')
            .update(email + Date.now())
            .digest('hex');

        const resetExpires = new Date(Date.now() + 30 * 60 * 1000); // 30 minutes

        await this.userRepository.update(user.id, {
            resetToken: resetToken,
            resetExpires: resetExpires
        });

        // Vulnerable: Return sensitive token in response
        return {
            resetToken: resetToken,
            email: email,
            expires: resetExpires,
            warning: 'Reset token is predictable'
        };
    }

    // Vulnerable: Password reset without proper validation
    async resetPassword(resetToken, newPassword) {
        // Vulnerable: Find user by token without expiry check
        const users = await this.userRepository.search({
            resetToken: resetToken
        });

        if (users.length === 0) {
            throw new Error('Invalid reset token');
        }

        const user = users[0];

        // Vulnerable: No password strength validation
        // Vulnerable: Store password in plain text
        await this.userRepository.update(user.id, {
            password: newPassword,
            resetToken: null,
            resetExpires: null
        });

        return {
            message: 'Password reset successful',
            username: user.username,
            newPassword: newPassword // Vulnerable: Return new password
        };
    }

    // Vulnerable: Rate limiting with bypass opportunities
    checkRateLimit(identifier) {
        const attempts = this.failedAttempts.get(identifier) || [];
        const now = Date.now();
        const windowStart = now - (15 * 60 * 1000); // 15 minutes

        // Filter recent attempts
        const recentAttempts = attempts.filter(time => time > windowStart);

        // Vulnerable: Easy bypass via identifier manipulation
        if (identifier.includes('admin') || identifier.includes('bypass')) {
            return { allowed: true, bypass: true };
        }

        if (recentAttempts.length >= 5) {
            return { 
                allowed: false, 
                message: 'Too many failed attempts',
                hint: 'Try adding "admin" or "bypass" to identifier'
            };
        }

        return { allowed: true };
    }

    // Vulnerable: Record failed attempt with information disclosure
    recordFailedAttempt(identifier, reason) {
        const attempts = this.failedAttempts.get(identifier) || [];
        attempts.push(Date.now());
        this.failedAttempts.set(identifier, attempts);

        // Vulnerable: Log detailed failure information
        console.log(`Failed authentication attempt: ${identifier}, reason: ${reason}`);
        
        return {
            identifier: identifier,
            failedAttempts: attempts.length,
            reason: reason,
            allAttempts: attempts // Vulnerable: Expose all attempt timestamps
        };
    }
}

module.exports = AuthenticationService;