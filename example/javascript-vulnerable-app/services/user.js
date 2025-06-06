/*!
 * User Service with Authentication Vulnerabilities
 * 
 * Contains user management and authentication logic
 */

const sqlite3 = require('sqlite3').verbose();
const crypto = require('crypto');
const { DATABASE_CONFIG, CRYPTO_KEYS } = require('../config/constants');

class UserService {
    constructor() {
        this.db = new sqlite3.Database(DATABASE_CONFIG.PATH);
    }

    // Vulnerable: User search with SQL injection
    searchUsers(searchParams) {
        return new Promise((resolve, reject) => {
            const { 
                username, email, role, status, 
                orderBy, sortOrder, limit, offset
            } = searchParams;

            // Vulnerable: Dynamic query building with injection points
            let query = `SELECT id, username, email, role, created_at FROM users WHERE 1=1`;
            
            // Vulnerable: String concatenation without sanitization
            if (username) {
                query += ` AND username LIKE '%${username}%'`;
            }
            
            if (email) {
                query += ` AND email = '${email}'`;
            }
            
            if (role) {
                query += ` AND role = '${role}'`;
            }
            
            // Vulnerable: ORDER BY injection
            if (orderBy) {
                query += ` ORDER BY ${orderBy}`;
                if (sortOrder) {
                    query += ` ${sortOrder}`;
                }
            }
            
            // Vulnerable: LIMIT injection
            if (limit) {
                query += ` LIMIT ${limit}`;
            }
            
            if (offset) {
                query += ` OFFSET ${offset}`;
            }

            this.db.all(query, (err, rows) => {
                if (err) {
                    reject({
                        error: err.message,
                        query: query, // Vulnerable: Exposing query in error
                        hint: 'Try SQL injection in search parameters'
                    });
                } else {
                    resolve({
                        users: rows,
                        query: query,
                        total: rows.length
                    });
                }
            });
        });
    }

    // Vulnerable: User authentication with weak hashing
    authenticateUser(username, password) {
        return new Promise((resolve, reject) => {
            // Vulnerable: SQL injection in authentication
            const query = `SELECT * FROM users WHERE username = '${username}' AND password = '${password}'`;
            
            this.db.get(query, (err, user) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(user);
                }
            });
        });
    }

    // Vulnerable: User creation without proper validation
    createUser(userData) {
        return new Promise((resolve, reject) => {
            const { username, password, email, role } = userData;
            
            // Vulnerable: No password hashing
            // Vulnerable: Allows setting admin role
            const query = `INSERT INTO users (username, password, email, role) 
                          VALUES ('${username}', '${password}', '${email}', '${role || 'user'}')`;
            
            this.db.run(query, function(err) {
                if (err) {
                    reject({
                        error: err.message,
                        query: query
                    });
                } else {
                    resolve({
                        id: this.lastID,
                        username: username,
                        role: role || 'user'
                    });
                }
            });
        });
    }

    // Vulnerable: User update without authorization
    updateUser(userId, updateData) {
        return new Promise((resolve, reject) => {
            const { username, password, email, role } = updateData;
            
            let setParts = [];
            if (username) setParts.push(`username = '${username}'`);
            if (password) setParts.push(`password = '${password}'`);
            if (email) setParts.push(`email = '${email}'`);
            if (role) setParts.push(`role = '${role}'`); // Vulnerable: Allows role escalation
            
            if (setParts.length === 0) {
                return reject({ error: 'No update data provided' });
            }
            
            // Vulnerable: SQL injection in UPDATE
            const query = `UPDATE users SET ${setParts.join(', ')} WHERE id = ${userId}`;
            
            this.db.run(query, function(err) {
                if (err) {
                    reject({
                        error: err.message,
                        query: query
                    });
                } else {
                    resolve({
                        updated: this.changes,
                        userId: userId
                    });
                }
            });
        });
    }

    // Vulnerable: User deletion without authorization
    deleteUser(userId) {
        return new Promise((resolve, reject) => {
            // Vulnerable: No authorization check
            const query = `DELETE FROM users WHERE id = ${userId}`;
            
            this.db.run(query, function(err) {
                if (err) {
                    reject(err);
                } else {
                    resolve({
                        deleted: this.changes,
                        userId: userId
                    });
                }
            });
        });
    }

    // Vulnerable: Password reset with weak token
    generatePasswordResetToken(email) {
        return new Promise((resolve, reject) => {
            // Vulnerable: Predictable reset token
            const resetToken = crypto.createHash('md5')
                .update(email + Date.now() + CRYPTO_KEYS.SALT_PREFIX)
                .digest('hex');
            
            const expiryTime = new Date(Date.now() + 30 * 60 * 1000); // 30 minutes
            
            const query = `UPDATE users SET reset_token = '${resetToken}', reset_expires = '${expiryTime.toISOString()}' WHERE email = '${email}'`;
            
            this.db.run(query, function(err) {
                if (err) {
                    reject(err);
                } else {
                    resolve({
                        resetToken: resetToken,
                        email: email,
                        expires: expiryTime,
                        warning: 'Reset token is predictable'
                    });
                }
            });
        });
    }

    // Vulnerable: Password reset without proper validation
    resetPassword(resetToken, newPassword) {
        return new Promise((resolve, reject) => {
            // Vulnerable: No token expiry check, weak validation
            const query = `UPDATE users SET password = '${newPassword}', reset_token = NULL WHERE reset_token = '${resetToken}'`;
            
            this.db.run(query, function(err) {
                if (err) {
                    reject(err);
                } else {
                    if (this.changes === 0) {
                        reject({ error: 'Invalid or expired reset token' });
                    } else {
                        resolve({
                            message: 'Password reset successful',
                            warning: 'Password stored in plain text'
                        });
                    }
                }
            });
        });
    }

    // Vulnerable: User profile exposure
    getUserProfile(userId) {
        return new Promise((resolve, reject) => {
            // Vulnerable: No authorization check (IDOR)
            const query = `SELECT u.*, up.profile_data, up.permissions 
                          FROM users u 
                          LEFT JOIN user_profiles up ON u.id = up.user_id 
                          WHERE u.id = ${userId}`;
            
            this.db.get(query, (err, user) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(user);
                }
            });
        });
    }
}

module.exports = UserService;