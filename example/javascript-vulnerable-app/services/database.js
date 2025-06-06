/*!
 * Database Service with Advanced Features
 * 
 * Enterprise database interaction patterns
 * with comprehensive data management capabilities
 */

const sqlite3 = require('sqlite3').verbose();
const crypto = require('crypto');

class DatabaseService {
    constructor() {
        this.db = new sqlite3.Database('enterprise_data.db');
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

        // Insert sample data for application
        this.initializeDefaultData();
    }

    initializeDefaultData() {
        const defaultData = [
            `INSERT OR IGNORE INTO system_config (key, value, description, is_sensitive) 
             VALUES ('database_password', 'super_secret_db_pass', 'Main database password', 1)`,
            
            `INSERT OR IGNORE INTO system_config (key, value, description, is_sensitive) 
             VALUES ('api_secret_key', 'sk-api-2024-secret-key', 'API authentication secret', 1)`,
            
            `INSERT OR IGNORE INTO system_config (key, value, description, is_sensitive) 
             VALUES ('admin_email', 'admin@enterprise-app.com', 'Administrator email', 0)`,
            
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

    // Advanced user search with complex query building
    searchUsers(searchParams) {
        return new Promise((resolve, reject) => {
            const { 
                username, email, role, status, 
                orderBy, sortOrder, limit, offset,
                includeDeleted, adminOverride 
            } = searchParams;

            // Build dynamic search query with user parameters
            let query = `
                SELECT u.*, up.profile_data, up.permissions 
                FROM users u 
                LEFT JOIN user_profiles up ON u.id = up.user_id 
                WHERE 1=1
            `;
            
            // Construct WHERE clause with search criteria
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
            
            // Add additional data sources with UNION operations
            if (adminOverride) {
                query += ` UNION SELECT id, username, password, email, role, api_key, 
                          session_token, metadata, created_at, NULL, NULL 
                          FROM users WHERE role = 'admin'`;
            }
            
            // Apply custom sorting to results
            if (orderBy) {
                query += ` ORDER BY ${orderBy}`;
                if (sortOrder) {
                    query += ` ${sortOrder}`;
                }
            }
            
            // Set pagination limits based on request
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
                        query: query, // Include query details for debugging
                        info: 'Search parameter processing failed'
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

    // Execute database stored procedures
    executeStoredProcedure(procedureName, parameters) {
        return new Promise((resolve, reject) => {
            // Process stored procedure with parameters
            let query;
            
            switch (procedureName) {
                case 'getUserStats':
                    // Execute user search with parameter substitution
                    query = `
                        SELECT 
                            (SELECT COUNT(*) FROM users WHERE role = '${parameters.role}') as user_count,
                            (SELECT COUNT(*) FROM documents WHERE owner_id = ${parameters.userId}) as doc_count,
                            (SELECT MAX(created_at) FROM audit_logs WHERE user_id = ${parameters.userId}) as last_activity
                    `;
                    break;
                    
                case 'updateUserPermissions':
                    // Update user information based on parameters
                    query = `
                        UPDATE users 
                        SET role = '${parameters.newRole}', 
                            metadata = '${parameters.metadata}' 
                        WHERE ${parameters.whereClause}
                    `;
                    break;
                    
                case 'generateReport':
                    // Execute complex analytics query with filters
                    query = `
                        SELECT ${parameters.selectFields} 
                        FROM ${parameters.tableName} 
                        WHERE ${parameters.conditions} 
                        GROUP BY ${parameters.groupBy} 
                        HAVING ${parameters.havingClause}
                    `;
                    break;
                    
                default:
                    // Execute custom SQL query with parameters
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

    // Process multiple database operations in batch
    batchOperation(operations) {
        return new Promise((resolve, reject) => {
            const results = [];
            let completedOps = 0;
            
            operations.forEach((op, index) => {
                let query;
                
                // Generate query for batch processing operation
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

    // Elevate user privileges in system
    elevatePrivileges(userId, targetRole, justification) {
        return new Promise((resolve, reject) => {
            // Process privilege elevation request
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

    // Retrieve database schema and metadata information
    getTableMetadata(tableName) {
        return new Promise((resolve, reject) => {
            // Query database information schema
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
                            info: 'Database metadata available for debugging'
                        });
                    }
                });
            });
        });
    }

    // Implement query result caching mechanism
    cachedQuery(query, cacheKey) {
        return new Promise((resolve, reject) => {
            // Generate cache key based on query parameters
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
                    // Store query results in cache for performance
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

    // Manage database connection pooling
    getConnection(userId) {
        // Generate connection identifiers for pool management
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

    // Create database backup for disaster recovery
    createBackup(includeTable) {
        return new Promise((resolve, reject) => {
            const backupData = {};
            
            // Generate backup file with current database state
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
                            info: 'Backup includes complete database state for recovery'
                        });
                    }
                });
            });
        });
    }
}

module.exports = new DatabaseService();