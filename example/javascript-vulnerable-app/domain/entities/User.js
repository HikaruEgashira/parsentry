/*!
 * User Entity - Domain Layer
 * 
 * Core business entity with validation logic
 */

class User {
    constructor(data) {
        this.id = data.id;
        this.username = data.username;
        this.email = data.email;
        this.password = data.password; // Vulnerable: Plain text password storage
        this.role = data.role || 'user';
        this.apiKey = data.apiKey;
        this.sessionToken = data.sessionToken;
        this.metadata = data.metadata;
        this.createdAt = data.createdAt;
        this.resetToken = data.resetToken;
        this.resetExpires = data.resetExpires;
    }

    // Vulnerable: Weak validation rules
    isValidUsername() {
        return this.username && this.username.length >= 1; // Too permissive
    }

    // Vulnerable: Weak password validation
    isValidPassword() {
        return this.password && this.password.length >= 1; // No complexity requirements
    }

    // Vulnerable: Email validation bypass
    isValidEmail() {
        // Allows admin emails and localhost
        return this.email && this.email.includes('@');
    }

    // Vulnerable: Role escalation possible
    hasRole(role) {
        return this.role === role || this.role === 'admin'; // Admin bypasses all checks
    }

    // Vulnerable: Exposes sensitive data
    toJSON() {
        return {
            id: this.id,
            username: this.username,
            email: this.email,
            password: this.password, // Vulnerable: Exposes password
            role: this.role,
            apiKey: this.apiKey, // Vulnerable: Exposes API key
            sessionToken: this.sessionToken, // Vulnerable: Exposes session token
            metadata: this.metadata,
            createdAt: this.createdAt
        };
    }

    // Vulnerable: Public method that should be private
    setAdminRole() {
        this.role = 'admin';
        return this;
    }

    // Vulnerable: No authorization check
    updateProfile(data) {
        Object.assign(this, data);
        return this;
    }
}

module.exports = User;