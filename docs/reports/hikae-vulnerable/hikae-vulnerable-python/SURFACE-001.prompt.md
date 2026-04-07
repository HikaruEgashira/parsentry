You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-001
- **Kind**: endpoint
- **Identifier**: Flask route handlers
- **Description**: HTTP endpoints defined in Flask application accepting untrusted user input. app.py is the entry point; api.py likely defines API routes. All request parameters, headers, and body data are potential injection vectors.
- **Locations**: app.py, api.py

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

### api.py
```py
"""
API endpoints with various vulnerability patterns
Demonstrates complex attack vectors and multi-layered vulnerabilities
"""
from flask import Blueprint, request, jsonify, session, redirect, send_file
import requests
import subprocess
import os
import xml.etree.ElementTree as ET
import yaml
import pickle
import base64
import hashlib
import jwt
from urllib.parse import urlparse, urljoin
import tempfile
import zipfile
from models import DatabaseManager, UserModel, DocumentModel, AuditLogger

api_bp = Blueprint('api', __name__, url_prefix='/api')

# Initialize models
db_manager = DatabaseManager()
user_model = UserModel(db_manager)
doc_model = DocumentModel(db_manager)
audit_logger = AuditLogger(db_manager)

# Vulnerable: Hardcoded secrets
JWT_SECRET = "super_secret_key_123"
API_KEYS = {
    "sk-1234567890abcdef": "admin",
    "pk-0987654321fedcba": "guest"
}

@api_bp.route('/auth/login', methods=['POST'])
def api_login():
    """Vulnerable authentication endpoint"""
    try:
        data = request.get_json()
        username = data.get('username', '')
        password = data.get('password', '')
        
        # Vulnerable: Authentication bypass
        user = user_model.authenticate_user(username, password)
        
        if user:
            # Vulnerable: Weak JWT implementation
            token = jwt.encode({
                'user_id': user['id'],
                'username': user['username'],
                'role': user['role']
            }, JWT_SECRET, algorithm='HS256')
            
            # Vulnerable: Logging sensitive information
            audit_logger.log_action(
                user['id'], 
                'LOGIN', 
                f"User {username} logged in with password {password}",  # Vulnerable!
                request.remote_addr
            )
            
            return jsonify({
                'token': token,
                'user': user,
                'api_key': user['api_key']  # Vulnerable: Exposing API key
            })
        else:
            return jsonify({'error': 'Invalid credentials'}), 401
            
    except Exception as e:
        # Vulnerable: Information disclosure in error messages
        return jsonify({'error': f'Authentication failed: {str(e)}'}), 500

@api_bp.route('/user/<user_id>', methods=['GET'])
def get_user(user_id):
    """Vulnerable user information endpoint (IDOR)"""
    try:
        # Vulnerable: No authorization check (IDOR)
        user = user_model.get_user_by_id(user_id)
        
        if user:
            return jsonify(user)
        else:
            return jsonify({'error': 'User not found'}), 404
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/documents/search', methods=['GET'])
def search_documents():
    """Vulnerable document search (SQL Injection + IDOR)"""
    query = request.args.get('q', '')
    user_id = request.args.get('user_id', 1)
    
    # Vulnerable: No input validation
    documents = doc_model.search_documents(query, int(user_id))
    
    return jsonify({'documents': documents})

@api_bp.route('/documents/<doc_id>/content', methods=['GET'])
def get_document_content(doc_id):
    """Vulnerable file inclusion endpoint (LFI)"""
    try:
        content = doc_model.get_document_content(doc_id)
        
        if content:
            return jsonify({'content': content})
        else:
            return jsonify({'error': 'Document not found'}), 404
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/ssrf/fetch', methods=['POST'])
def ssrf_fetch():
    """Server-Side Request Forgery vulnerability"""
    data = request.get_json()
    url = data.get('url', '')
    
    # Vulnerable: No URL validation (SSRF)
    try:
        # Vulnerable: Making requests to arbitrary URLs
        response = requests.get(url, timeout=10)
        
        return jsonify({
            'status_code': response.status_code,
            'content': response.text[:1000],  # Limit response size
            'headers': dict(response.headers)
        })
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/xml/parse', methods=['POST'])
def parse_xml():
    """XML External Entity (XXE) vulnerability"""
    xml_data = request.data.decode('utf-8')
    
    try:
        # Vulnerable: XXE attack vector
        parser = ET.XMLParser()
        root = ET.fromstring(xml_data, parser)
        
        # Process XML and return parsed content
        result = {}
        for child in root:
            result[child.tag] = child.text
        
        return jsonify({'parsed_xml': result})
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/yaml/load', methods=['POST'])
def load_yaml():
    """YAML deserialization vulnerability"""
    yaml_data = request.data.decode('utf-8')
    
    try:
        # Vulnerable: Unsafe YAML loading
        parsed_data = yaml.load(yaml_data, Loader=yaml.FullLoader)
        
        return jsonify({'parsed_yaml': parsed_data})
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/pickle/deserialize', methods=['POST'])
def deserialize_pickle():
    """Pickle deserialization vulnerability"""
    data = request.get_json()
    pickle_data = data.get('data', '')
    
    try:
        # Vulnerable: Unsafe pickle deserialization
        decoded = base64.b64decode(pickle_data)
        obj = pickle.loads(decoded)
        
        return jsonify({'deserialized': str(obj)})
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/exec/command', methods=['POST'])
def execute_command():
    """Command injection vulnerability"""
    data = request.get_json()
    command = data.get('command', '')
    
    # Vulnerable: Direct command execution
    try:
        result = subprocess.run(
            command, 
            shell=True, 
            capture_output=True, 
            text=True,
            timeout=10
        )
        
        return jsonify({
            'stdout': result.stdout,
            'stderr': result.stderr,
            'return_code': result.returncode
        })
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/file/upload', methods=['POST'])
def upload_file():
    """Vulnerable file upload"""
    if 'file' not in request.files:
        return jsonify({'error': 'No file uploaded'}), 400
    
    file = request.files['file']
    
    if file.filename == '':
        return jsonify({'error': 'No file selected'}), 400
    
    # Vulnerable: No file type validation
    upload_dir = '/tmp/uploads'
    os.makedirs(upload_dir, exist_ok=True)
    
    # Vulnerable: Path traversal via filename
    file_path = os.path.join(upload_dir, file.filename)
    file.save(file_path)
    
    return jsonify({
        'message': 'File uploaded successfully',
        'path': file_path,
        'filename': file.filename
    })

@api_bp.route('/file/extract', methods=['POST'])
def extract_archive():
    """Zip slip vulnerability"""
    if 'archive' not in request.files:
        return jsonify({'error': 'No archive uploaded'}), 400
    
    archive = request.files['archive']
    
    # Create temporary directory
    extract_dir = tempfile.mkdtemp()
    
    try:
        # Save uploaded archive
        archive_path = os.path.join(extract_dir, 'archive.zip')
        archive.save(archive_path)
        
        # Vulnerable: Zip slip attack - no path validation
        with zipfile.ZipFile(archive_path, 'r') as zip_ref:
            zip_ref.extractall(extract_dir)
        
        # List extracted files
        extracted_files = []
        for root, dirs, files in os.walk(extract_dir):
            for file in files:
                if file != 'archive.zip':
                    extracted_files.append(os.path.join(root, file))
        
        return jsonify({
            'message': 'Archive extracted successfully',
            'extracted_files': extracted_files,
            'extract_dir': extract_dir
        })
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/redirect', methods=['GET'])
def open_redirect():
    """Open redirect vulnerability"""
    next_url = request.args.get('next', '/')
    
    # Vulnerable: No URL validation (Open Redirect)
    return redirect(next_url)

@api_bp.route('/eval/python', methods=['POST'])
def eval_python():
    """Code injection vulnerability"""
    data = request.get_json()
    code = data.get('code', '')
    
    try:
        # Vulnerable: Direct code evaluation
        result = eval(code)
        return jsonify({'result': str(result)})
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/template/render', methods=['POST'])
def render_template():
    """Server-Side Template Injection (SSTI)"""
    data = request.get_json()
    template = data.get('template', '')
    context = data.get('context', {})
    
    try:
        # Vulnerable: Template injection via string formatting
        from jinja2 import Template
        
        # Vulnerable: No sandboxing
        t = Template(template)
        rendered = t.render(**context)
        
        return jsonify({'rendered': rendered})
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@api_bp.route('/ldap/search', methods=['POST'])
def ldap_search():
    """LDAP Injection vulnerability"""
    data = request.get_json()
    username = data.get('username', '')
    
    # Simulate LDAP query construction
    # Vulnerable: LDAP injection
    ldap_query = f"(&(objectClass=user)(cn={username}))"
    
    # Simulate LDAP search (would normally connect to LDAP server)
    return jsonify({
        'ldap_query': ldap_query,
        'message': 'LDAP search simulated (vulnerable to injection)',
        'username': username
    })

@api_bp.route('/logs/<user_id>', methods=['GET'])
def get_user_logs(user_id):
    """Vulnerable log access (IDOR + Injection)"""
    try:
        logs = audit_logger.get_user_logs(user_id)
        return jsonify({'logs': logs})
    except Exception as e:
        return jsonify({'error': str(e)}), 500
```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0
