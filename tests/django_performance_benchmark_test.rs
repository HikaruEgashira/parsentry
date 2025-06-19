use anyhow::Result;
use parsentry::analyzer::analyze_file;
use parsentry::locales::Language as LocaleLanguage;
use parsentry::parser::CodeParser;
use std::time::Instant;
use tempfile::tempdir;

/// Django実世界脆弱性検出のパフォーマンスベンチマーク
/// Issue #120: PERF: Create Python Django performance benchmark for real-world vulnerability detection
/// 
/// Django Webアプリケーションでの複数脆弱性タイプの同時検出性能を測定し、
/// 実世界のWebアプリケーションで実用的な検出率とパフォーマンスを維持できているかを検証する

#[derive(Debug)]
struct DjangoBenchmarkResult {
    execution_time_ms: u128,
    lines_analyzed: usize,
    vulnerabilities_detected: usize,
    vulnerability_types_found: Vec<String>,
    analysis_speed: f64,
    detection_rate: f64,
    performance_target_met: bool,
}

fn generate_django_ecommerce_app() -> String {
    let mut code = String::new();
    
    // Django settings with vulnerabilities
    code.push_str(r#"
# Django E-commerce Application with Multiple Vulnerabilities
import os
import pickle
import subprocess
import yaml
from django.conf import settings
from django.shortcuts import render, redirect
from django.http import HttpResponse, JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.utils.safestring import mark_safe
from django.db import connection
from django.contrib.auth import authenticate, login
from django.core.serializers import serialize
import requests

# Settings with hardcoded secrets
DEBUG = True
SECRET_KEY = 'django-insecure-hardcoded-secret-key-123456789'
ALLOWED_HOSTS = ['*']

# Database with raw SQL vulnerabilities
class DatabaseManager:
    @staticmethod
    def get_user_by_id(user_id):
        # SQL Injection vulnerability
        with connection.cursor() as cursor:
            query = f"SELECT * FROM auth_user WHERE id = {user_id}"
            cursor.execute(query)
            return cursor.fetchone()
    
    @staticmethod 
    def search_products(search_term, category):
        # Multiple SQL injection points
        with connection.cursor() as cursor:
            query = f"""
                SELECT p.*, c.name as category_name
                FROM products p 
                JOIN categories c ON p.category_id = c.id
                WHERE p.name LIKE '%{search_term}%' 
                AND c.name = '{category}'
                ORDER BY p.created_at DESC
            """
            cursor.execute(query)
            return cursor.fetchall()

"#);

    // Generate Django models with vulnerabilities
    for i in 0..10 {
        code.push_str(&format!(r#"
# Model {} with serialization vulnerabilities
class Product{}(models.Model):
    name = models.CharField(max_length=255)
    description = models.TextField()
    price = models.DecimalField(max_digits=10, decimal_places=2)
    category = models.CharField(max_length=100)
    metadata = models.TextField()  # Stores pickled data
    
    def save_metadata(self, data):
        # Insecure deserialization
        self.metadata = pickle.dumps(data)
        self.save()
    
    def load_metadata(self):
        # Unsafe pickle loading
        if self.metadata:
            return pickle.loads(self.metadata.encode('latin1'))
        return {{}}
    
    def process_config(self, config_data):
        # YAML deserialization vulnerability
        try:
            config = yaml.load(config_data, Loader=yaml.Loader)
            return config
        except Exception as e:
            return {{'error': str(e)}}

class Order{}(models.Model):
    user = models.ForeignKey('auth.User', on_delete=models.CASCADE)
    products = models.ManyToManyField(Product{})
    total_amount = models.DecimalField(max_digits=10, decimal_places=2)
    payment_info = models.TextField()  # Sensitive data storage
    
    def save_payment_data(self, payment_data):
        # Insecure sensitive data storage
        self.payment_info = str(payment_data)  # Credit card data in plain text
        self.save()

"#, i, i, i, i));
    }

    // Generate Django views with multiple vulnerability types
    for i in 0..15 {
        code.push_str(&format!(r#"
# View {} with multiple vulnerabilities
@csrf_exempt  # CSRF protection disabled
def product_search_view_{}(request):
    if request.method == 'POST':
        search_term = request.POST.get('search', '')
        category = request.POST.get('category', '')
        sort_by = request.POST.get('sort', 'name')
        
        # SQL injection in search
        with connection.cursor() as cursor:
            query = f"""
                SELECT * FROM products 
                WHERE name LIKE '%{search_term}%' 
                AND category = '{category}'
                ORDER BY {sort_by}
            """
            cursor.execute(query)
            products = cursor.fetchall()
        
        # XSS vulnerability in template rendering
        search_html = f"<h2>Search results for: {search_term}</h2>"
        
        context = {{
            'products': products,
            'search_html': mark_safe(search_html),  # XSS via mark_safe
            'search_term': search_term,
            'category': category
        }}
        
        return render(request, 'search_results.html', context)
    
    return render(request, 'search_form.html')

def user_profile_view_{}(request, user_id):
    # IDOR - Insecure Direct Object Reference
    user_data = DatabaseManager.get_user_by_id(user_id)
    
    if request.method == 'POST':
        # Mass assignment vulnerability
        update_data = request.POST.dict()
        
        # Direct SQL update without validation
        with connection.cursor() as cursor:
            updates = ', '.join([f"{k} = '{v}'" for k, v in update_data.items()])
            query = f"UPDATE auth_user SET {updates} WHERE id = {user_id}"
            cursor.execute(query)
    
    # Information disclosure
    context = {{
        'user': user_data,
        'debug_info': {{
            'query_count': len(connection.queries),
            'session_data': request.session.items(),
            'meta_data': request.META
        }}
    }}
    
    return render(request, 'profile.html', context)

def file_upload_view_{}(request):
    if request.method == 'POST':
        uploaded_file = request.FILES.get('file')
        destination_path = request.POST.get('path', '/tmp/')
        filename = request.POST.get('filename', uploaded_file.name)
        
        # Path traversal vulnerability
        full_path = os.path.join(destination_path, filename)
        
        with open(full_path, 'wb') as f:
            for chunk in uploaded_file.chunks():
                f.write(chunk)
        
        # Command injection in file processing
        file_type = request.POST.get('process_type', 'none')
        if file_type == 'image':
            cmd = f"identify {full_path}"
            result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        elif file_type == 'document':
            cmd = f"file {full_path}"
            result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        
        return JsonResponse({{
            'success': True,
            'path': full_path,
            'file_info': result.stdout if 'result' in locals() else None
        }})
    
    return render(request, 'upload.html')

"#, i, i, i, i));
    }

    // Generate template injection vulnerabilities
    for i in 0..8 {
        code.push_str(&format!(r#"
# Template injection view {}
def dynamic_template_view_{}(request):
    template_content = request.GET.get('template', '')
    user_data = request.GET.get('data', '')
    
    # Server-side template injection
    from django.template import Template, Context
    
    # Dynamic template creation from user input
    template = Template(template_content)
    context = Context({{'user_data': user_data, 'request': request}})
    
    try:
        rendered = template.render(context)
        return HttpResponse(rendered)
    except Exception as e:
        # Error information disclosure
        return HttpResponse(f"Template error: {e}, Template: {template_content}")

def webhook_handler_{}(request):
    if request.method == 'POST':
        target_url = request.POST.get('url')
        payload = request.POST.get('payload')
        method = request.POST.get('method', 'POST')
        
        # SSRF vulnerability
        try:
            if method == 'GET':
                response = requests.get(target_url, timeout=10)
            else:
                response = requests.post(target_url, data=payload, timeout=10)
            
            return JsonResponse({{
                'status': response.status_code,
                'content': response.text,
                'url': target_url
            }})
        except Exception as e:
            return JsonResponse({{
                'error': str(e),
                'url': target_url
            }})
    
    return JsonResponse({{'error': 'Only POST method allowed'}})

"#, i, i, i));
    }

    // Add Django middleware with vulnerabilities
    code.push_str(r#"
# Custom middleware with security vulnerabilities
class VulnerableMiddleware:
    def __init__(self, get_response):
        self.get_response = get_response
    
    def __call__(self, request):
        # Logging sensitive data
        if request.method == 'POST':
            print(f"POST data: {request.POST}")
            if 'password' in request.POST:
                print(f"Password attempt: {request.POST['password']}")
        
        # Session fixation
        if not request.session.session_key:
            request.session.save()
        
        # Response header injection
        response = self.get_response(request)
        
        custom_header = request.GET.get('custom_header', '')
        if custom_header:
            response['X-Custom-Header'] = custom_header  # Header injection
        
        return response
    
    def process_exception(self, request, exception):
        # Information disclosure in error handling
        return HttpResponse(f"Error occurred: {exception}, Request: {request.GET}")

# Authentication bypass utilities
class AuthUtils:
    @staticmethod
    def weak_hash_password(password):
        # Weak password hashing
        import hashlib
        return hashlib.md5(password.encode()).hexdigest()
    
    @staticmethod
    def check_password(password, stored_hash):
        # Timing attack vulnerability
        return AuthUtils.weak_hash_password(password) == stored_hash
    
    @staticmethod
    def generate_token():
        # Predictable token generation
        import time
        return f"token_{int(time.time())}"
    
    @staticmethod
    def validate_session(session_token):
        # Session validation bypass
        if session_token.startswith('admin_'):
            return True
        return session_token in ['valid_token_1', 'valid_token_2']

# File handling with vulnerabilities
class FileHandler:
    @staticmethod
    def process_uploaded_file(file_path, file_type):
        # Command injection in file processing
        if file_type == 'archive':
            cmd = f"tar -tf {file_path}"
            return subprocess.run(cmd, shell=True, capture_output=True, text=True)
        elif file_type == 'image':
            cmd = f"exiftool {file_path}"
            return subprocess.run(cmd, shell=True, capture_output=True, text=True)
        elif file_type == 'document':
            cmd = f"strings {file_path}"
            return subprocess.run(cmd, shell=True, capture_output=True, text=True)
    
    @staticmethod
    def read_config_file(config_path):
        # Path traversal in config reading
        full_path = f"./configs/{config_path}"
        try:
            with open(full_path, 'r') as f:
                content = f.read()
                # YAML deserialization
                return yaml.load(content, Loader=yaml.Loader)
        except Exception as e:
            return {'error': str(e), 'path': full_path}

# URL patterns with vulnerabilities
urlpatterns = [
    path('search/<int:page>/', product_search_view_0, name='search'),
    path('profile/<path:user_path>/', user_profile_view_0, name='profile'),
    path('upload/<str:filename>/', file_upload_view_0, name='upload'),
    path('template/<str:template_name>/', dynamic_template_view_0, name='template'),
    path('webhook/<path:endpoint>/', webhook_handler_0, name='webhook'),
]
"#);

    code
}

async fn run_django_performance_benchmark(model: &str) -> Result<DjangoBenchmarkResult> {
    let start_time = Instant::now();
    
    // Generate Django application code
    let django_code = generate_django_ecommerce_app();
    let lines_analyzed = django_code.lines().count();
    
    // Create temporary file
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("django_ecommerce.py");
    std::fs::write(&test_file, &django_code)?;
    
    println!("📊 Django Performance Benchmark");
    println!("   ├─ Generated code: {} lines", lines_analyzed);
    println!("   ├─ File size: {} KB", django_code.len() / 1024);
    println!("   └─ Analysis target: Django e-commerce application");
    
    // Parse and build context
    let parse_start = Instant::now();
    let mut parser = CodeParser::new()?;
    parser.add_file(&test_file)?;
    let context = parser.build_context_from_file(&test_file)?;
    let parse_duration = parse_start.elapsed();
    
    println!("   ├─ Parsing time: {:.2} seconds", parse_duration.as_secs_f64());
    
    // Analyze file
    let analysis_start = Instant::now();
    let response = analyze_file(
        &test_file,
        model,
        &[test_file.clone()],
        0,
        &context,
        0,
        false,
        &None,
        None,
        &LocaleLanguage::Japanese,
    ).await?;
    let analysis_duration = analysis_start.elapsed();
    
    let total_duration = start_time.elapsed();
    let analysis_speed = lines_analyzed as f64 / total_duration.as_secs_f64();
    
    // Calculate vulnerability types found
    let vulnerability_types_found: Vec<String> = response.vulnerability_types
        .iter()
        .map(|v| format!("{:?}", v))
        .collect();
    
    // Performance targets for Django
    let target_max_time_ms = 240_000; // 4 minutes
    let target_min_speed = 60.0; // 60 lines per second
    let target_min_vulnerabilities = 40; // Should detect at least 40 vulnerabilities
    let expected_vulnerability_types = 8; // Should find multiple types
    
    let detection_rate = vulnerability_types_found.len() as f64 / expected_vulnerability_types as f64;
    
    let performance_target_met = total_duration.as_millis() <= target_max_time_ms 
        && analysis_speed >= target_min_speed
        && response.vulnerability_types.len() >= target_min_vulnerabilities
        && vulnerability_types_found.len() >= 5; // At least 5 different vulnerability types
    
    println!("   ├─ Analysis time: {:.2} seconds", analysis_duration.as_secs_f64());
    println!("   ├─ Total time: {:.2} seconds", total_duration.as_secs_f64());
    println!("   ├─ Analysis speed: {:.1} lines/second", analysis_speed);
    println!("   ├─ Vulnerabilities detected: {}", response.vulnerability_types.len());
    println!("   ├─ Vulnerability types: {}", vulnerability_types_found.len());
    println!("   ├─ Detection rate: {:.1}%", detection_rate * 100.0);
    println!("   └─ Performance target: {}", if performance_target_met { "✅ MET" } else { "❌ FAILED" });
    
    Ok(DjangoBenchmarkResult {
        execution_time_ms: total_duration.as_millis(),
        lines_analyzed,
        vulnerabilities_detected: response.vulnerability_types.len(),
        vulnerability_types_found,
        analysis_speed,
        detection_rate,
        performance_target_met,
    })
}

#[tokio::test]
async fn test_django_multi_vulnerability_detection() -> Result<()> {
    // Skip API-based tests in CI or when API key is not available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping Django performance benchmark test");
        return Ok(());
    }
    
    let model = "gpt-4.1-mini";
    
    println!("\n🐍 Django Multi-Vulnerability Detection Performance Benchmark");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Testing performance with Django e-commerce application");
    println!("Target: Detect multiple vulnerability types simultaneously in < 4 minutes");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let result = run_django_performance_benchmark(model).await?;
    
    println!("\n📈 Performance Results:");
    println!("   ├─ Execution Time: {:.2} seconds ({} ms)", 
            result.execution_time_ms as f64 / 1000.0, result.execution_time_ms);
    println!("   ├─ Lines Analyzed: {} lines", result.lines_analyzed);
    println!("   ├─ Analysis Speed: {:.1} lines/second", result.analysis_speed);
    println!("   ├─ Vulnerabilities: {} detected", result.vulnerabilities_detected);
    println!("   ├─ Vulnerability Types: {} different types", result.vulnerability_types_found.len());
    println!("   ├─ Detection Rate: {:.1}%", result.detection_rate * 100.0);
    println!("   └─ Overall Performance: {}", if result.performance_target_met { "✅ PASSED" } else { "❌ FAILED" });
    
    if !result.vulnerability_types_found.is_empty() {
        println!("\n🔍 Detected Vulnerability Types:");
        for (i, vuln_type) in result.vulnerability_types_found.iter().enumerate() {
            println!("   {}. {}", i + 1, vuln_type);
        }
    }
    
    // Detailed performance assertions
    assert!(
        result.execution_time_ms <= 240_000,
        "Analysis took too long: {} ms (limit: 240,000 ms / 4 minutes)",
        result.execution_time_ms
    );
    
    assert!(
        result.analysis_speed >= 60.0,
        "Analysis too slow: {:.1} lines/second (minimum: 60.0 lines/second)",
        result.analysis_speed
    );
    
    assert!(
        result.vulnerabilities_detected >= 40,
        "Too few vulnerabilities detected: {} (minimum: 40)",
        result.vulnerabilities_detected
    );
    
    assert!(
        result.vulnerability_types_found.len() >= 5,
        "Too few vulnerability types detected: {} (minimum: 5)",
        result.vulnerability_types_found.len()
    );
    
    assert!(
        result.detection_rate >= 0.6,
        "Detection rate too low: {:.1}% (minimum: 60%)",
        result.detection_rate * 100.0
    );
    
    println!("\n🎉 Django Multi-Vulnerability Detection Performance Benchmark PASSED!");
    println!("   The scanner successfully analyzed Django application with multiple");
    println!("   vulnerability types within performance targets.");
    
    Ok(())
}

#[tokio::test]
async fn test_django_orm_performance() -> Result<()> {
    println!("\n🗄️ Django ORM Analysis Performance Test (API-free)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Generate Django models with ORM vulnerabilities
    let django_models = r#"
from django.db import models, connection
from django.contrib.auth.models import User

class UserProfile(models.Model):
    user = models.OneToOneField(User, on_delete=models.CASCADE)
    ssn = models.CharField(max_length=11)
    credit_card = models.CharField(max_length=16)
    
    @classmethod
    def get_profile_by_user_input(cls, user_input):
        # SQL injection in Django ORM
        with connection.cursor() as cursor:
            query = f"SELECT * FROM userprofile WHERE user_id = {user_input}"
            cursor.execute(query)
            return cursor.fetchone()

class Product(models.Model):
    name = models.CharField(max_length=255)
    category = models.CharField(max_length=100)
    
    @classmethod
    def search_products(cls, search_term, category):
        # Multiple SQL injection points
        with connection.cursor() as cursor:
            query = f"""
                SELECT * FROM products 
                WHERE name LIKE '%{search_term}%' 
                AND category = '{category}'
            """
            cursor.execute(query)
            return cursor.fetchall()

class Order(models.Model):
    user = models.ForeignKey(User, on_delete=models.CASCADE)
    products = models.ManyToManyField(Product)
    
    def process_payment(self, payment_data):
        # Insecure payment processing
        card_number = payment_data['card_number']
        with connection.cursor() as cursor:
            query = f"INSERT INTO payments (card_number) VALUES ('{card_number}')"
            cursor.execute(query)
"#;
    
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("django_models.py");
    std::fs::write(&test_file, django_models)?;
    
    let start_time = Instant::now();
    
    // Test Django ORM parsing performance
    let mut parser = CodeParser::new()?;
    parser.add_file(&test_file)?;
    let _context = parser.build_context_from_file(&test_file)?;
    
    let duration = start_time.elapsed();
    let lines = django_models.lines().count();
    let speed = lines as f64 / duration.as_secs_f64();
    
    println!("   📊 Django ORM Analysis:");
    println!("      ├─ Lines: {} lines", lines);
    println!("      ├─ Time: {:.3} seconds", duration.as_secs_f64());
    println!("      └─ Speed: {:.1} lines/second", speed);
    
    // ORM analysis should be efficient
    assert!(
        speed > 150.0,
        "Django ORM analysis too slow: {:.1} lines/s (minimum: 150 lines/s)",
        speed
    );
    
    println!("   ✅ Django ORM analysis performance acceptable");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    Ok(())
}

#[tokio::test]
async fn test_django_template_injection_performance() -> Result<()> {
    println!("\n📄 Django Template Injection Performance Test (API-free)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Generate Django views with template injection patterns
    let mut template_code = String::new();
    template_code.push_str(r#"
from django.template import Template, Context
from django.http import HttpResponse
from django.utils.safestring import mark_safe

"#);
    
    for i in 0..20 {
        template_code.push_str(&format!(r#"
def template_view_{}(request):
    template_content = request.GET.get('template', '')
    user_data = request.GET.get('data', '')
    
    # Server-side template injection
    template = Template(template_content)
    context = Context({{'user_data': user_data, 'request': request}})
    
    try:
        rendered = template.render(context)
        return HttpResponse(rendered)
    except Exception as e:
        return HttpResponse(f"Error: {{e}}, Template: {{template_content}}")

def unsafe_render_{}(request):
    content = request.POST.get('content', '')
    # XSS via mark_safe
    safe_content = mark_safe(content)
    return HttpResponse(f"<div>{{safe_content}}</div>")
"#, i, i));
    }
    
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("django_templates.py");
    std::fs::write(&test_file, &template_code)?;
    
    let start_time = Instant::now();
    
    let mut parser = CodeParser::new()?;
    parser.add_file(&test_file)?;
    let _context = parser.build_context_from_file(&test_file)?;
    
    let duration = start_time.elapsed();
    let lines = template_code.lines().count();
    let speed = lines as f64 / duration.as_secs_f64();
    
    println!("   📊 Template Injection Analysis:");
    println!("      ├─ Lines: {} lines", lines);
    println!("      ├─ Template functions: 40");
    println!("      ├─ Time: {:.3} seconds", duration.as_secs_f64());
    println!("      └─ Speed: {:.1} lines/second", speed);
    
    // Template analysis should handle complex patterns efficiently
    assert!(
        speed > 100.0,
        "Template injection analysis too slow: {:.1} lines/s (minimum: 100 lines/s)",
        speed
    );
    
    println!("   ✅ Django template injection analysis performance acceptable");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    Ok(())
}