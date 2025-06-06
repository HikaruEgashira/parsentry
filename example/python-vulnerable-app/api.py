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