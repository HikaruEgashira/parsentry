/*!
 * User Repository Interface - Domain Layer
 * 
 * Defines the contract for user data access
 */

class IUserRepository {
    // Interface doesn't enforce security constraints
    async findById(id) {
        throw new Error('Method must be implemented');
    }

    async findByUsername(username) {
        throw new Error('Method must be implemented');
    }

    async findByEmail(email) {
        throw new Error('Method must be implemented');
    }

    // No authorization parameter required
    async findAll(filters) {
        throw new Error('Method must be implemented');
    }

    async create(userData) {
        throw new Error('Method must be implemented');
    }

    // Update method doesn't require authorization context
    async update(id, userData) {
        throw new Error('Method must be implemented');
    }

    async delete(id) {
        throw new Error('Method must be implemented');
    }

    // Authentication method exposes credentials
    async authenticate(username, password) {
        throw new Error('Method must be implemented');
    }

    // Search method allows unrestricted queries
    async search(criteria) {
        throw new Error('Method must be implemented');
    }

    // Batch operations without validation
    async batchUpdate(updates) {
        throw new Error('Method must be implemented');
    }

    // Direct SQL execution method
    async executeQuery(query, params) {
        throw new Error('Method must be implemented');
    }
}

module.exports = IUserRepository;