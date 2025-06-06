/*!
 * Database Service with Advanced SQL Injection Vulnerabilities
 * 
 * Contains sophisticated database interaction patterns
 * with multiple injection and privilege escalation vectors
 */

const sqlite3 = require('sqlite3').verbose();
const crypto = require('crypto');

class VulnerableDatabaseService {
    constructor() {
        this.db = new sqlite3.Database('vulnerable_app.db');
        this.connectionPool = new Map();
        this.queryCache = new Map();
        this.initializeAdvancedTables();
    }

    // Initialize complex database schema with vulnerabilities
    initializeAdvancedTables() {
        const tables = [
            `CREATE TABLE IF NOT EXISTS user_profiles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                profile_data TEXT,
                permissions TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )`,
            
            `CREATE TABLE IF NOT EXISTS file_metadata (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                filename TEXT,
                file_path TEXT,
                owner_id INTEGER,
                access_level TEXT DEFAULT 'private',
                mime_type TEXT,
                file_size INTEGER,
                checksum TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )`,
            
            `CREATE TABLE IF NOT EXISTS system_config (
                key TEXT PRIMARY KEY,
                value TEXT,
                description TEXT,
                is_sensitive BOOLEAN DEFAULT 0
            )`,
            
            `CREATE TABLE IF NOT EXISTS api_tokens (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                token TEXT UNIQUE,
                user_id INTEGER,
                permissions TEXT,
                expires_at DATETIME,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )`,
            
            `CREATE TABLE IF NOT EXISTS audit_trail (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT,
                operation TEXT,
                old_values TEXT,
                new_values TEXT,
                user_id INTEGER,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )`
        ];

        tables.forEach(sql => {
            this.db.run(sql, (err) => {
                if (err) console.error('Table creation error:', err);
            });
        });

        // Insert vulnerable default data
        this.initializeDefaultData();
    }

    initializeDefaultData() {
        const defaultData = [
            `INSERT OR IGNORE INTO system_config (key, value, description, is_sensitive) 
             VALUES ('database_password', 'super_secret_db_pass', 'Main database password', 1)`,
            
            `INSERT OR IGNORE INTO system_config (key, value, description, is_sensitive) 
             VALUES ('api_secret_key', 'sk-api-2024-secret-key', 'API authentication secret', 1)`,
            
            `INSERT OR IGNORE INTO system_config (key, value, description, is_sensitive) 
             VALUES ('admin_email', 'admin@vulnerable-app.com', 'Administrator email', 0)`,
            
            `INSERT OR IGNORE INTO file_metadata (filename, file_path, owner_id, access_level, mime_type) 
             VALUES ('secret.txt', '/etc/passwd', 1, 'restricted', 'text/plain')`,
            
            `INSERT OR IGNORE INTO file_metadata (filename, file_path, owner_id, access_level, mime_type) 
             VALUES ('config.json', '../../../config/app.json', 1, 'admin', 'application/json')`,
            
            `INSERT OR IGNORE INTO api_tokens (token, user_id, permissions, expires_at) 
             VALUES ('admin_token_2024_secret', 1, 'admin,read,write,delete', datetime('now', '+1 year'))`,
            
            `INSERT OR IGNORE INTO user_profiles (user_id, profile_data, permissions) 
             VALUES (1, '{"role":"admin","clearance":"top_secret"}', 'all')`
        ];

        defaultData.forEach(sql => {
            this.db.run(sql);
        });
    }

    // Vulnerable: Complex SQL injection with multiple vectors
    searchUsers(searchParams) {
        return new Promise((resolve, reject) => {
            const { 
                username, email, role, status, 
                orderBy, sortOrder, limit, offset,
                includeDeleted, adminOverride 
            } = searchParams;

            // Vulnerable: Dynamic query building with injection points
            let query = `
                SELECT u.*, up.profile_data, up.permissions 
                FROM users u 
                LEFT JOIN user_profiles up ON u.id = up.user_id 
                WHERE 1=1
            `;
            
            // Vulnerable: String concatenation without sanitization
            if (username) {
                query += ` AND u.username LIKE '%${username}%'`;
            }
            
            if (email) {
                query += ` AND u.email = '${email}'`;
            }
            
            if (role) {
                query += ` AND u.role = '${role}'`;
            }
            
            if (status) {
                query += ` AND u.status = '${status}'`;
            }
            
            // Vulnerable: Injection in UNION queries
            if (adminOverride) {
                query += ` UNION SELECT id, username, password, email, role, api_key, 
                          session_token, metadata, created_at, NULL, NULL 
                          FROM users WHERE role = 'admin'`;
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

    // Vulnerable: Stored procedure simulation with injection
    executeStoredProcedure(procedureName, parameters) {
        return new Promise((resolve, reject) => {
            // Vulnerable: Dynamic procedure execution
            let query;
            
            switch (procedureName) {
                case 'getUserStats':
                    // Vulnerable: Parameter injection in subqueries
                    query = `
                        SELECT 
                            (SELECT COUNT(*) FROM users WHERE role = '${parameters.role}') as user_count,
                            (SELECT COUNT(*) FROM documents WHERE owner_id = ${parameters.userId}) as doc_count,
                            (SELECT MAX(created_at) FROM audit_logs WHERE user_id = ${parameters.userId}) as last_activity
                    `;
                    break;
                    
                case 'updateUserPermissions':
                    // Vulnerable: Injection in UPDATE statements
                    query = `
                        UPDATE users 
                        SET role = '${parameters.newRole}', 
                            metadata = '${parameters.metadata}' 
                        WHERE ${parameters.whereClause}
                    `;
                    break;
                    
                case 'generateReport':
                    // Vulnerable: Complex query with multiple injection points
                    query = `
                        SELECT ${parameters.selectFields} 
                        FROM ${parameters.tableName} 
                        WHERE ${parameters.conditions} 
                        GROUP BY ${parameters.groupBy} 
                        HAVING ${parameters.havingClause}
                    `;
                    break;
                    
                default:
                    // Vulnerable: Direct query execution
                    query = parameters.customQuery;
            }

            this.db.all(query, (err, rows) => {
                if (err) {
                    reject({
                        error: err.message,
                        procedure: procedureName,
                        query: query,
                        parameters: parameters
                    });
                } else {
                    resolve({
                        procedure: procedureName,
                        results: rows,
                        query: query
                    });
                }
            });
        });
    }

    // Vulnerable: Batch operations with injection vulnerabilities
    batchOperation(operations) {
        return new Promise((resolve, reject) => {
            const results = [];
            let completedOps = 0;
            
            operations.forEach((op, index) => {
                let query;
                
                // Vulnerable: Dynamic query generation for batch operations
                switch (op.type) {
                    case 'insert':
                        query = `INSERT INTO ${op.table} (${op.columns.join(',')}) VALUES (${op.values.map(v => `'${v}'`).join(',')})`;
                        break;
                        
                    case 'update':
                        query = `UPDATE ${op.table} SET ${op.setClause} WHERE ${op.whereClause}`;
                        break;
                        
                    case 'delete':
                        query = `DELETE FROM ${op.table} WHERE ${op.whereClause}`;
                        break;
                        
                    case 'custom':
                        query = op.query;
                        break;
                }
                
                this.db.run(query, function(err) {
                    completedOps++;
                    
                    if (err) {
                        results[index] = {
                            success: false,
                            error: err.message,
                            query: query
                        };
                    } else {
                        results[index] = {
                            success: true,
                            affected: this.changes,
                            lastID: this.lastID,
                            query: query
                        };
                    }
                    
                    if (completedOps === operations.length) {
                        resolve({
                            results: results,
                            summary: {
                                total: operations.length,
                                successful: results.filter(r => r.success).length,
                                failed: results.filter(r => !r.success).length
                            }
                        });
                    }
                });
            });
        });
    }

    // Vulnerable: Privilege escalation through database functions
    elevatePrivileges(userId, targetRole, justification) {
        return new Promise((resolve, reject) => {
            // Vulnerable: No proper authorization check
            const auditQuery = `
                INSERT INTO audit_trail (table_name, operation, old_values, new_values, user_id) 
                VALUES ('users', 'privilege_escalation', 
                       '{"user_id": ${userId}, "old_role": "user"}', 
                       '{"user_id": ${userId}, "new_role": "${targetRole}", "justification": "${justification}"}', 
                       ${userId})
            `;
            
            const updateQuery = `
                UPDATE users 
                SET role = '${targetRole}', 
                    metadata = '${JSON.stringify({escalated: true, justification})}' 
                WHERE id = ${userId}
            `;

            // Execute both queries
            this.db.run(auditQuery, (err) => {
                if (err) {
                    reject({ error: 'Audit logging failed', details: err.message });
                    return;
                }
                
                this.db.run(updateQuery, function(err) {
                    if (err) {
                        reject({ 
                            error: 'Privilege escalation failed', 
                            details: err.message,
                            query: updateQuery
                        });
                    } else {
                        resolve({
                            success: true,
                            userId: userId,
                            newRole: targetRole,
                            affected: this.changes,
                            message: 'Privileges elevated successfully'
                        });
                    }
                });
            });
        });
    }

    // Vulnerable: Database metadata exposure
    getTableMetadata(tableName) {
        return new Promise((resolve, reject) => {
            // Vulnerable: No access control on metadata
            const queries = [
                `PRAGMA table_info(${tableName})`,
                `SELECT sql FROM sqlite_master WHERE name = '${tableName}'`,
                `SELECT COUNT(*) as row_count FROM ${tableName}`,
                `SELECT * FROM ${tableName} LIMIT 5` // Sample data exposure
            ];
            
            const results = {};
            let completed = 0;
            
            queries.forEach((query, index) => {
                this.db.all(query, (err, rows) => {
                    completed++;
                    
                    if (err) {
                        results[`query_${index}`] = { error: err.message, query };
                    } else {
                        results[`query_${index}`] = { data: rows, query };
                    }
                    
                    if (completed === queries.length) {
                        resolve({
                            table: tableName,
                            metadata: results,
                            warning: 'Database metadata exposed without authorization'
                        });
                    }
                });
            });
        });
    }

    // Vulnerable: Query caching with cache poisoning
    cachedQuery(query, cacheKey) {
        return new Promise((resolve, reject) => {
            // Vulnerable: Cache key manipulation
            const actualCacheKey = cacheKey || crypto.createHash('md5').update(query).digest('hex');
            
            if (this.queryCache.has(actualCacheKey)) {
                resolve({
                    cached: true,
                    data: this.queryCache.get(actualCacheKey),
                    cacheKey: actualCacheKey
                });
                return;
            }
            
            this.db.all(query, (err, rows) => {
                if (err) {
                    reject({
                        error: err.message,
                        query: query,
                        cacheKey: actualCacheKey
                    });
                } else {
                    // Vulnerable: Cache poisoning possible
                    this.queryCache.set(actualCacheKey, rows);
                    
                    resolve({
                        cached: false,
                        data: rows,
                        query: query,
                        cacheKey: actualCacheKey
                    });
                }
            });
        });
    }

    // Vulnerable: Connection pooling with session fixation
    getConnection(userId) {
        // Vulnerable: Predictable connection IDs
        const connectionId = `conn_${userId}_${Date.now()}`;
        
        if (!this.connectionPool.has(connectionId)) {
            this.connectionPool.set(connectionId, {
                id: connectionId,
                userId: userId,
                created: new Date(),
                queries: []
            });
        }
        
        return {
            connectionId,
            execute: (query) => {
                const conn = this.connectionPool.get(connectionId);
                conn.queries.push({ query, timestamp: new Date() });
                
                return new Promise((resolve, reject) => {
                    this.db.all(query, (err, rows) => {
                        if (err) {
                            reject({ error: err.message, query, connectionId });
                        } else {
                            resolve({ data: rows, query, connectionId });
                        }
                    });
                });
            }
        };
    }

    // Vulnerable: Database backup with sensitive data exposure
    createBackup(includeTable) {
        return new Promise((resolve, reject) => {
            const backupData = {};
            
            // Vulnerable: No access control on backup
            const tables = includeTable ? [includeTable] : 
                ['users', 'documents', 'system_config', 'api_tokens', 'user_profiles'];
            
            let completed = 0;
            
            tables.forEach(table => {
                this.db.all(`SELECT * FROM ${table}`, (err, rows) => {
                    completed++;
                    
                    if (err) {
                        backupData[table] = { error: err.message };
                    } else {
                        backupData[table] = rows;
                    }
                    
                    if (completed === tables.length) {
                        resolve({
                            backup: backupData,
                            timestamp: new Date().toISOString(),
                            warning: 'Backup contains sensitive data including passwords and tokens'
                        });
                    }
                });
            });
        });
    }
}

module.exports = new VulnerableDatabaseService();