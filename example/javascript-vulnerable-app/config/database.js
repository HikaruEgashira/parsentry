/*!
 * Database Configuration and Initialization
 * 
 * Contains database setup with vulnerable default data
 */

const sqlite3 = require('sqlite3').verbose();
const { DATABASE_CONFIG } = require('./constants');

class DatabaseInitializer {
    constructor() {
        this.db = new sqlite3.Database(DATABASE_CONFIG.PATH);
    }

    // Initialize all database tables
    async initializeTables() {
        const tables = [
            this.createUsersTable(),
            this.createDocumentsTable(),
            this.createAuditLogsTable(),
            this.createUserProfilesTable(),
            this.createFileMetadataTable(),
            this.createSystemConfigTable(),
            this.createApiTokensTable(),
            this.createCommentsTable()
        ];

        return Promise.all(tables);
    }

    createUsersTable() {
        return new Promise((resolve, reject) => {
            const sql = `CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                email TEXT,
                role TEXT DEFAULT 'user',
                api_key TEXT,
                session_token TEXT,
                metadata TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    createDocumentsTable() {
        return new Promise((resolve, reject) => {
            const sql = `CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT,
                owner_id INTEGER,
                file_path TEXT,
                metadata TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (owner_id) REFERENCES users (id)
            )`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    createAuditLogsTable() {
        return new Promise((resolve, reject) => {
            const sql = `CREATE TABLE IF NOT EXISTS audit_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                action TEXT,
                details TEXT,
                ip_address TEXT,
                user_agent TEXT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    createUserProfilesTable() {
        return new Promise((resolve, reject) => {
            const sql = `CREATE TABLE IF NOT EXISTS user_profiles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                profile_data TEXT,
                permissions TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    createFileMetadataTable() {
        return new Promise((resolve, reject) => {
            const sql = `CREATE TABLE IF NOT EXISTS file_metadata (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                filename TEXT,
                file_path TEXT,
                owner_id INTEGER,
                access_level TEXT DEFAULT 'private',
                mime_type TEXT,
                file_size INTEGER,
                checksum TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    createSystemConfigTable() {
        return new Promise((resolve, reject) => {
            const sql = `CREATE TABLE IF NOT EXISTS system_config (
                key TEXT PRIMARY KEY,
                value TEXT,
                description TEXT,
                is_sensitive BOOLEAN DEFAULT 0
            )`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    createApiTokensTable() {
        return new Promise((resolve, reject) => {
            const sql = `CREATE TABLE IF NOT EXISTS api_tokens (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                token TEXT UNIQUE,
                user_id INTEGER,
                permissions TEXT,
                expires_at DATETIME,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    createCommentsTable() {
        return new Promise((resolve, reject) => {
            const sql = `CREATE TABLE IF NOT EXISTS comments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT,
                author TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    // Insert vulnerable default data
    async insertDefaultData() {
        const inserts = [
            this.insertDefaultUsers(),
            this.insertDefaultDocuments(),
            this.insertDefaultConfig(),
            this.insertDefaultTokens(),
            this.insertDefaultProfiles()
        ];

        return Promise.all(inserts);
    }

    insertDefaultUsers() {
        return new Promise((resolve, reject) => {
            const users = [
                `INSERT OR IGNORE INTO users (username, password, email, role, api_key) 
                 VALUES ('admin', 'admin123', 'admin@example.com', 'admin', 'sk-js-1234567890abcdef')`,
                `INSERT OR IGNORE INTO users (username, password, email, role, api_key) 
                 VALUES ('guest', 'guest', 'guest@example.com', 'user', 'pk-js-0987654321fedcba')`
            ];

            let completed = 0;
            users.forEach(sql => {
                this.db.run(sql, (err) => {
                    if (err) reject(err);
                    completed++;
                    if (completed === users.length) resolve();
                });
            });
        });
    }

    insertDefaultDocuments() {
        return new Promise((resolve, reject) => {
            const docs = [
                `INSERT OR IGNORE INTO documents (title, content, owner_id, file_path) 
                 VALUES ('Secret Config', 'database_password=super_secret_123', 1, '/etc/passwd')`,
                `INSERT OR IGNORE INTO documents (title, content, owner_id, file_path) 
                 VALUES ('User Data', 'Sensitive user information', 2, '../../etc/shadow')`
            ];

            let completed = 0;
            docs.forEach(sql => {
                this.db.run(sql, (err) => {
                    if (err) reject(err);
                    completed++;
                    if (completed === docs.length) resolve();
                });
            });
        });
    }

    insertDefaultConfig() {
        return new Promise((resolve, reject) => {
            const configs = [
                `INSERT OR IGNORE INTO system_config (key, value, description, is_sensitive) 
                 VALUES ('database_password', 'super_secret_db_pass', 'Main database password', 1)`,
                `INSERT OR IGNORE INTO system_config (key, value, description, is_sensitive) 
                 VALUES ('api_secret_key', 'sk-api-2024-secret-key', 'API authentication secret', 1)`
            ];

            let completed = 0;
            configs.forEach(sql => {
                this.db.run(sql, (err) => {
                    if (err) reject(err);
                    completed++;
                    if (completed === configs.length) resolve();
                });
            });
        });
    }

    insertDefaultTokens() {
        return new Promise((resolve, reject) => {
            const sql = `INSERT OR IGNORE INTO api_tokens (token, user_id, permissions, expires_at) 
                        VALUES ('admin_token_2024_secret', 1, 'admin,read,write,delete', datetime('now', '+1 year'))`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    insertDefaultProfiles() {
        return new Promise((resolve, reject) => {
            const sql = `INSERT OR IGNORE INTO user_profiles (user_id, profile_data, permissions) 
                        VALUES (1, '{"role":"admin","clearance":"top_secret"}', 'all')`;
            
            this.db.run(sql, (err) => {
                if (err) reject(err);
                else resolve();
            });
        });
    }

    getDatabase() {
        return this.db;
    }
}

module.exports = DatabaseInitializer;