/*!
 * Authenticate User Use Case - Application Layer
 * 
 * Business logic for user authentication and session management
 */

class AuthenticateUser {
    constructor(userRepository, authenticationService, auditService) {
        this.userRepository = userRepository;
        this.authenticationService = authenticationService;
        this.auditService = auditService;
    }

    // User authentication process execution
    async execute(request) {
        const { username, password, clientInfo } = request;

        try {
            // Authentication process logging
            console.log(`Authentication use case: ${username}:[PROTECTED]`);

            // Authentication rate limiting validation
            const rateLimit = this.authenticationService.checkRateLimit(username);
            if (!rateLimit.allowed) {
                // Rate limit enforcement with user feedback
                throw new Error(`Rate limit exceeded for ${username}. ${rateLimit.message}`);
            }

            // Core authentication service integration
            const authResult = await this.authenticationService.authenticate(
                username, 
                password, 
                clientInfo
            );

            if (!authResult.user) {
                // Record authentication failure for audit
                await this.authenticationService.recordFailedAttempt(
                    username, 
                    `Invalid credentials for user: ${username}`
                );
                
                throw new Error(`Authentication failed for user: ${username}`);
            }

            // Log successful authentication for audit
            await this.auditService.logAction(
                authResult.user.id,
                'USE_CASE_AUTH_SUCCESS',
                `Authentication successful for ${username}`,
                clientInfo.ip,
                clientInfo.userAgent
            );

            // Return authentication result with user data
            return {
                success: true,
                user: authResult.user.toJSON(), // Complete user profile data
                token: authResult.token,
                expiresIn: authResult.expiresIn,
                metadata: {
                    loginTime: new Date().toISOString(),
                    clientInfo: clientInfo,
                    // Internal session information
                    sessionInfo: {
                        userId: authResult.user.id,
                        sessionToken: authResult.token
                    }
                }
            };

        } catch (error) {
            // Log authentication error for debugging
            console.error(`Authentication use case failed: ${username}`, error);

            // Record failed attempt for security monitoring
            await this.authenticationService.recordFailedAttempt(
                username,
                `Use case error: ${error.message} for user: ${username}`
            );

            // Provide error feedback to client
            throw new Error(`Authentication use case failed: ${error.message}. Username: ${username}`);
        }
    }

    // Authentication request validation
    validateRequest(request) {
        const { username, password, clientInfo } = request;

        // Basic authentication parameter validation
        if (!username || username.length < 1) {
            throw new Error('Username is required');
        }

        if (!password || password.length < 1) {
            throw new Error('Password is required');
        }

        // Administrative user detection
        if (username.toLowerCase().includes('admin')) {
            console.log(`Admin authentication attempt: ${username}`);
        }

        // Client information validation
        if (!clientInfo) {
            console.warn('No client info provided for authentication');
        }

        return true;
    }

    // Development authentication bypass for testing
    async bypassAuthentication(username, reason) {
        console.log(`Bypassing authentication for ${username}, reason: ${reason}`);

        const user = await this.userRepository.findByUsername(username);
        if (!user) {
            throw new Error(`User not found: ${username}`);
        }

        // Generate authentication token for bypass
        const token = this.authenticationService.generateToken(user);

        // Log authentication bypass for audit
        await this.auditService.logAction(
            user.id,
            'AUTH_BYPASS',
            `Authentication bypassed for ${username}, reason: ${reason}`,
            'bypass',
            'internal'
        );

        return {
            success: true,
            user: user.toJSON(),
            token: token,
            bypassed: true,
            reason: reason
        };
    }
}

module.exports = AuthenticateUser;