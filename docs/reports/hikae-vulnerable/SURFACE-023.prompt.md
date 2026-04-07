You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-023
- **Kind**: endpoint
- **Identifier**: GET /logs?user_id=
- **Description**: IDOR in web audit log viewer — user_id parameter overrides session user, allowing authenticated users to view any user's audit logs. SQL injection in underlying get_user_logs() query.
- **Locations**: app.py, models.py

## Repository Context

## Directory Structure
```
README.md
api.py
app.py
models.py
requirements.txt

```

## Languages
- Python: 3 files

## Dependencies
### requirements.txt
```
flask==3.0.0
werkzeug==3.1.3
requests==2.31.0
PyJWT==2.8.0
PyYAML==6.0.1
Jinja2==3.1.2

```

## Entry Points
- app.py

Total source files: 3


## Source Code

### app.py
```py
"""
Enhanced Vulnerable Python Application
Multi-layered architecture with complex vulnerability patterns
"""
from flask import Flask, request, render_template_string, session, jsonify, redirect, url_for
import sqlite3
import os
import secrets
import hashlib
import jwt
from datetime import datetime, timedelta
from api import api_bp
from models import DatabaseManager, UserModel, DocumentModel, AuditLogger
import logging

# Configure logging
logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

app = Flask(__name__)

# Vulnerable: Weak session configuration
app.secret_key = "vulnerable_secret_key_123"  # Vulnerable: Hardcoded secret
app.config['SESSION_COOKIE_SECURE'] = False  # Vulnerable: No HTTPS requirement
app.config['SESSION_COOKIE_HTTPONLY'] = False  # Vulnerable: Accessible via JS

# Register API blueprint
app.register_blueprint(api_bp)

# Initialize models
db_manager = DatabaseManager()
user_model = UserModel(db_manager)
doc_model = DocumentModel(db_manager)
audit_logger = AuditLogger(db_manager)

@app.route("/")
def index():
    """Enhanced main page with navigation"""
    user = session.get('user')
    
    return render_template_string("""
    <!DOCTYPE html>
    <html>
    <head>
        <title>Advanced Vulnerable Application</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .nav { background: #f0f0f0; padding: 20px; margin-bottom: 20px; }
            .section { margin: 20px 0; padding: 20px; border: 1px solid #ddd; }
            .vuln-type { color: #d9534f; font-weight: bold; }
            .endpoint { background: #f5f5f5; padding: 10px; margin: 5px 0; }
            ul { list-style-type: none; }
            li { margin: 10px 0; }
            a { color: #337ab7; text-decoration: none; }
            a:hover { text-decoration: underline; }
        </style>
    </head>
    <body>
        <div class="nav">
            <h1>🔓 Advanced Vulnerable Application</h1>
            {% if user %}
                <p>Welcome, {{ user.username }}! | <a href="/logout">Logout</a></p>
            {% else %}
                <p><a href="/login">Login</a> to access more features</p>
            {% endif %}
        </div>
        
        <div class="section">
            <h2>Classic Web Vulnerabilities</h2>
            <ul>
                <li><a href="/sqli">🗃️ SQL Injection</a> - <span class="vuln-type">CWE-89</span></li>
                <li><a href="/xss">⚡ Cross-Site Scripting (XSS)</a> - <span class="vuln-type">CWE-79</span></li>
                <li><a href="/cmdi">💻 Command Injection</a> - <span class="vuln-type">CWE-78</span></li>
                <li><a href="/lfi">📁 Local File Inclusion</a> - <span class="vuln-type">CWE-22</span></li>
                <li><a href="/ssti">🎭 Server-Side Template Injection</a> - <span class="vuln-type">CWE-94</span></li>
            </ul>
        </div>

        <div class="section">
            <h2>API Vulnerabilities</h2>
            <div class="endpoint">
                <strong>Authentication:</strong>
                <ul>
                    <li>POST /api/auth/login - Vulnerable authentication</li>
                    <li>GET /api/user/&lt;id&gt; - IDOR vulnerability</li>
                </ul>
            </div>
            
            <div class="endpoint">
                <strong>Server-Side Request Forgery:</strong>
                <ul>
                    <li>POST /api/ssrf/fetch - SSRF via URL parameter</li>
                </ul>
            </div>
            
            <div class="endpoint">
                <strong>Injection Attacks:</strong>
                <ul>
                    <li>POST /api/xml/parse - XXE vulnerability</li>
                    <li>POST /api/yaml/load - YAML deserialization</li>
                    <li>POST /api/pickle/deserialize - Pickle deserialization</li>
                    <li>POST /api/eval/python - Code injection</li>
                    <li>POST /api/ldap/search - LDAP injection</li>
                </ul>
            </div>
            
            <div class="endpoint">
                <strong>File Operations:</strong>
                <ul>
                    <li>POST /api/file/upload - Unrestricted file upload</li>
                    <li>POST /api/file/extract - Zip slip vulnerability</li>
                    <li>GET /api/documents/&lt;id&gt;/content - Path traversal</li>
                </ul>
            </div>
            
            <div class="endpoint">
                <strong>Other Vulnerabilities:</strong>
                <ul>
                    <li>GET /api/redirect - Open redirect</li>
                    <li>POST /api/template/render - Template injection</li>
                    <li>POST /api/exec/command - Command execution</li>
                </ul>
            </div>
        </div>

        <div class="section">
            <h2>Documentation & Testing</h2>
            <ul>
                <li><a href="/docs">📚 API Documentation</a></li>
                <li><a href="/test">🧪 Vulnerability Test Suite</a></li>
                <li><a href="/logs">📊 Audit Logs</a> {% if not user %}(Login required){% endif %}</li>
            </ul>
        </div>
    </body>
    </html>
    """, user=user)

@app.route("/login", methods=['GET', 'POST'])
def login():
    """Vulnerable login with multiple attack vectors"""
    if request.method == 'POST':
        username = request.form.get('username', '')
        password = request.form.get('password', '')
        
        # Log the attempt (vulnerable - logs password)
        logger.info(f"Login attempt: {username}:{password} from {request.remote_addr}")
        
        user = user_model.authenticate_user(username, password)
        
        if user:
            # Vulnerable: Storing sensitive data in session
            session['user'] = user
            session['logged_in'] = True
            
            # Vulnerable: Predictable session tokens
            session['session_token'] = hashlib.md5(f"{username}{password}".encode()).hexdigest()
            
            audit_logger.log_action(
                user['id'], 
                'WEB_LOGIN', 
                f"Web login successful for {username}",
                request.remote_addr
            )
            
            return redirect(url_for('index'))
        else:
            # Vulnerable: Information disclosure in error messages
            error = f"Invalid credentials for user '{username}'"
            return render_template_string("""
                <h2>Login Failed</h2>
                <p style="color: red;">{{ error }}</p>
                <a href="/login">Try Again</a>
            """, error=error)
    
    return render_template_string("""
    <!DOCTYPE html>
    <html>
    <head><title>Login</title></head>
    <body style="font-family: Arial; margin: 40px;">
        <h2>Login</h2>
        <form method="post">
            <p>
                <label>Username:</label><br>
                <input type="text" name="username" value="">
            </p>
            <p>
                <label>Password:</label><br>
                <input type="password" name="password" value="">
            </p>
            <p>
                <input type="submit" value="Login">
            </p>
        </form>
        <p>Default credentials: admin/admin123 or guest/guest</p>
        <p><a href="/">Back to Home</a></p>
    </body>
    </html>
    """)

@app.route("/logout")
def logout():
    """Vulnerable logout (doesn't properly clear session)"""
    username = session.get('user', {}).get('username', 'unknown')
    
    # Vulnerable: Session fixation - doesn't regenerate session ID
    session.pop('user', None)
    session.pop('logged_in', None)
    # Vulnerable: Leaves session_token in session
    
    return redirect(url_for('index'))

@app.route("/sqli")
def sql_injection():
    """Enhanced SQL injection with multiple injection points"""
    username = request.args.get("username", "")
    order_by = request.args.get("order", "id")  # Additional injection point
    
    conn = sqlite3.connect("users.db")
    cursor = conn.cursor()
    
    # Multiple vulnerable queries
    try:
        # Main query with injection
        query1 = f"SELECT id, username, email, role FROM users WHERE username LIKE '%{username}%'"
        
        # Order by injection
        query2 = f"SELECT * FROM users ORDER BY {order_by}"
        
        results1 = cursor.execute(query1).fetchall()
        results2 = cursor.execute(query2).fetchall()
        
    except Exception as e:
        results1 = f"Error in main query: {e}"
        results2 = f"Error in order query: {e}"
    
    conn.close()
    
    return render_template_string("""
    <h2>Advanced SQL Injection Example</h2>
    <form>
        <p>
            Username Search: <input type="text" name="username" value="{{ username }}">
        </p>
        <p>
            Order By: 
            <select name="order">
                <option value="id" {{ 'selected' if order == 'id' else '' }}>ID</option>
                <option value="username" {{ 'selected' if order == 'username' else '' }}>Username</option>
                <option value="email" {{ 'selected' if order == 'email' else '' }}>Email</option>
            </select>
        </p>
        <input type="submit" value="Search">
    </form>
    
    <h3>Query 1 (LIKE injection):</h3>
    <pre>{{ query1 }}</pre>
    <pre>{{ results1 }}</pre>
    
    <h3>Query 2 (ORDER BY injection):</h3>
    <pre>{{ query2 }}</pre>
    <pre>{{ results2 }}</pre>
    
    <p><a href="/">Back to Home</a></p>
    """, 
    username=username, 
    order=order_by,
    query1=f"SELECT id, username, email, role FROM users WHERE username LIKE '%{username}%'",
    query2=f"SELECT * FROM users ORDER BY {order_by}",
    results1=results1, 
    results2=results2)

@app.route("/xss")
def xss():
    """Enhanced XSS with multiple contexts"""
    name = request.args.get("name", "")
    comment = request.args.get("comment", "")
    
    # Vulnerable template with multiple XSS contexts
    template = f"""
    <h2>XSS Playground</h2>
    
    <!-- Reflected XSS in HTML context -->
    <div>Hello, {name}!</div>
    
    <!-- XSS in attribute context -->
    <input type="text" value="{name}" placeholder="Your name">
    
    <!-- XSS in JavaScript context -->
    <script>
        var userName = '{name}';
        var userComment = '{comment}';
        console.log('User: ' + userName);
    </script>
    
    <!-- XSS in CSS context -->
    <style>
        .user-style {{ color: {comment}; }}
    </style>
    
    <form>
        <p>Name: <input type="text" name="name" value="{name}"></p>
        <p>Comment: <input type="text" name="comment" value="{comment}"></p>
        <input type="submit" value="Submit">
    </form>
    
    <h3>Stored Comments:</h3>
    <div class="user-style">Comment: {comment}</div>
    
    <p><a href="/">Back to Home</a></p>
    """
    
    return render_template_string(template)

@app.route("/cmdi")
def command_injection():
    """Enhanced command injection with multiple vectors"""
    hostname = request.args.get("hostname", "localhost")
    count = request.args.get("count", "1")
    
    # Multiple command injection points
    try:
        # Direct injection in command
        output1 = os.popen(f"ping -c {count} {hostname}").read()
        
        # Injection via filename
        temp_file = f"/tmp/ping_result_{hostname}.txt"
        os.system(f"ping -c 1 {hostname} > {temp_file}")
        
        with open(temp_file, 'r') as f:
            output2 = f.read()
            
    except Exception as e:
        output1 = f"Error: {e}"
        output2 = f"Error: {e}"
    
    return render_template_string("""
    <h2>Command Injection Playground</h2>
    <form>
        <p>Hostname: <input type="text" name="hostname" value="{{ hostname }}"></p>
        <p>Count: <input type="text" name="count" value="{{ count }}"></p>
        <input type="submit" value="Ping">
    </form>
    
    <h3>Direct Command Output:</h3>
    <pre>{{ output1 }}</pre>
    
    <h3>File-based Command Output:</h3>
    <pre>{{ output2 }}</pre>
    
    <p><a href="/">Back to Home</a></p>
    """, hostname=hostname, count=count, output1=output1, output2=output2)

@app.route("/lfi")
def local_file_inclusion():
    """Local File Inclusion vulnerability"""
    file_path = request.args.get("file", "")
    
    if file_path:
        try:
            # Vulnerable: No path validation
            with open(file_path, 'r') as f:
                content = f.read()
        except Exception as e:
            content = f"Error reading file: {e}"
    else:
        content = "No file specified"
    
    return render_template_string("""
    <h2>Local File Inclusion</h2>
    <form>
        <p>File Path: <input type="text" name="file" value="{{ file_path }}" size="50"></p>
        <input type="submit" value="Read File">
    </form>
    
    <h3>File Content:</h3>
    <pre>{{ content }}</pre>
    
    <p>Try: /etc/passwd, /etc/hosts, ../../etc/passwd</p>
    <p><a href="/">Back to Home</a></p>
    """, file_path=file_path, content=content)

@app.route("/ssti")
def server_side_template_injection():
    """Server-Side Template Injection"""
    template_input = request.args.get("template", "Hello {{ name }}!")
    name = request.args.get("name", "World")
    
    try:
        # Vulnerable: Direct template rendering
        from jinja2 import Template
        t = Template(template_input)
        result = t.render(name=name)
    except Exception as e:
        result = f"Template error: {e}"
    
    return render_template_string("""
    <h2>Server-Side Template Injection</h2>
    <form>
        <p>Template: <input type="text" name="template" value="{{ template_input }}" size="50"></p>
        <p>Name: <input type="text" name="name" value="{{ name }}"></p>
        <input type="submit" value="Render">
    </form>
    
    <h3>Rendered Output:</h3>
    <div style="border: 1px solid #ddd; padding: 10px;">{{ result|safe }}</div>
    
    <h3>Payload Examples:</h3>
    <ul>
        <li>{{ "{{ config }}" }}</li>
        <li>{{ "{{ request }}" }}</li>
        <li>{{ "{{ ''.__class__.__mro__[1].__subclasses__() }}" }}</li>
    </ul>
    
    <p><a href="/">Back to Home</a></p>
    """, template_input=template_input, name=name, result=result)

@app.route("/logs")
def view_logs():
    """Vulnerable log viewing (IDOR)"""
    if not session.get('logged_in'):
        return redirect(url_for('login'))
    
    user_id = request.args.get('user_id', session.get('user', {}).get('id', 1))
    
    try:
        logs = audit_logger.get_user_logs(user_id)
        return render_template_string("""
        <h2>Audit Logs</h2>
        <p>Viewing logs for user ID: {{ user_id }}</p>
        
        <table border="1" style="border-collapse: collapse;">
            <tr>
                <th>Timestamp</th>
                <th>Action</th>
                <th>Details</th>
                <th>IP Address</th>
            </tr>
            {% for log in logs %}
            <tr>
                <td>{{ log.timestamp }}</td>
                <td>{{ log.action }}</td>
                <td>{{ log.details }}</td>
                <td>{{ log.ip_address }}</td>
            </tr>
            {% endfor %}
        </table>
        
        <p><a href="/">Back to Home</a></p>
        """, user_id=user_id, logs=logs)
    except Exception as e:
        return f"Error loading logs: {e}"

if __name__ == "__main__":
    # Initialize database
    db_manager = DatabaseManager()
    
    # Vulnerable: Debug mode in production
    app.run(host='0.0.0.0', port=5000, debug=True)

```

### models.py
```py
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
```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-023.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
