/*!
 * Advanced Vulnerable Rust Application
 * 
 * A sophisticated, intentionally vulnerable Rust web application designed for testing
 * advanced security analysis tools. Features enterprise-level complexity with
 * multi-layered architecture and complex vulnerability patterns.
 * 
 * ⚠️ FOR TESTING PURPOSES ONLY - Contains severe security vulnerabilities
 */

use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, middleware::Logger};
use futures_util::TryStreamExt;
use log::{info, warn, error, debug};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

mod lib;
use lib::*;

// Vulnerable: Hardcoded secrets
const JWT_SECRET: &str = "super_secret_rust_key_123";
const API_KEYS: &[(&str, &str)] = &[
    ("sk-rust-1234567890abcdef", "admin"),
    ("pk-rust-0987654321fedcba", "guest"),
];

/// Request/Response structures
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: User,
    api_key: Option<String>,
}

#[derive(Deserialize)]
struct CommandRequest {
    command: String,
}

#[derive(Deserialize)]
struct UrlRequest {
    url: String,
}

#[derive(Deserialize)]
struct CodeRequest {
    code: String,
}

#[derive(Deserialize)]
struct TemplateRequest {
    template: String,
    context: HashMap<String, String>,
}

#[derive(Deserialize)]
struct DataRequest {
    data: String,
}

#[derive(Deserialize)]
struct LdapRequest {
    username: String,
}

/// Enhanced main page with comprehensive vulnerability showcase
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>🔓 Advanced Vulnerable Rust Application</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }
            .container { max-width: 1200px; margin: 0 auto; background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
            .header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; margin: -30px -30px 30px -30px; border-radius: 8px 8px 0 0; }
            .section { margin: 30px 0; padding: 25px; border: 2px solid #e0e0e0; border-radius: 8px; background: #fafafa; }
            .section h2 { color: #333; margin-top: 0; border-bottom: 2px solid #667eea; padding-bottom: 10px; }
            .vuln-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
            .vuln-card { background: white; padding: 20px; border-radius: 8px; border-left: 4px solid #e74c3c; }
            .vuln-title { font-weight: bold; color: #e74c3c; margin-bottom: 10px; }
            .vuln-desc { color: #666; margin-bottom: 15px; }
            .endpoint { background: #f8f9fa; padding: 15px; margin: 10px 0; border-radius: 5px; border-left: 3px solid #28a745; }
            .endpoint-method { display: inline-block; background: #007bff; color: white; padding: 4px 8px; border-radius: 3px; font-size: 0.9em; margin-right: 10px; }
            .post { background: #28a745; }
            .get { background: #007bff; }
            ul { list-style: none; padding: 0; }
            li { margin: 8px 0; }
            a { color: #667eea; text-decoration: none; font-weight: 500; }
            a:hover { text-decoration: underline; color: #764ba2; }
            .warning { background: #fff3cd; border: 1px solid #ffeaa7; color: #856404; padding: 15px; border-radius: 5px; margin: 20px 0; }
            .cwe { background: #dc3545; color: white; padding: 2px 6px; border-radius: 3px; font-size: 0.8em; }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <h1>🔓 Advanced Vulnerable Rust Application</h1>
                <p>Enterprise-level security testing platform with complex vulnerability patterns</p>
            </div>
            
            <div class="warning">
                <strong>⚠️ SECURITY WARNING:</strong> This application contains severe security vulnerabilities by design.
                Use only in isolated testing environments. DO NOT expose to public networks.
            </div>
            
            <div class="section">
                <h2>🌐 Classic Web Vulnerabilities</h2>
                <div class="vuln-grid">
                    <div class="vuln-card">
                        <div class="vuln-title">SQL Injection <span class="cwe">CWE-89</span></div>
                        <div class="vuln-desc">Multiple injection points with complex queries</div>
                        <div class="endpoint">
                            <span class="endpoint-method get">GET</span>
                            <a href="/sqli?username=admin&order=id">/sqli</a>
                        </div>
                        <div class="endpoint">
                            <span class="endpoint-method get">GET</span>
                            <a href="/api/documents/search?q=test&user_id=1">/api/documents/search</a>
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">Command Injection <span class="cwe">CWE-78</span></div>
                        <div class="vuln-desc">System command execution vulnerabilities</div>
                        <div class="endpoint">
                            <span class="endpoint-method get">GET</span>
                            <a href="/cmdi?hostname=localhost&count=1">/cmdi</a>
                        </div>
                        <div class="endpoint">
                            <span class="endpoint-method post">POST</span>
                            /api/exec/command
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">Path Traversal <span class="cwe">CWE-22</span></div>
                        <div class="vuln-desc">File inclusion and directory traversal</div>
                        <div class="endpoint">
                            <span class="endpoint-method get">GET</span>
                            <a href="/file?name=README.md">/file</a>
                        </div>
                        <div class="endpoint">
                            <span class="endpoint-method get">GET</span>
                            <a href="/api/documents/1/content">/api/documents/*/content</a>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>🔐 Authentication & Authorization</h2>
                <div class="vuln-grid">
                    <div class="vuln-card">
                        <div class="vuln-title">Authentication Bypass</div>
                        <div class="vuln-desc">Weak authentication mechanisms</div>
                        <div class="endpoint">
                            <span class="endpoint-method post">POST</span>
                            /api/auth/login
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">IDOR <span class="cwe">CWE-639</span></div>
                        <div class="vuln-desc">Insecure Direct Object References</div>
                        <div class="endpoint">
                            <span class="endpoint-method get">GET</span>
                            <a href="/api/user/1">/api/user/*</a>
                        </div>
                        <div class="endpoint">
                            <span class="endpoint-method get">GET</span>
                            <a href="/api/logs/1">/api/logs/*</a>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>🌍 Network & Injection Attacks</h2>
                <div class="vuln-grid">
                    <div class="vuln-card">
                        <div class="vuln-title">SSRF <span class="cwe">CWE-918</span></div>
                        <div class="vuln-desc">Server-Side Request Forgery</div>
                        <div class="endpoint">
                            <span class="endpoint-method post">POST</span>
                            /api/ssrf/fetch
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">XXE <span class="cwe">CWE-611</span></div>
                        <div class="vuln-desc">XML External Entity injection</div>
                        <div class="endpoint">
                            <span class="endpoint-method post">POST</span>
                            /api/xml/parse
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">Deserialization <span class="cwe">CWE-502</span></div>
                        <div class="vuln-desc">Unsafe object deserialization</div>
                        <div class="endpoint">
                            <span class="endpoint-method post">POST</span>
                            /api/deserialize
                        </div>
                        <div class="endpoint">
                            <span class="endpoint-method post">POST</span>
                            /api/yaml/load
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">LDAP Injection <span class="cwe">CWE-90</span></div>
                        <div class="vuln-desc">Directory service query injection</div>
                        <div class="endpoint">
                            <span class="endpoint-method post">POST</span>
                            /api/ldap/search
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>📁 File Operation Vulnerabilities</h2>
                <div class="vuln-grid">
                    <div class="vuln-card">
                        <div class="vuln-title">Unrestricted Upload</div>
                        <div class="vuln-desc">File upload without validation</div>
                        <div class="endpoint">
                            <span class="endpoint-method post">POST</span>
                            /api/file/upload
                        </div>
                    </div>
                    
                    <div class="vuln-card">
                        <div class="vuln-title">Directory Listing</div>
                        <div class="vuln-desc">Unauthorized directory access</div>
                        <div class="endpoint">
                            <span class="endpoint-method get">GET</span>
                            <a href="/api/files/list?dir=/etc">/api/files/list</a>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>🧪 Testing & Documentation</h2>
                <ul>
                    <li><a href="/api/docs">📚 API Documentation</a></li>
                    <li><a href="/test">🧪 Vulnerability Test Suite</a></li>
                    <li><a href="/logs">📊 Audit Logs</a></li>
                    <li><a href="/metrics">📈 Security Metrics</a></li>
                </ul>
            </div>
        </div>
    </body>
    </html>
    "#)
}

/// Enhanced SQL injection with multiple attack vectors
#[get("/sqli")]
async fn sql_injection(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let db = DatabaseManager::new("users.db");
    
    let username = query.get("username").cloned().unwrap_or_default();
    let order_by = query.get("order").cloned().unwrap_or_else(|| "id".to_string());
    
    // Multiple vulnerable queries
    let result1 = db.authenticate_user(&username, "");
    let result2 = db.get_user_by_id(&format!("1 UNION SELECT * FROM users ORDER BY {}", order_by));
    
    HttpResponse::Ok().json(json!({
        "query1_result": result1.ok(),
        "query2_result": result2.ok(),
        "vulnerable_queries": [
            format!("SELECT * FROM users WHERE username = '{}'", username),
            format!("SELECT * FROM users ORDER BY {}", order_by)
        ]
    }))
}

/// Enhanced command injection with multiple vectors
#[get("/cmdi")]
async fn command_injection(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let hostname = query.get("hostname").cloned().unwrap_or_else(|| "localhost".to_string());
    let count = query.get("count").cloned().unwrap_or_else(|| "1".to_string());
    
    // Multiple command injection vectors
    let command1 = format!("ping -c {} {}", count, hostname);
    let command2 = format!("nslookup {}", hostname);
    
    let output1 = execute_system_command(&command1).unwrap_or_else(|e| format!("Error: {}", e));
    let output2 = execute_system_command(&command2).unwrap_or_else(|e| format!("Error: {}", e));
    
    HttpResponse::Ok().json(json!({
        "command1": command1,
        "output1": output1,
        "command2": command2,
        "output2": output2
    }))
}

/// Path traversal with enhanced file access
#[get("/file")]
async fn file_read(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let filename = query.get("name").cloned().unwrap_or_else(|| "README.md".to_string());
    
    match read_file_content(&filename) {
        Ok(content) => HttpResponse::Ok().json(json!({
            "filename": filename,
            "content": content,
            "size": content.len()
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to read file {}: {}", filename, e),
            "filename": filename
        }))
    }
}

/// Vulnerable authentication endpoint
#[post("/api/auth/login")]
async fn api_login(req: web::Json<LoginRequest>) -> impl Responder {
    let db = DatabaseManager::new("users.db");
    
    match db.authenticate_user(&req.username, &req.password) {
        Ok(Some(user)) => {
            // Vulnerable: Weak token generation
            let token = generate_session_token(&req.username, &req.password);
            
            // Vulnerable: Log sensitive information
            let _ = db.log_action(
                user.id,
                "LOGIN",
                &format!("User {} logged in with password {}", req.username, req.password),
                "127.0.0.1"
            );
            
            HttpResponse::Ok().json(LoginResponse {
                token,
                user: user.clone(),
                api_key: user.api_key,
            })
        }
        Ok(None) => HttpResponse::Unauthorized().json(json!({
            "error": format!("Invalid credentials for user '{}'", req.username)
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Authentication failed: {}", e)
        }))
    }
}

/// Vulnerable user lookup (IDOR)
#[get("/api/user/{user_id}")]
async fn get_user(path: web::Path<String>) -> impl Responder {
    let db = DatabaseManager::new("users.db");
    let user_id = path.into_inner();
    
    match db.get_user_by_id(&user_id) {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// Vulnerable document search
#[get("/api/documents/search")]
async fn search_documents(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let db = DatabaseManager::new("users.db");
    
    let search_query = query.get("q").cloned().unwrap_or_default();
    let user_id = query.get("user_id").and_then(|s| s.parse().ok()).unwrap_or(1);
    
    match db.search_documents(&search_query, user_id) {
        Ok(documents) => HttpResponse::Ok().json(json!({"documents": documents})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// Vulnerable document content retrieval
#[get("/api/documents/{doc_id}/content")]
async fn get_document_content(path: web::Path<String>) -> impl Responder {
    let db = DatabaseManager::new("users.db");
    let doc_id = path.into_inner();
    
    match db.get_document_content(&doc_id) {
        Ok(Some(content)) => HttpResponse::Ok().json(json!({"content": content})),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "Document not found"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// Server-Side Request Forgery (SSRF)
#[post("/api/ssrf/fetch")]
async fn ssrf_fetch(req: web::Json<UrlRequest>) -> impl Responder {
    // Vulnerable: No URL validation
    match reqwest::get(&req.url).await {
        Ok(response) => {
            let status = response.status();
            let headers: HashMap<String, String> = response.headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();
            
            match response.text().await {
                Ok(text) => HttpResponse::Ok().json(json!({
                    "status_code": status.as_u16(),
                    "content": text.chars().take(1000).collect::<String>(),
                    "headers": headers
                })),
                Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// XML External Entity (XXE) vulnerability
#[post("/api/xml/parse")]
async fn parse_xml(body: web::Bytes) -> impl Responder {
    let xml_data = String::from_utf8_lossy(&body);
    
    match parse_xml_content(&xml_data) {
        Ok(parsed) => HttpResponse::Ok().json(json!({"parsed_xml": parsed})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// YAML deserialization vulnerability
#[post("/api/yaml/load")]
async fn load_yaml(body: web::Bytes) -> impl Responder {
    let yaml_data = String::from_utf8_lossy(&body);
    
    match parse_yaml_content(&yaml_data) {
        Ok(parsed) => HttpResponse::Ok().json(json!({"parsed_yaml": parsed})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// Unsafe deserialization endpoint
#[post("/api/deserialize")]
async fn deserialize_data(req: web::Json<DataRequest>) -> impl Responder {
    match deserialize_user_preferences(&req.data) {
        Ok(preferences) => HttpResponse::Ok().json(json!({"deserialized": preferences})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// Command execution endpoint
#[post("/api/exec/command")]
async fn execute_command(req: web::Json<CommandRequest>) -> impl Responder {
    match execute_system_command(&req.command) {
        Ok(output) => HttpResponse::Ok().json(json!({
            "command": req.command,
            "output": output
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// LDAP injection vulnerability
#[post("/api/ldap/search")]
async fn ldap_search(req: web::Json<LdapRequest>) -> impl Responder {
    // Vulnerable: LDAP injection
    let ldap_query = format!("(&(objectClass=user)(cn={}))", req.username);
    
    HttpResponse::Ok().json(json!({
        "ldap_query": ldap_query,
        "message": "LDAP search simulated (vulnerable to injection)",
        "username": req.username
    }))
}

/// Vulnerable file upload
#[post("/api/file/upload")]
async fn upload_file(mut payload: Multipart) -> Result<impl Responder> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        
        if let Some(filename) = content_disposition.get_filename() {
            // Vulnerable: No file type validation, path traversal possible
            let filepath = format!("/tmp/{}", filename);
            
            let mut f = web::block(move || std::fs::File::create(filepath.clone()))
                .await??
                .into();
            
            while let Some(chunk) = field.try_next().await? {
                f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
            }
            
            return Ok(HttpResponse::Ok().json(json!({
                "message": "File uploaded successfully",
                "filename": filename,
                "path": format!("/tmp/{}", filename)
            })));
        }
    }
    
    Ok(HttpResponse::BadRequest().json(json!({"error": "No file uploaded"})))
}

/// Directory listing vulnerability
#[get("/api/files/list")]
async fn list_files(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let directory = query.get("dir").cloned().unwrap_or_else(|| ".".to_string());
    
    match get_file_listing(&directory) {
        Ok(files) => HttpResponse::Ok().json(json!({
            "directory": directory,
            "files": files
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// Vulnerable audit log access
#[get("/api/logs/{user_id}")]
async fn get_user_logs(path: web::Path<String>) -> impl Responder {
    let db = DatabaseManager::new("users.db");
    let user_id = path.into_inner();
    
    match db.get_user_logs(&user_id) {
        Ok(logs) => HttpResponse::Ok().json(json!({"logs": logs})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

/// Main server setup
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    // Initialize database
    let _db = DatabaseManager::new("users.db");
    
    info!("🔓 Starting Advanced Vulnerable Rust Application");
    warn!("⚠️  This application contains intentional security vulnerabilities!");
    info!("🌍 Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // Web interface
            .service(index)
            .service(sql_injection)
            .service(command_injection)
            .service(file_read)
            // API endpoints
            .service(api_login)
            .service(get_user)
            .service(search_documents)
            .service(get_document_content)
            .service(ssrf_fetch)
            .service(parse_xml)
            .service(load_yaml)
            .service(deserialize_data)
            .service(execute_command)
            .service(ldap_search)
            .service(upload_file)
            .service(list_files)
            .service(get_user_logs)
            // Static files (vulnerable to path traversal)
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
