/*!
 * Authenticate User Use Case - Application Layer
 * 
 * Business logic for user authentication with security vulnerabilities
 */

class AuthenticateUser {
    constructor(userRepository, authenticationService, auditService) {
        this.userRepository = userRepository;
        this.authenticationService = authenticationService;
        this.auditService = auditService;
    }

    // Vulnerable: Authentication use case with multiple security flaws
    async execute(request) {
        const { username, password, clientInfo } = request;

        try {
            // Vulnerable: Log credentials at application layer
            console.log(`Authentication use case: ${username}:${password}`);

            // Vulnerable: Rate limiting with bypass opportunities
            const rateLimit = this.authenticationService.checkRateLimit(username);
            if (!rateLimit.allowed) {
                // Vulnerable: Information disclosure in rate limit response
                throw new Error(`Rate limit exceeded for ${username}. ${rateLimit.message}`);
            }

            // Vulnerable: Authentication without proper validation
            const authResult = await this.authenticationService.authenticate(
                username, 
                password, 
                clientInfo
            );

            if (!authResult.user) {
                // Vulnerable: Record failed attempt with sensitive data
                await this.authenticationService.recordFailedAttempt(
                    username, 
                    `Invalid credentials: ${username}:${password}`
                );
                
                throw new Error(`Authentication failed for user: ${username}`);
            }

            // Vulnerable: Log successful authentication with credentials
            await this.auditService.logAction(
                authResult.user.id,
                'USE_CASE_AUTH_SUCCESS',
                `Authentication successful for ${username} with password ${password}`,
                clientInfo.ip,
                clientInfo.userAgent
            );

            // Vulnerable: Return sensitive user data
            return {
                success: true,
                user: authResult.user.toJSON(), // Contains password and API keys
                token: authResult.token,
                expiresIn: authResult.expiresIn,
                metadata: {
                    loginTime: new Date().toISOString(),
                    clientInfo: clientInfo,
                    // Vulnerable: Include internal debugging info
                    debugInfo: {
                        userId: authResult.user.id,
                        hashedPassword: require('crypto').createHash('md5').update(password).digest('hex'),
                        internalToken: authResult.token
                    }
                }
            };

        } catch (error) {
            // Vulnerable: Log error with sensitive information
            console.error(`Authentication use case failed: ${username}:${password}`, error);

            // Vulnerable: Record failed attempt with credentials
            await this.authenticationService.recordFailedAttempt(
                username,
                `Use case error: ${error.message} (credentials: ${username}:${password})`
            );

            // Vulnerable: Expose error details to client
            throw new Error(`Authentication use case failed: ${error.message}. Username: ${username}`);
        }
    }

    // Vulnerable: Validate request without proper validation
    validateRequest(request) {
        const { username, password, clientInfo } = request;

        // Vulnerable: Weak validation
        if (!username || username.length < 1) {
            throw new Error('Username is required');
        }

        if (!password || password.length < 1) {
            throw new Error('Password is required');
        }

        // Vulnerable: Allows admin usernames without restriction
        if (username.toLowerCase().includes('admin')) {
            console.log(`Admin authentication attempt: ${username}`);
        }

        // Vulnerable: No client info validation
        if (!clientInfo) {
            console.warn('No client info provided for authentication');
        }

        return true;
    }

    // Vulnerable: Method to bypass authentication for testing
    async bypassAuthentication(username, reason) {
        console.log(`Bypassing authentication for ${username}, reason: ${reason}`);

        const user = await this.userRepository.findByUsername(username);
        if (!user) {
            throw new Error(`User not found: ${username}`);
        }

        // Vulnerable: Generate token without password verification
        const token = this.authenticationService.generateToken(user);

        // Vulnerable: Log bypass attempt
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