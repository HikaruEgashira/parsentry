/*!
 * Vulnerable Rust Application Library
 * 
 * Contains database models and vulnerable utility functions
 * FOR TESTING PURPOSES ONLY - Contains intentional security vulnerabilities
 */

use rusqlite::{Connection, Result as SqliteResult, Row};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use log::{debug, error, warn};

/// User model with vulnerable patterns
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String, // Vulnerable: Password stored in plain text
    pub email: Option<String>,
    pub role: String,
    pub api_key: Option<String>,
    pub session_token: Option<String>,
    pub metadata: Option<String>, // Vulnerable: Serialized data storage
}

/// Document model for file operations
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub owner_id: i32,
    pub file_path: Option<String>,
    pub metadata: Option<String>,
}

/// Audit log entry
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i32,
    pub user_id: i32,
    pub action: String,
    pub details: String,
    pub ip_address: String,
    pub timestamp: String,
}

/// Database manager with vulnerable operations
pub struct DatabaseManager {
    db_path: String,
}

impl DatabaseManager {
    pub fn new(db_path: &str) -> Self {
        let manager = DatabaseManager {
            db_path: db_path.to_string(),
        };
        manager.init_database().expect("Failed to initialize database");
        manager
    }

    /// Initialize database with vulnerable schema
    pub fn init_database(&self) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        
        // Vulnerable: No encryption, weak schema design
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                email TEXT,
                role TEXT DEFAULT 'user',
                api_key TEXT,
                session_token TEXT,
                metadata TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT,
                owner_id INTEGER,
                file_path TEXT,
                metadata TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (owner_id) REFERENCES users (id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS audit_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                action TEXT,
                details TEXT,
                ip_address TEXT,
                timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Insert vulnerable default data
        conn.execute(
            "INSERT OR IGNORE INTO users (username, password, email, role, api_key) 
             VALUES ('admin', 'admin123', 'admin@example.com', 'admin', 'sk-rust-1234567890abcdef')",
            [],
        )?;

        conn.execute(
            "INSERT OR IGNORE INTO users (username, password, email, role, api_key) 
             VALUES ('guest', 'guest', 'guest@example.com', 'user', 'pk-rust-0987654321fedcba')",
            [],
        )?;

        // Insert sample documents with vulnerable paths
        conn.execute(
            "INSERT OR IGNORE INTO documents (title, content, owner_id, file_path) 
             VALUES ('Secret Config', 'database_password=super_secret_123', 1, '/etc/passwd')",
            [],
        )?;

        conn.execute(
            "INSERT OR IGNORE INTO documents (title, content, owner_id, file_path) 
             VALUES ('User Data', 'Sensitive user information', 2, '../../etc/shadow')",
            [],
        )?;

        Ok(())
    }

    /// Vulnerable authentication - SQL injection possible
    pub fn authenticate_user(&self, username: &str, password: &str) -> SqliteResult<Option<User>> {
        let conn = Connection::open(&self.db_path)?;
        
        // Vulnerable: SQL injection via string interpolation
        let query = format!(
            "SELECT * FROM users WHERE username = '{}' AND password = '{}'",
            username, password
        );
        
        // Vulnerable: Logging sensitive information
        debug!("Executing authentication query: {}", query);
        debug!("Attempting login with credentials: {}:{}", username, password);

        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query_map([], |row| self.row_to_user(row))?;

        match rows.next() {
            Some(Ok(user)) => Ok(Some(user)),
            Some(Err(e)) => {
                error!("Database error during authentication: {}", e);
                Err(e)
            }
            None => Ok(None),
        }
    }

    /// Vulnerable user lookup - injection possible
    pub fn get_user_by_id(&self, user_id: &str) -> SqliteResult<Option<User>> {
        let conn = Connection::open(&self.db_path)?;
        
        // Vulnerable: No input validation, SQL injection possible
        let query = format!("SELECT * FROM users WHERE id = {}", user_id);
        
        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query_map([], |row| self.row_to_user(row))?;

        match rows.next() {
            Some(Ok(user)) => Ok(Some(user)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    /// Vulnerable document search with IDOR
    pub fn search_documents(&self, query: &str, _user_id: i32) -> SqliteResult<Vec<Document>> {
        let conn = Connection::open(&self.db_path)?;
        
        // Vulnerable: No authorization check (IDOR) + SQL injection
        let sql = format!("SELECT * FROM documents WHERE title LIKE '%{}%'", query);
        
        let mut stmt = conn.prepare(&sql)?;
        let document_iter = stmt.query_map([], |row| self.row_to_document(row))?;

        let mut documents = Vec::new();
        for document in document_iter {
            if let Ok(doc) = document {
                documents.push(doc);
            }
        }

        Ok(documents)
    }

    /// Vulnerable file content retrieval
    pub fn get_document_content(&self, doc_id: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        
        // Vulnerable: SQL injection + path traversal
        let query = format!("SELECT file_path FROM documents WHERE id = {}", doc_id);
        
        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query_map([], |row| {
            Ok(row.get::<_, Option<String>>(0)?)
        })?;

        if let Some(Ok(Some(file_path))) = rows.next() {
            // Vulnerable: No path validation (LFI/path traversal)
            match fs::read_to_string(&file_path) {
                Ok(content) => Ok(Some(content)),
                Err(e) => {
                    error!("Failed to read file {}: {}", file_path, e);
                    Err(Box::new(e))
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Vulnerable audit logging with injection
    pub fn log_action(&self, user_id: i32, action: &str, details: &str, ip_address: &str) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        
        // Vulnerable: SQL injection in logging
        let query = format!(
            "INSERT INTO audit_logs (user_id, action, details, ip_address) 
             VALUES ({}, '{}', '{}', '{}')",
            user_id, action, details, ip_address
        );
        
        // Vulnerable: Logging sensitive data
        debug!("Audit log query: {}", query);
        
        conn.execute(&query, [])?;
        Ok(())
    }

    /// Vulnerable log retrieval
    pub fn get_user_logs(&self, user_id: &str) -> SqliteResult<Vec<AuditLog>> {
        let conn = Connection::open(&self.db_path)?;
        
        // Vulnerable: No input validation + potential injection
        let query = format!(
            "SELECT * FROM audit_logs WHERE user_id = {} ORDER BY timestamp DESC",
            user_id
        );
        
        let mut stmt = conn.prepare(&query)?;
        let log_iter = stmt.query_map([], |row| self.row_to_audit_log(row))?;

        let mut logs = Vec::new();
        for log in log_iter {
            if let Ok(audit_log) = log {
                logs.push(audit_log);
            }
        }

        Ok(logs)
    }

    /// Helper: Convert database row to User
    fn row_to_user(&self, row: &Row) -> SqliteResult<User> {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            email: row.get(3)?,
            role: row.get(4)?,
            api_key: row.get(5)?,
            session_token: row.get(6)?,
            metadata: row.get(7)?,
        })
    }

    /// Helper: Convert database row to Document
    fn row_to_document(&self, row: &Row) -> SqliteResult<Document> {
        Ok(Document {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            owner_id: row.get(3)?,
            file_path: row.get(4)?,
            metadata: row.get(5)?,
        })
    }

    /// Helper: Convert database row to AuditLog
    fn row_to_audit_log(&self, row: &Row) -> SqliteResult<AuditLog> {
        Ok(AuditLog {
            id: row.get(0)?,
            user_id: row.get(1)?,
            action: row.get(2)?,
            details: row.get(3)?,
            ip_address: row.get(4)?,
            timestamp: row.get(5)?,
        })
    }
}

/// Vulnerable utility functions

/// Vulnerable: Unsafe deserialization
pub fn deserialize_user_preferences(data: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    // Vulnerable: Using bincode without validation
    let decoded = base64::decode(data)?;
    let preferences: HashMap<String, String> = bincode::deserialize(&decoded)?;
    Ok(preferences)
}

/// Vulnerable: Command execution
pub fn execute_system_command(command: &str) -> Result<String, std::io::Error> {
    // Vulnerable: Direct command execution without validation
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Vulnerable: Path validation bypass
pub fn read_file_content(file_path: &str) -> Result<String, std::io::Error> {
    // Vulnerable: No path validation
    warn!("Reading file: {}", file_path);
    fs::read_to_string(file_path)
}

/// Vulnerable: Weak token generation
pub fn generate_session_token(username: &str, password: &str) -> String {
    use sha2::{Sha256, Digest};
    
    // Vulnerable: Predictable token generation
    let mut hasher = Sha256::new();
    hasher.update(format!("{}:{}", username, password));
    format!("{:x}", hasher.finalize())
}

/// Vulnerable: No input validation for XML
pub fn parse_xml_content(xml_data: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    use xml::reader::{EventReader, XmlEvent};
    
    let mut result = HashMap::new();
    let parser = EventReader::from_str(xml_data);
    
    // Vulnerable: XXE susceptible parser
    for event in parser {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                result.insert("element".to_string(), name.local_name);
            }
            XmlEvent::Characters(data) => {
                result.insert("content".to_string(), data);
            }
            _ => {}
        }
    }
    
    Ok(result)
}

/// Vulnerable: Unsafe YAML loading
pub fn parse_yaml_content(yaml_data: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    use yaml_rust::YamlLoader;
    
    // Vulnerable: Direct YAML parsing without sanitization
    let docs = YamlLoader::load_from_str(yaml_data)?;
    
    if let Some(doc) = docs.first() {
        // Convert YAML to JSON (simplified)
        let json_str = format!("{:?}", doc);
        Ok(serde_json::Value::String(json_str))
    } else {
        Ok(serde_json::Value::Null)
    }
}

/// Vulnerable: Directory traversal helper
pub fn get_file_listing(directory: &str) -> Result<Vec<String>, std::io::Error> {
    // Vulnerable: No path validation
    let mut files = Vec::new();
    
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(filename) = path.file_name() {
            files.push(filename.to_string_lossy().to_string());
        }
    }
    
    Ok(files)
}