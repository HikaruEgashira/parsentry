"""
Database models for the vulnerable application
Contains various vulnerable database interaction patterns
"""
import sqlite3
import hashlib
import pickle
import json
from typing import Optional, List, Dict
import logging

# Vulnerable: Using root logger without configuration
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

class DatabaseManager:
    def __init__(self, db_path: str = "users.db"):
        self.db_path = db_path
        self.init_database()
    
    def init_database(self):
        """Initialize database with vulnerable schema"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Vulnerable: Storing sensitive data in plain text
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                email TEXT,
                role TEXT DEFAULT 'user',
                api_key TEXT,
                session_token TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        """)
        
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT,
                owner_id INTEGER,
                file_path TEXT,
                metadata TEXT,
                FOREIGN KEY (owner_id) REFERENCES users (id)
            )
        """)
        
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS audit_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                action TEXT,
                details TEXT,
                ip_address TEXT,
                timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        """)
        
        # Insert default vulnerable data
        try:
            # Vulnerable: Hardcoded credentials
            cursor.execute("""
                INSERT OR IGNORE INTO users (username, password, email, role, api_key) 
                VALUES (?, ?, ?, ?, ?)
            """, ('admin', 'admin123', 'admin@example.com', 'admin', 'sk-1234567890abcdef'))
            
            cursor.execute("""
                INSERT OR IGNORE INTO users (username, password, email, role, api_key) 
                VALUES (?, ?, ?, ?, ?)
            """, ('guest', 'guest', 'guest@example.com', 'user', 'pk-0987654321fedcba'))
            
        except Exception as e:
            logger.error(f"Error inserting default data: {e}")
        
        conn.commit()
        conn.close()

class UserModel:
    def __init__(self, db_manager: DatabaseManager):
        self.db = db_manager
    
    def authenticate_user(self, username: str, password: str) -> Optional[Dict]:
        """Vulnerable authentication with SQL injection"""
        conn = sqlite3.connect(self.db.db_path)
        cursor = conn.cursor()
        
        # Vulnerable: SQL Injection via string formatting
        query = f"SELECT * FROM users WHERE username = '{username}' AND password = '{password}'"
        logger.debug(f"Executing query: {query}")  # Vulnerable: Logging sensitive data
        
        try:
            result = cursor.fetchone()
            conn.close()
            
            if result:
                return {
                    'id': result[0],
                    'username': result[1],
                    'email': result[3],
                    'role': result[4],
                    'api_key': result[5]  # Vulnerable: Exposing API key
                }
        except Exception as e:
            logger.error(f"Database error: {e}")  # Vulnerable: Information disclosure
            conn.close()
        
        return None
    
    def get_user_by_id(self, user_id: str) -> Optional[Dict]:
        """Vulnerable user lookup with potential injection"""
        conn = sqlite3.connect(self.db.db_path)
        cursor = conn.cursor()
        
        # Vulnerable: No input validation/sanitization
        query = f"SELECT * FROM users WHERE id = {user_id}"
        
        try:
            cursor.execute(query)
            result = cursor.fetchone()
            conn.close()
            
            if result:
                return {
                    'id': result[0],
                    'username': result[1],
                    'email': result[3],
                    'role': result[4]
                }
        except Exception as e:
            conn.close()
            raise e
        
        return None
    
    def update_user_preferences(self, user_id: int, preferences: str):
        """Vulnerable deserialization"""
        try:
            # Vulnerable: Unsafe deserialization
            prefs = pickle.loads(preferences.encode('latin1'))
            
            conn = sqlite3.connect(self.db.db_path)
            cursor = conn.cursor()
            
            # Store serialized preferences (vulnerable pattern)
            cursor.execute(
                "UPDATE users SET metadata = ? WHERE id = ?", 
                (preferences, user_id)
            )
            conn.commit()
            conn.close()
            
            return prefs
        except Exception as e:
            logger.error(f"Preference update failed: {e}")
            return None

class DocumentModel:
    def __init__(self, db_manager: DatabaseManager):
        self.db = db_manager
    
    def search_documents(self, query: str, user_id: int) -> List[Dict]:
        """Vulnerable document search with IDOR"""
        conn = sqlite3.connect(self.db.db_path)
        cursor = conn.cursor()
        
        # Vulnerable: No authorization check (IDOR)
        # Vulnerable: SQL Injection in LIKE clause
        sql = f"SELECT * FROM documents WHERE title LIKE '%{query}%'"
        
        try:
            cursor.execute(sql)
            results = cursor.fetchall()
            conn.close()
            
            documents = []
            for row in results:
                documents.append({
                    'id': row[0],
                    'title': row[1],
                    'content': row[2],
                    'owner_id': row[3],
                    'file_path': row[4]
                })
            
            return documents
        except Exception as e:
            conn.close()
            logger.error(f"Search failed: {e}")
            return []
    
    def get_document_content(self, doc_id: str) -> Optional[str]:
        """Vulnerable file inclusion"""
        conn = sqlite3.connect(self.db.db_path)
        cursor = conn.cursor()
        
        # Vulnerable: SQL injection + Path traversal
        query = f"SELECT file_path FROM documents WHERE id = {doc_id}"
        
        try:
            cursor.execute(query)
            result = cursor.fetchone()
            conn.close()
            
            if result and result[0]:
                file_path = result[0]
                # Vulnerable: No path validation (LFI)
                with open(file_path, 'r') as f:
                    return f.read()
        except Exception as e:
            logger.error(f"File read error: {e}")
        
        return None
    
    def save_document_metadata(self, doc_id: int, metadata: str):
        """Vulnerable JSON parsing"""
        try:
            # Vulnerable: No input validation on JSON
            parsed_metadata = json.loads(metadata)
            
            # Vulnerable: Potential for JSON injection
            conn = sqlite3.connect(self.db.db_path)
            cursor = conn.cursor()
            
            cursor.execute(
                "UPDATE documents SET metadata = ? WHERE id = ?",
                (metadata, doc_id)
            )
            conn.commit()
            conn.close()
            
            return parsed_metadata
        except json.JSONDecodeError as e:
            logger.error(f"JSON parsing error: {e}")
            return None

class AuditLogger:
    def __init__(self, db_manager: DatabaseManager):
        self.db = db_manager
    
    def log_action(self, user_id: int, action: str, details: str, ip_address: str):
        """Vulnerable audit logging with injection"""
        conn = sqlite3.connect(self.db.db_path)
        cursor = conn.cursor()
        
        # Vulnerable: SQL Injection in logging
        query = f"""
            INSERT INTO audit_logs (user_id, action, details, ip_address) 
            VALUES ({user_id}, '{action}', '{details}', '{ip_address}')
        """
        
        try:
            cursor.execute(query)
            conn.commit()
            conn.close()
        except Exception as e:
            logger.error(f"Audit logging failed: {e}")
            conn.close()
    
    def get_user_logs(self, user_id: str) -> List[Dict]:
        """Vulnerable log retrieval"""
        conn = sqlite3.connect(self.db.db_path)
        cursor = conn.cursor()
        
        # Vulnerable: No input validation + potential injection
        query = f"SELECT * FROM audit_logs WHERE user_id = {user_id} ORDER BY timestamp DESC"
        
        try:
            cursor.execute(query)
            results = cursor.fetchall()
            conn.close()
            
            logs = []
            for row in results:
                logs.append({
                    'id': row[0],
                    'action': row[2],
                    'details': row[3],
                    'ip_address': row[4],
                    'timestamp': row[5]
                })
            
            return logs
        except Exception as e:
            conn.close()
            logger.error(f"Log retrieval failed: {e}")
            return []