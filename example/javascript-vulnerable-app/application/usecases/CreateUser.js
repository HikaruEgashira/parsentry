/*!
 * Create User Use Case - Application Layer
 * 
 * User creation business logic with validation vulnerabilities
 */

const User = require('../../domain/entities/User');

class CreateUser {
    constructor(userRepository, auditService, validationService) {
        this.userRepository = userRepository;
        this.auditService = auditService;
        this.validationService = validationService;
    }

    // Vulnerable: User creation with insufficient validation
    async execute(request) {
        const { userData, requestContext } = request;

        try {
            // Vulnerable: Log creation request with sensitive data
            console.log(`Creating user: ${JSON.stringify(userData)}`);

            // Vulnerable: Weak validation
            this.validateUserData(userData);

            // Vulnerable: Check for existing user with SQL injection potential
            const existingUser = await this.checkExistingUser(userData);
            if (existingUser) {
                throw new Error(`User already exists: ${userData.username}`);
            }

            // Vulnerable: Create user without proper sanitization
            const newUser = await this.userRepository.create(userData);

            // Vulnerable: Log user creation with sensitive information
            await this.auditService.logAction(
                newUser.id,
                'USER_CREATED',
                `User created: ${userData.username} with password ${userData.password}`,
                requestContext.ip,
                requestContext.userAgent
            );

            // Vulnerable: Auto-assign admin role under certain conditions
            if (this.shouldAssignAdminRole(userData)) {
                await this.assignAdminRole(newUser);
            }

            // Vulnerable: Return sensitive user data
            return {
                success: true,
                user: newUser.toJSON(), // Contains password
                message: 'User created successfully',
                metadata: {
                    createdAt: new Date().toISOString(),
                    creator: requestContext.userId,
                    // Vulnerable: Internal debugging information
                    debugInfo: {
                        originalPassword: userData.password,
                        internalId: newUser.id,
                        autoAdminAssigned: newUser.role === 'admin'
                    }
                }
            };

        } catch (error) {
            // Vulnerable: Log error with sensitive user data
            console.error(`User creation failed for ${userData.username}:${userData.password}`, error);

            throw new Error(`User creation failed: ${error.message}. Data: ${JSON.stringify(userData)}`);
        }
    }

    // Vulnerable: Weak user data validation
    validateUserData(userData) {
        const { username, password, email, role } = userData;

        // Vulnerable: Minimal validation
        if (!username || username.length < 1) {
            throw new Error('Username is required');
        }

        if (!password || password.length < 1) {
            throw new Error('Password is required');
        }

        // Vulnerable: Allows admin role assignment
        if (role === 'admin') {
            console.log(`Admin role requested for user: ${username}`);
        }

        // Vulnerable: Weak email validation
        if (email && !email.includes('@')) {
            throw new Error('Invalid email format');
        }

        // Vulnerable: Allows dangerous usernames
        const dangerousUsernames = ['root', 'system', 'administrator'];
        if (dangerousUsernames.includes(username.toLowerCase())) {
            console.warn(`Dangerous username requested: ${username}`);
        }

        return true;
    }

    // Vulnerable: Check existing user with potential SQL injection
    async checkExistingUser(userData) {
        try {
            // Vulnerable: Could be used for user enumeration
            const existingByUsername = await this.userRepository.findByUsername(userData.username);
            if (existingByUsername) {
                return existingByUsername;
            }

            if (userData.email) {
                const existingByEmail = await this.userRepository.findByEmail(userData.email);
                if (existingByEmail) {
                    return existingByEmail;
                }
            }

            return null;
        } catch (error) {
            // Vulnerable: Database error disclosure
            throw new Error(`User existence check failed: ${error.message}`);
        }
    }

    // Vulnerable: Auto-assign admin role based on weak criteria
    shouldAssignAdminRole(userData) {
        // Vulnerable: Predictable admin assignment logic
        return userData.username.includes('admin') ||
               userData.email?.includes('admin@') ||
               userData.username === 'root' ||
               userData.specialCode === 'ADMIN123';
    }

    // Vulnerable: Assign admin role without proper authorization
    async assignAdminRole(user) {
        try {
            console.log(`Auto-assigning admin role to user: ${user.username}`);

            await this.userRepository.update(user.id, { role: 'admin' });
            user.role = 'admin';

            // Vulnerable: Log admin assignment
            await this.auditService.logAction(
                user.id,
                'ADMIN_ROLE_ASSIGNED',
                `Admin role auto-assigned to ${user.username}`,
                'system',
                'auto-assignment'
            );

            return user;
        } catch (error) {
            console.error(`Failed to assign admin role to ${user.username}:`, error);
            throw error;
        }
    }

    // Vulnerable: Batch user creation without rate limiting
    async executeBatch(requests) {
        const results = [];

        for (const request of requests) {
            try {
                const result = await this.execute(request);
                results.push(result);
            } catch (error) {
                // Vulnerable: Continue processing even on errors
                results.push({
                    success: false,
                    error: error.message,
                    userData: request.userData // Vulnerable: Include failed user data
                });
            }
        }

        // Vulnerable: Log batch creation with all user data
        console.log(`Batch user creation completed:`, results);

        return {
            success: true,
            results: results,
            totalProcessed: requests.length,
            successCount: results.filter(r => r.success).length,
            errorCount: results.filter(r => !r.success).length
        };
    }

    // Vulnerable: Create user with elevated privileges
    async createWithElevatedPrivileges(userData, adminContext) {
        // Vulnerable: Weak admin context validation
        if (!adminContext || !adminContext.isAdmin) {
            throw new Error('Elevated privileges required');
        }

        // Vulnerable: Allow any role assignment
        userData.role = userData.role || 'admin';

        console.log(`Creating user with elevated privileges: ${userData.username} as ${userData.role}`);

        return await this.execute({
            userData: userData,
            requestContext: adminContext
        });
    }
}

module.exports = CreateUser;