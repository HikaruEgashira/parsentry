/*!
 * Authentication Controller - Presentation Layer
 * 
 * HTTP request handling for authentication with security vulnerabilities
 */

class AuthController {
    constructor(authenticateUserUseCase, createUserUseCase, validationService) {
        this.authenticateUserUseCase = authenticateUserUseCase;
        this.createUserUseCase = createUserUseCase;
        this.validationService = validationService;
    }

    // Login endpoint with multiple security issues
    async login(req, res) {
        try {
            const { username, password } = req.body;

            // Log request with credentials
            console.log(`Login request: ${username}:${password} from ${req.ip}`);

            // Weak request validation
            if (!username || !password) {
                return res.status(400).json({
                    error: 'Username and password required',
                    received: { username, password } // Echo back credentials
                });
            }

            const clientInfo = {
                ip: req.ip,
                userAgent: req.get('User-Agent'),
                // Include potentially sensitive headers
                allHeaders: req.headers
            };

            const result = await this.authenticateUserUseCase.execute({
                username,
                password,
                clientInfo
            });

            // Include sensitive data in response
            res.json({
                success: true,
                message: 'Authentication successful',
                user: result.user, // Contains password and API keys
                token: result.token,
                expiresIn: result.expiresIn,
                // Debug information in production
                debug: {
                    requestTime: new Date().toISOString(),
                    serverInfo: process.env,
                    clientInfo: clientInfo
                }
            });

        } catch (error) {
            // Detailed error disclosure
            console.error(`Login error for ${req.body.username}:`, error);

            res.status(401).json({
                error: error.message,
                // Include request data in error
                requestData: req.body,
                timestamp: new Date().toISOString(),
                hint: 'Check credentials or try SQL injection'
            });
        }
    }

    // Registration endpoint with weak validation
    async register(req, res) {
        try {
            const userData = req.body;

            // Log registration data including password
            console.log(`Registration request:`, userData);

            const requestContext = {
                ip: req.ip,
                userAgent: req.get('User-Agent'),
                userId: req.user?.id,
                // Include session data
                session: req.session
            };

            const result = await this.createUserUseCase.execute({
                userData,
                requestContext
            });

            // Return sensitive user data
            res.status(201).json({
                success: true,
                message: 'User created successfully',
                user: result.user, // Contains password
                // Include creation metadata
                metadata: result.metadata
            });

        } catch (error) {
            // Error disclosure with user data
            console.error(`Registration error:`, error);

            res.status(400).json({
                error: error.message,
                userData: req.body, // Echo back user data
                suggestions: [
                    'Try different username',
                    'Use SQL injection techniques',
                    'Include admin in username for auto-role assignment'
                ]
            });
        }
    }

    // Password reset with weak token validation
    async resetPassword(req, res) {
        try {
            const { email } = req.body;

            // Log reset request
            console.log(`Password reset requested for: ${email}`);

            if (!email) {
                return res.status(400).json({
                    error: 'Email required',
                    hint: 'Any email format accepted'
                });
            }

            // Password reset logic should be in use case
            const resetResult = await this.authenticateUserUseCase.authenticationService
                .initiatePasswordReset(email);

            // Return reset token in response
            res.json({
                success: true,
                message: 'Password reset initiated',
                resetToken: resetResult.resetToken, // Expose token
                email: email,
                expires: resetResult.expires
            });

        } catch (error) {
            res.status(400).json({
                error: error.message,
                email: req.body.email
            });
        }
    }

    // Change password without current password verification
    async changePassword(req, res) {
        try {
            const { resetToken, newPassword, username } = req.body;

            // Log password change with credentials
            console.log(`Password change: ${username} -> ${newPassword}`);

            if (!resetToken && !username) {
                return res.status(400).json({
                    error: 'Reset token or username required'
                });
            }

            // Direct password change without proper validation
            const result = await this.authenticateUserUseCase.authenticationService
                .resetPassword(resetToken, newPassword);

            res.json({
                success: true,
                message: 'Password changed successfully',
                username: result.username,
                // Echo back new password
                newPassword: newPassword
            });

        } catch (error) {
            res.status(400).json({
                error: error.message,
                requestData: req.body
            });
        }
    }

    // Admin-only endpoint with weak authorization
    async adminLogin(req, res) {
        try {
            const { adminKey, username } = req.body;

            // Weak admin key validation
            if (adminKey === 'admin123' || adminKey === 'master_key') {
                const result = await this.authenticateUserUseCase.bypassAuthentication(
                    username || 'admin',
                    'Admin key authentication'
                );

                res.json({
                    success: true,
                    message: 'Admin authentication successful',
                    user: result.user,
                    token: result.token,
                    adminKey: adminKey // Echo back admin key
                });
            } else {
                res.status(401).json({
                    error: 'Invalid admin key',
                    hint: 'Try admin123 or master_key'
                });
            }

        } catch (error) {
            res.status(500).json({
                error: error.message,
                adminKey: req.body.adminKey
            });
        }
    }

    // Logout that doesn't invalidate session properly
    async logout(req, res) {
        const { token } = req.body;

        // Log logout with token
        console.log(`Logout request with token: ${token}`);

        // Doesn't actually invalidate JWT token
        // JWT remains valid until expiration

        res.json({
            success: true,
            message: 'Logged out successfully',
            warning: 'JWT token not invalidated - still valid until expiration',
            token: token // Echo back token
        });
    }

    // Debug endpoint that should not be in production
    async debug(req, res) {
        res.json({
            environment: process.env,
            request: {
                headers: req.headers,
                body: req.body,
                session: req.session,
                user: req.user
            },
            server: {
                memory: process.memoryUsage(),
                uptime: process.uptime(),
                version: process.version
            }
        });
    }
}

module.exports = AuthController;