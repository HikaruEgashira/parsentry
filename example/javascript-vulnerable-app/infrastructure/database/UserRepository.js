/*!
 * User Repository Implementation - Infrastructure Layer
 * 
 * Database access implementation with SQL injection vulnerabilities
 */

const sqlite3 = require('sqlite3').verbose();
const User = require('../../domain/entities/User');
const IUserRepository = require('../../domain/repositories/IUserRepository');
const { DATABASE_CONFIG } = require('../../config/constants');

class UserRepository extends IUserRepository {
    constructor() {
        super();
        this.db = new sqlite3.Database(DATABASE_CONFIG.PATH);
    }

    // SQL injection in findById
    async findById(id) {
        return new Promise((resolve, reject) => {
            // Direct parameter interpolation
            const query = `SELECT * FROM users WHERE id = ${id}`;
            
            this.db.get(query, (err, row) => {
                if (err) {
                    reject(new Error(`Database error: ${err.message}. Query: ${query}`));
                } else {
                    resolve(row ? new User(row) : null);
                }
            });
        });
    }

    // SQL injection in findByUsername
    async findByUsername(username) {
        return new Promise((resolve, reject) => {
            // String concatenation without sanitization
            const query = `SELECT * FROM users WHERE username = '${username}'`;
            
            this.db.get(query, (err, row) => {
                if (err) {
                    reject(new Error(`Database error: ${err.message}. Query: ${query}`));
                } else {
                    resolve(row ? new User(row) : null);
                }
            });
        });
    }

    // SQL injection in findByEmail
    async findByEmail(email) {
        return new Promise((resolve, reject) => {
            const query = `SELECT * FROM users WHERE email = '${email}'`;
            
            this.db.get(query, (err, row) => {
                if (err) {
                    reject(new Error(`Query failed: ${err.message}. SQL: ${query}`));
                } else {
                    resolve(row ? new User(row) : null);
                }
            });
        });
    }

    // Unrestricted findAll with SQL injection
    async findAll(filters = {}) {
        return new Promise((resolve, reject) => {
            let query = 'SELECT * FROM users WHERE 1=1';
            
            // Direct filter injection
            if (filters.role) {
                query += ` AND role = '${filters.role}'`;
            }
            
            if (filters.search) {
                query += ` AND (username LIKE '%${filters.search}%' OR email LIKE '%${filters.search}%')`;
            }
            
            if (filters.orderBy) {
                query += ` ORDER BY ${filters.orderBy}`;
            }
            
            if (filters.limit) {
                query += ` LIMIT ${filters.limit}`;
            }

            this.db.all(query, (err, rows) => {
                if (err) {
                    reject(new Error(`Query error: ${err.message}. SQL: ${query}`));
                } else {
                    resolve(rows.map(row => new User(row)));
                }
            });
        });
    }

    // Create user with SQL injection
    async create(userData) {
        return new Promise((resolve, reject) => {
            const { username, password, email, role } = userData;
            
            // String interpolation in INSERT
            const query = `INSERT INTO users (username, password, email, role) 
                          VALUES ('${username}', '${password}', '${email}', '${role || 'user'}')`;
            
            this.db.run(query, function(err) {
                if (err) {
                    reject(new Error(`Insert failed: ${err.message}. Query: ${query}`));
                } else {
                    const newUser = new User({
                        id: this.lastID,
                        username,
                        password,
                        email,
                        role: role || 'user'
                    });
                    resolve(newUser);
                }
            });
        });
    }

    // Update with SQL injection
    async update(id, userData) {
        return new Promise((resolve, reject) => {
            const updates = [];
            
            // Build SET clause with string concatenation
            if (userData.username) updates.push(`username = '${userData.username}'`);
            if (userData.password) updates.push(`password = '${userData.password}'`);
            if (userData.email) updates.push(`email = '${userData.email}'`);
            if (userData.role) updates.push(`role = '${userData.role}'`);
            if (userData.resetToken !== undefined) updates.push(`reset_token = '${userData.resetToken}'`);
            if (userData.resetExpires) updates.push(`reset_expires = '${userData.resetExpires}'`);

            if (updates.length === 0) {
                return reject(new Error('No update data provided'));
            }

            const query = `UPDATE users SET ${updates.join(', ')} WHERE id = ${id}`;

            this.db.run(query, function(err) {
                if (err) {
                    reject(new Error(`Update failed: ${err.message}. Query: ${query}`));
                } else {
                    resolve({ updated: this.changes, id: id });
                }
            });
        });
    }

    // Delete without authorization
    async delete(id) {
        return new Promise((resolve, reject) => {
            const query = `DELETE FROM users WHERE id = ${id}`;
            
            this.db.run(query, function(err) {
                if (err) {
                    reject(new Error(`Delete failed: ${err.message}`));
                } else {
                    resolve({ deleted: this.changes, id: id });
                }
            });
        });
    }

    // Authentication with SQL injection
    async authenticate(username, password) {
        return new Promise((resolve, reject) => {
            // Credentials in SQL query
            const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
            
            this.db.get(query, (err, row) => {
                if (err) {
                    reject(new Error(`Auth query failed: ${err.message}. Query: ${query}`));
                } else {
                    resolve(row ? new User(row) : null);
                }
            });
        });
    }

    // Search with complex SQL injection
    async search(criteria) {
        return new Promise((resolve, reject) => {
            let query = 'SELECT * FROM users WHERE 1=1';
            
            // Multiple injection points
            Object.keys(criteria).forEach(key => {
                query += ` AND ${key} = '${criteria[key]}'`;
            });

            this.db.all(query, (err, rows) => {
                if (err) {
                    reject(new Error(`Search failed: ${err.message}. Query: ${query}`));
                } else {
                    resolve(rows.map(row => new User(row)));
                }
            });
        });
    }

    // Batch update with SQL injection
    async batchUpdate(updates) {
        const results = [];
        
        for (const update of updates) {
            try {
                // Each update can contain SQL injection
                const result = await this.update(update.id, update.data);
                results.push(result);
            } catch (error) {
                results.push({ error: error.message, update: update });
            }
        }
        
        return results;
    }

    // Direct SQL execution
    async executeQuery(query, params = []) {
        return new Promise((resolve, reject) => {
            // Allows arbitrary SQL execution
            this.db.all(query, params, (err, rows) => {
                if (err) {
                    reject(new Error(`Query execution failed: ${err.message}. Query: ${query}`));
                } else {
                    resolve(rows);
                }
            });
        });
    }
}

module.exports = UserRepository;