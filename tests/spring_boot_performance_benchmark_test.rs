use anyhow::Result;
use parsentry::analyzer::analyze_file;
use parsentry::locales::Language as LocaleLanguage;
use parsentry::parser::CodeParser;
use std::time::Instant;
use tempfile::tempdir;

/// Spring Boot マイクロサービス パフォーマンスベンチマーク
/// Issue #121: PERF: Create Java Spring Boot microservices performance benchmark
/// 
/// Spring Boot マイクロサービス環境での脆弱性解析パフォーマンスを測定し、
/// 大規模プロジェクト構造でのスケーラビリティと検出精度を検証する

#[derive(Debug)]
struct SpringBootBenchmarkResult {
    execution_time_ms: u128,
    lines_analyzed: usize,
    classes_analyzed: usize,
    vulnerabilities_detected: usize,
    microservices_count: usize,
    analysis_speed: f64,
    scalability_score: f64,
    performance_target_met: bool,
}

fn generate_spring_boot_gateway_service() -> String {
    r#"
package com.example.gateway;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.gateway.route.RouteLocator;
import org.springframework.cloud.gateway.route.builder.RouteLocatorBuilder;
import org.springframework.context.annotation.Bean;
import org.springframework.web.bind.annotation.*;
import org.springframework.http.ResponseEntity;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.WebSecurityConfigurerAdapter;
import org.springframework.web.client.RestTemplate;

import javax.servlet.http.HttpServletRequest;
import java.sql.*;
import java.io.IOException;
import java.util.Map;

@SpringBootApplication
public class GatewayApplication {
    public static void main(String[] args) {
        SpringApplication.run(GatewayApplication.class, args);
    }
    
    @Bean
    public RouteLocator customRouteLocator(RouteLocatorBuilder builder) {
        return builder.routes()
            .route("user-service", r -> r.path("/api/users/**")
                .uri("http://user-service:8081"))
            .route("order-service", r -> r.path("/api/orders/**") 
                .uri("http://order-service:8082"))
            .build();
    }
}

@RestController
@RequestMapping("/gateway")
public class ProxyController {
    
    private final RestTemplate restTemplate = new RestTemplate();
    private Connection dbConnection;
    
    // SSRF vulnerability in proxy endpoint
    @GetMapping("/proxy")
    public ResponseEntity<String> proxyRequest(
            @RequestParam String targetUrl,
            @RequestParam(required = false) String method) {
        
        try {
            // No URL validation - SSRF vulnerability
            String result = restTemplate.getForObject(targetUrl, String.class);
            return ResponseEntity.ok(result);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body("Error: " + e.getMessage() + ", URL: " + targetUrl);
        }
    }
    
    // Path traversal vulnerability
    @GetMapping("/files/{filename}")
    public ResponseEntity<String> getFile(@PathVariable String filename) {
        try {
            String filePath = "/data/" + filename; // No path sanitization
            java.nio.file.Path path = java.nio.file.Paths.get(filePath);
            String content = new String(java.nio.file.Files.readAllBytes(path));
            return ResponseEntity.ok(content);
        } catch (IOException e) {
            return ResponseEntity.badRequest().body("File not found: " + filename);
        }
    }
    
    // SQL injection in admin endpoint
    @PostMapping("/admin/query")
    public ResponseEntity<String> executeAdminQuery(
            @RequestParam String query,
            @RequestParam String userRole,
            HttpServletRequest request) {
        
        // Role verification bypass via header
        String adminHeader = request.getHeader("X-Admin-Token");
        if (!"admin123".equals(adminHeader)) {
            return ResponseEntity.status(403).body("Access denied");
        }
        
        try {
            // SQL injection vulnerability
            Statement stmt = dbConnection.createStatement();
            ResultSet rs = stmt.executeQuery(query);
            
            StringBuilder result = new StringBuilder();
            while (rs.next()) {
                result.append(rs.getString(1)).append("\n");
            }
            
            return ResponseEntity.ok(result.toString());
        } catch (SQLException e) {
            return ResponseEntity.badRequest().body("SQL Error: " + e.getMessage() + ", Query: " + query);
        }
    }
    
    // XSS vulnerability in error response
    @GetMapping("/error")
    public ResponseEntity<String> handleError(@RequestParam String message) {
        String errorHtml = "<html><body><h1>Error: " + message + "</h1></body></html>";
        return ResponseEntity.ok(errorHtml);
    }
}

@Configuration
public class SecurityConfig extends WebSecurityConfigurerAdapter {
    
    @Override
    protected void configure(HttpSecurity http) throws Exception {
        http
            .csrf().disable() // CSRF protection disabled
            .authorizeRequests()
                .antMatchers("/gateway/admin/**").hasRole("ADMIN")
                .antMatchers("/gateway/**").permitAll()
                .anyRequest().authenticated()
            .and()
            .headers()
                .frameOptions().disable() // Clickjacking protection disabled
                .and()
            .sessionManagement()
                .maximumSessions(1)
                .maxSessionsPreventsLogin(false);
    }
}

@Component
public class AuthenticationFilter implements javax.servlet.Filter {
    
    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) 
            throws IOException, ServletException {
        
        HttpServletRequest httpRequest = (HttpServletRequest) request;
        
        // Weak authentication check
        String token = httpRequest.getHeader("Authorization");
        String userAgent = httpRequest.getHeader("User-Agent");
        
        // Log sensitive information
        System.out.println("Auth attempt - Token: " + token + ", User-Agent: " + userAgent);
        
        // Authentication bypass
        if (token != null && (token.startsWith("Bearer admin") || userAgent.contains("AdminBot"))) {
            httpRequest.setAttribute("authenticated", true);
        }
        
        chain.doFilter(request, response);
    }
}
"#.to_string()
}

fn generate_spring_boot_user_service() -> String {
    r#"
package com.example.userservice;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.*;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.WebSecurityConfigurerAdapter;

import javax.persistence.*;
import javax.servlet.http.HttpServletRequest;
import java.sql.*;
import java.util.*;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.SignatureAlgorithm;

@SpringBootApplication
public class UserApplication {
    public static void main(String[] args) {
        SpringApplication.run(UserApplication.class, args);
    }
}

@Entity
@Table(name = "users")
public class User {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    
    @Column(unique = true)
    private String username;
    
    private String password;
    private String email;
    private String role;
    private String ssn; // Sensitive data storage
    private String creditCard; // PCI data in user table
    private String apiKey; // Exposed API key
    
    // Getters and setters with no validation
    public void setPassword(String password) {
        // Weak password hashing
        this.password = new BCryptPasswordEncoder(4).encode(password); // Low rounds
    }
    
    public void setSsn(String ssn) {
        this.ssn = ssn; // No encryption for SSN
    }
    
    public void setCreditCard(String creditCard) {
        this.creditCard = creditCard; // Plain text credit card storage
    }
}

@RestController
@RequestMapping("/api/users")
public class UserController {
    
    private final UserService userService;
    private Connection dbConnection;
    
    public UserController(UserService userService) {
        this.userService = userService;
    }
    
    // IDOR vulnerability
    @GetMapping("/{userId}")
    public ResponseEntity<User> getUser(@PathVariable Long userId, HttpServletRequest request) {
        // No authorization check - IDOR vulnerability
        User user = userService.findById(userId);
        
        if (user != null) {
            // Return sensitive data including SSN and credit card
            return ResponseEntity.ok(user);
        }
        
        return ResponseEntity.notFound().build();
    }
    
    // SQL injection in search
    @GetMapping("/search")
    public ResponseEntity<List<User>> searchUsers(
            @RequestParam String query,
            @RequestParam(required = false) String role,
            @RequestParam(required = false) String sortBy) {
        
        try {
            // SQL injection vulnerability
            String sql = String.format(
                "SELECT * FROM users WHERE username LIKE '%%%s%%' AND role = '%s' ORDER BY %s",
                query, role != null ? role : "user", sortBy != null ? sortBy : "id"
            );
            
            Statement stmt = dbConnection.createStatement();
            ResultSet rs = stmt.executeQuery(sql);
            
            List<User> users = new ArrayList<>();
            while (rs.next()) {
                User user = new User();
                user.setId(rs.getLong("id"));
                user.setUsername(rs.getString("username"));
                user.setEmail(rs.getString("email"));
                user.setSsn(rs.getString("ssn")); // Include sensitive data
                users.add(user);
            }
            
            return ResponseEntity.ok(users);
        } catch (SQLException e) {
            // Information disclosure in error message
            return ResponseEntity.badRequest().body(null);
        }
    }
    
    // Mass assignment vulnerability
    @PostMapping("/update/{userId}")
    public ResponseEntity<User> updateUser(
            @PathVariable Long userId,
            @RequestBody Map<String, Object> userData,
            HttpServletRequest request) {
        
        try {
            // Mass assignment - update any field from request
            StringBuilder updateQuery = new StringBuilder("UPDATE users SET ");
            List<String> setParts = new ArrayList<>();
            
            for (Map.Entry<String, Object> entry : userData.entrySet()) {
                setParts.add(String.format("%s = '%s'", entry.getKey(), entry.getValue()));
            }
            
            updateQuery.append(String.join(", ", setParts));
            updateQuery.append(String.format(" WHERE id = %d", userId));
            
            Statement stmt = dbConnection.createStatement();
            stmt.executeUpdate(updateQuery.toString());
            
            return ResponseEntity.ok(userService.findById(userId));
        } catch (SQLException e) {
            return ResponseEntity.badRequest().body(null);
        }
    }
    
    // JWT with weak secret
    @PostMapping("/login")
    public ResponseEntity<Map<String, String>> login(@RequestBody Map<String, String> credentials) {
        String username = credentials.get("username");
        String password = credentials.get("password");
        
        // Weak authentication
        User user = userService.findByUsername(username);
        if (user != null && new BCryptPasswordEncoder().matches(password, user.getPassword())) {
            
            // JWT with hardcoded secret
            String token = Jwts.builder()
                .setSubject(username)
                .claim("role", user.getRole())
                .claim("userId", user.getId())
                .signWith(SignatureAlgorithm.HS256, "secret123") // Weak secret
                .compact();
            
            Map<String, String> response = new HashMap<>();
            response.put("token", token);
            response.put("role", user.getRole());
            response.put("apiKey", user.getApiKey()); // Expose API key
            
            return ResponseEntity.ok(response);
        }
        
        return ResponseEntity.status(401).body(Map.of("error", "Invalid credentials"));
    }
    
    // Information disclosure endpoint
    @GetMapping("/admin/debug/{userId}")
    public ResponseEntity<Map<String, Object>> debugUser(@PathVariable Long userId) {
        User user = userService.findById(userId);
        
        Map<String, Object> debugInfo = new HashMap<>();
        debugInfo.put("user", user);
        debugInfo.put("password_hash", user.getPassword()); // Expose password hash
        debugInfo.put("ssn", user.getSsn()); // Expose SSN
        debugInfo.put("credit_card", user.getCreditCard()); // Expose credit card
        debugInfo.put("system_time", System.currentTimeMillis());
        debugInfo.put("jvm_memory", Runtime.getRuntime().totalMemory());
        
        return ResponseEntity.ok(debugInfo);
    }
}

@Service
public class UserService {
    private final UserRepository userRepository;
    private Connection dbConnection;
    
    public UserService(UserRepository userRepository) {
        this.userRepository = userRepository;
    }
    
    public User findById(Long id) {
        return userRepository.findById(id).orElse(null);
    }
    
    public User findByUsername(String username) {
        return userRepository.findByUsername(username);
    }
    
    // Batch operation with SQL injection
    public void bulkUpdateUsers(List<Map<String, Object>> updates) {
        try {
            for (Map<String, Object> update : updates) {
                String sql = String.format(
                    "UPDATE users SET role = '%s', status = '%s' WHERE id = %s",
                    update.get("role"), update.get("status"), update.get("id")
                );
                
                Statement stmt = dbConnection.createStatement();
                stmt.executeUpdate(sql);
            }
        } catch (SQLException e) {
            System.err.println("Bulk update error: " + e.getMessage());
        }
    }
}

public interface UserRepository extends JpaRepository<User, Long> {
    User findByUsername(String username);
    
    // Native query with potential injection
    @Query(value = "SELECT * FROM users WHERE role = ?1 AND status = 'active'", nativeQuery = true)
    List<User> findActiveUsersByRole(String role);
    
    // Dynamic query construction vulnerability
    @Query(value = "SELECT * FROM users WHERE username LIKE CONCAT('%', ?1, '%') ORDER BY ?2", nativeQuery = true)
    List<User> searchUsersWithSort(String searchTerm, String sortColumn);
}

@Utility
public class JwtUtil {
    private static final String SECRET = "hardcoded-jwt-secret-key-123"; // Hardcoded secret
    
    public static String generateToken(String username, String role) {
        return Jwts.builder()
            .setSubject(username)
            .claim("role", role)
            .setExpiration(new Date(System.currentTimeMillis() + 86400000)) // 24 hours
            .signWith(SignatureAlgorithm.HS256, SECRET)
            .compact();
    }
    
    public static boolean validateToken(String token) {
        try {
            Jwts.parser().setSigningKey(SECRET).parseClaimsJws(token);
            return true;
        } catch (Exception e) {
            return false;
        }
    }
}

@Configuration
public class SecurityConfig extends WebSecurityConfigurerAdapter {
    
    @Override
    protected void configure(HttpSecurity http) throws Exception {
        http
            .csrf().disable()
            .cors().disable()
            .authorizeRequests()
                .antMatchers("/api/users/admin/**").hasRole("ADMIN")
                .antMatchers("/api/users/login").permitAll()
                .anyRequest().permitAll() // Allow all requests - security bypass
            .and()
            .sessionManagement()
                .sessionCreationPolicy(SessionCreationPolicy.STATELESS);
    }
}
"#.to_string()
}

fn generate_spring_boot_order_service() -> String {
    r#"
package com.example.orderservice;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.client.RestTemplate;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;

import javax.persistence.*;
import javax.servlet.http.HttpServletRequest;
import java.sql.*;
import java.util.*;
import java.math.BigDecimal;

@SpringBootApplication
public class OrderApplication {
    public static void main(String[] args) {
        SpringApplication.run(OrderApplication.class, args);
    }
}

@Entity
@Table(name = "orders")
public class Order {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    
    private Long userId;
    private BigDecimal totalAmount;
    private String paymentMethod;
    private String creditCardNumber; // PCI data storage
    private String status;
    private Date createdAt;
    
    @OneToMany(mappedBy = "order", cascade = CascadeType.ALL)
    private List<OrderItem> items = new ArrayList<>();
    
    // Getters and setters
}

@Entity
@Table(name = "order_items")
public class OrderItem {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    
    @ManyToOne
    @JoinColumn(name = "order_id")
    private Order order;
    
    private Long productId;
    private Integer quantity;
    private BigDecimal price;
    private String productData; // Serialized data storage
}

@RestController
@RequestMapping("/api/orders")
public class OrderController {
    
    private final OrderService orderService;
    private final RestTemplate restTemplate = new RestTemplate();
    private Connection dbConnection;
    
    public OrderController(OrderService orderService) {
        this.orderService = orderService;
    }
    
    // IDOR vulnerability in order access
    @GetMapping("/{orderId}")
    public ResponseEntity<Order> getOrder(@PathVariable Long orderId, HttpServletRequest request) {
        // No authorization check - any user can access any order
        Order order = orderService.findById(orderId);
        
        if (order != null) {
            // Return order with sensitive payment information
            return ResponseEntity.ok(order);
        }
        
        return ResponseEntity.notFound().build();
    }
    
    // SQL injection in order search
    @GetMapping("/search")
    public ResponseEntity<List<Order>> searchOrders(
            @RequestParam String criteria,
            @RequestParam(required = false) String status,
            @RequestParam(required = false) String dateRange) {
        
        try {
            // SQL injection vulnerability
            String sql = String.format(
                "SELECT * FROM orders WHERE status = '%s' AND created_at %s AND (%s)",
                status != null ? status : "pending",
                dateRange != null ? dateRange : "> '2020-01-01'",
                criteria
            );
            
            Statement stmt = dbConnection.createStatement();
            ResultSet rs = stmt.executeQuery(sql);
            
            List<Order> orders = new ArrayList<>();
            while (rs.next()) {
                Order order = new Order();
                order.setId(rs.getLong("id"));
                order.setUserId(rs.getLong("user_id"));
                order.setTotalAmount(rs.getBigDecimal("total_amount"));
                order.setCreditCardNumber(rs.getString("credit_card_number"));
                orders.add(order);
            }
            
            return ResponseEntity.ok(orders);
        } catch (SQLException e) {
            return ResponseEntity.badRequest().body(null);
        }
    }
    
    // SSRF vulnerability in payment processing
    @PostMapping("/{orderId}/payment")
    public ResponseEntity<Map<String, Object>> processPayment(
            @PathVariable Long orderId,
            @RequestBody Map<String, Object> paymentData) {
        
        String paymentGatewayUrl = (String) paymentData.get("gateway_url");
        
        try {
            // SSRF vulnerability - no URL validation
            Map<String, Object> response = restTemplate.postForObject(
                paymentGatewayUrl, paymentData, Map.class);
            
            // Update order with payment information
            Order order = orderService.findById(orderId);
            order.setCreditCardNumber((String) paymentData.get("card_number"));
            order.setPaymentMethod((String) paymentData.get("method"));
            orderService.save(order);
            
            return ResponseEntity.ok(response);
        } catch (Exception e) {
            Map<String, Object> errorResponse = new HashMap<>();
            errorResponse.put("error", e.getMessage());
            errorResponse.put("gateway_url", paymentGatewayUrl);
            errorResponse.put("payment_data", paymentData);
            return ResponseEntity.badRequest().body(errorResponse);
        }
    }
    
    // Batch update with SQL injection
    @PostMapping("/admin/batch-update")
    public ResponseEntity<String> batchUpdateOrders(@RequestBody List<Map<String, Object>> updates) {
        try {
            for (Map<String, Object> update : updates) {
                // SQL injection in batch update
                String sql = String.format(
                    "UPDATE orders SET status = '%s', total_amount = %s WHERE user_id = %s",
                    update.get("status"),
                    update.get("amount"),
                    update.get("user_id")
                );
                
                Statement stmt = dbConnection.createStatement();
                int affected = stmt.executeUpdate(sql);
                
                System.out.println("Updated " + affected + " orders with query: " + sql);
            }
            
            return ResponseEntity.ok("Batch update completed");
        } catch (SQLException e) {
            return ResponseEntity.badRequest().body("Error: " + e.getMessage());
        }
    }
    
    // Information disclosure in admin endpoint
    @GetMapping("/admin/stats")
    public ResponseEntity<Map<String, Object>> getOrderStats(
            @RequestParam(required = false) String period,
            @RequestParam(required = false) String groupBy) {
        
        Map<String, Object> stats = new HashMap<>();
        
        try {
            // Dynamic query construction
            String sql = String.format(
                "SELECT %s, COUNT(*), SUM(total_amount), AVG(total_amount) FROM orders WHERE created_at > '%s' GROUP BY %s",
                groupBy != null ? groupBy : "DATE(created_at)",
                period != null ? period : "2020-01-01",
                groupBy != null ? groupBy : "DATE(created_at)"
            );
            
            Statement stmt = dbConnection.createStatement();
            ResultSet rs = stmt.executeQuery(sql);
            
            List<Map<String, Object>> results = new ArrayList<>();
            while (rs.next()) {
                Map<String, Object> row = new HashMap<>();
                row.put("period", rs.getString(1));
                row.put("count", rs.getInt(2));
                row.put("total", rs.getBigDecimal(3));
                row.put("average", rs.getBigDecimal(4));
                results.add(row);
            }
            
            stats.put("data", results);
            stats.put("query", sql); // Expose query in response
            stats.put("execution_time", System.currentTimeMillis());
            
        } catch (SQLException e) {
            stats.put("error", e.getMessage());
            stats.put("sql_state", e.getSQLState());
        }
        
        return ResponseEntity.ok(stats);
    }
}

@Service
public class OrderService {
    private final OrderRepository orderRepository;
    
    public OrderService(OrderRepository orderRepository) {
        this.orderRepository = orderRepository;
    }
    
    public Order findById(Long id) {
        return orderRepository.findById(id).orElse(null);
    }
    
    public Order save(Order order) {
        return orderRepository.save(order);
    }
    
    // Insecure deserialization
    public void processOrderData(String serializedData) {
        try {
            // Unsafe deserialization
            java.io.ObjectInputStream ois = new java.io.ObjectInputStream(
                new java.io.ByteArrayInputStream(
                    java.util.Base64.getDecoder().decode(serializedData)
                )
            );
            Object orderData = ois.readObject();
            
            // Process deserialized data without validation
            processUnsafeData(orderData);
        } catch (Exception e) {
            System.err.println("Deserialization error: " + e.getMessage());
        }
    }
    
    private void processUnsafeData(Object data) {
        // Unsafe processing of deserialized data
        if (data instanceof Map) {
            Map<String, Object> orderMap = (Map<String, Object>) data;
            // Process without validation
        }
    }
}

public interface OrderRepository extends JpaRepository<Order, Long> {
    
    // Native query with SQL injection risk
    @Query(value = "SELECT * FROM orders WHERE user_id = ?1 AND status IN (?2)", nativeQuery = true)
    List<Order> findOrdersByUserAndStatuses(Long userId, String statuses);
    
    // Dynamic sorting vulnerability
    @Query(value = "SELECT * FROM orders ORDER BY ?1 ?2", nativeQuery = true)
    List<Order> findAllOrdersSorted(String sortColumn, String sortDirection);
}

@Configuration
public class WebConfig implements WebMvcConfigurer {
    
    @Override
    public void addCorsMappings(CorsRegistry registry) {
        registry.addMapping("/**")
                .allowedOrigins("*") // Allow all origins
                .allowedMethods("*") // Allow all methods
                .allowedHeaders("*") // Allow all headers
                .allowCredentials(true);
    }
}
"#.to_string()
}

async fn run_spring_boot_performance_benchmark(model: &str) -> Result<SpringBootBenchmarkResult> {
    let start_time = Instant::now();
    
    // Generate Spring Boot microservices code
    let gateway_service = generate_spring_boot_gateway_service();
    let user_service = generate_spring_boot_user_service();
    let order_service = generate_spring_boot_order_service();
    
    // Combine all microservices
    let full_code = format!("{}\n\n{}\n\n{}", gateway_service, user_service, order_service);
    let lines_analyzed = full_code.lines().count();
    
    // Count classes and services
    let classes_count = full_code.matches("class ").count() + full_code.matches("interface ").count();
    let microservices_count = 3; // Gateway, User, Order services
    
    // Create temporary file
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("spring_boot_microservices.java");
    std::fs::write(&test_file, &full_code)?;
    
    println!("📊 Spring Boot Microservices Performance Benchmark");
    println!("   ├─ Generated code: {} lines", lines_analyzed);
    println!("   ├─ File size: {} KB", full_code.len() / 1024);
    println!("   ├─ Classes/Interfaces: {}", classes_count);
    println!("   ├─ Microservices: {}", microservices_count);
    println!("   └─ Analysis target: Spring Boot microservices architecture");
    
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
    
    // Calculate scalability score based on complexity handling
    let complexity_factor = (classes_count as f64 * microservices_count as f64) / lines_analyzed as f64;
    let scalability_score = if analysis_speed > 40.0 && complexity_factor > 0.1 {
        100.0
    } else if analysis_speed > 30.0 {
        80.0
    } else if analysis_speed > 20.0 {
        60.0
    } else {
        40.0
    };
    
    // Performance targets for Spring Boot microservices
    let target_max_time_ms = 360_000; // 6 minutes (complex enterprise code)
    let target_min_speed = 30.0; // 30 lines per second (lower due to complexity)
    let target_min_vulnerabilities = 30; // Should detect at least 30 vulnerabilities
    let target_min_scalability = 60.0; // Scalability score
    
    let performance_target_met = total_duration.as_millis() <= target_max_time_ms 
        && analysis_speed >= target_min_speed
        && response.vulnerability_types.len() >= target_min_vulnerabilities
        && scalability_score >= target_min_scalability;
    
    println!("   ├─ Analysis time: {:.2} seconds", analysis_duration.as_secs_f64());
    println!("   ├─ Total time: {:.2} seconds", total_duration.as_secs_f64());
    println!("   ├─ Analysis speed: {:.1} lines/second", analysis_speed);
    println!("   ├─ Vulnerabilities detected: {}", response.vulnerability_types.len());
    println!("   ├─ Scalability score: {:.1}%", scalability_score);
    println!("   └─ Performance target: {}", if performance_target_met { "✅ MET" } else { "❌ FAILED" });
    
    Ok(SpringBootBenchmarkResult {
        execution_time_ms: total_duration.as_millis(),
        lines_analyzed,
        classes_analyzed: classes_count,
        vulnerabilities_detected: response.vulnerability_types.len(),
        microservices_count,
        analysis_speed,
        scalability_score,
        performance_target_met,
    })
}

#[tokio::test]
async fn test_spring_boot_microservices_performance() -> Result<()> {
    // Skip API-based tests in CI or when API key is not available
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("OPENAI_API_KEY not set, skipping Spring Boot performance benchmark test");
        return Ok(());
    }
    
    let model = "gpt-4.1-mini";
    
    println!("\n☕ Spring Boot Microservices Performance Benchmark");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Testing performance with Spring Boot microservices architecture");
    println!("Target: Analyze complex enterprise patterns in < 6 minutes with 30+ vulnerabilities");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let result = run_spring_boot_performance_benchmark(model).await?;
    
    println!("\n📈 Performance Results:");
    println!("   ├─ Execution Time: {:.2} seconds ({} ms)", 
            result.execution_time_ms as f64 / 1000.0, result.execution_time_ms);
    println!("   ├─ Lines Analyzed: {} lines", result.lines_analyzed);
    println!("   ├─ Classes Analyzed: {} classes/interfaces", result.classes_analyzed);
    println!("   ├─ Microservices: {} services", result.microservices_count);
    println!("   ├─ Analysis Speed: {:.1} lines/second", result.analysis_speed);
    println!("   ├─ Vulnerabilities: {} detected", result.vulnerabilities_detected);
    println!("   ├─ Scalability Score: {:.1}%", result.scalability_score);
    println!("   └─ Overall Performance: {}", if result.performance_target_met { "✅ PASSED" } else { "❌ FAILED" });
    
    // Detailed performance assertions
    assert!(
        result.execution_time_ms <= 360_000,
        "Analysis took too long: {} ms (limit: 360,000 ms / 6 minutes)",
        result.execution_time_ms
    );
    
    assert!(
        result.analysis_speed >= 30.0,
        "Analysis too slow: {:.1} lines/second (minimum: 30.0 lines/second)",
        result.analysis_speed
    );
    
    assert!(
        result.vulnerabilities_detected >= 30,
        "Too few vulnerabilities detected: {} (minimum: 30)",
        result.vulnerabilities_detected
    );
    
    assert!(
        result.scalability_score >= 60.0,
        "Scalability score too low: {:.1}% (minimum: 60%)",
        result.scalability_score
    );
    
    assert!(
        result.classes_analyzed >= 10,
        "Should analyze multiple classes: {} (minimum: 10)",
        result.classes_analyzed
    );
    
    println!("\n🎉 Spring Boot Microservices Performance Benchmark PASSED!");
    println!("   The scanner successfully analyzed complex microservices architecture");
    println!("   within performance targets while maintaining high detection accuracy.");
    
    Ok(())
}

#[tokio::test]
async fn test_spring_boot_annotation_performance() -> Result<()> {
    println!("\n📝 Spring Boot Annotation Processing Performance Test (API-free)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Generate Spring Boot code with heavy annotation usage
    let mut annotation_code = String::new();
    annotation_code.push_str(r#"
package com.example.annotations;

import org.springframework.web.bind.annotation.*;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.data.jpa.repository.Query;
import org.springframework.transaction.annotation.Transactional;
import javax.validation.Valid;
import javax.validation.constraints.*;

"#);
    
    for i in 0..25 {
        annotation_code.push_str(&format!(r#"
@RestController
@RequestMapping("/api/annotation-test-{}")
@CrossOrigin(origins = "*")
public class AnnotationController{} {{
    
    @GetMapping("/data/{{id}}")
    @PreAuthorize("hasRole('USER')")
    @ResponseBody
    public ResponseEntity<String> getData(@PathVariable Long id, @RequestParam String query) {{
        // SQL injection in annotated method
        String sql = "SELECT * FROM data WHERE id = " + id + " AND name LIKE '%" + query + "%'";
        return ResponseEntity.ok(executeQuery(sql));
    }}
    
    @PostMapping("/update")
    @Transactional
    @PreAuthorize("hasRole('ADMIN')")
    public ResponseEntity<String> updateData(
            @Valid @RequestBody DataRequest request,
            @RequestHeader("X-Auth-Token") String token) {{
        
        // Command injection in annotated method
        String command = "update_script.sh " + request.getData();
        Runtime.getRuntime().exec(command);
        
        return ResponseEntity.ok("Updated");
    }}
    
    @DeleteMapping("/remove/{{path:.+}}")
    @PreAuthorize("hasRole('ADMIN')")
    public ResponseEntity<String> removeFile(@PathVariable String path) {{
        // Path traversal in annotated method
        File file = new File("/data/" + path);
        file.delete();
        return ResponseEntity.ok("Deleted");
    }}
}}

@Entity
@Table(name = "annotation_data_{}")
public class AnnotationEntity{} {{
    
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    
    @NotNull
    @Size(min = 1, max = 255)
    @Column(name = "sensitive_data")
    private String sensitiveData;
    
    @Email
    @Column(unique = true)
    private String email;
    
    @Pattern(regexp = "^[0-9]{{3}}-[0-9]{{2}}-[0-9]{{4}}$")
    @Column(name = "ssn")
    private String ssn;
}}

public interface AnnotationRepository{} extends JpaRepository<AnnotationEntity{}, Long> {{
    
    @Query(value = "SELECT * FROM annotation_data_{} WHERE data = ?1 ORDER BY ?2", nativeQuery = true)
    List<AnnotationEntity{}> findByDataWithSort(String data, String sortColumn);
    
    @Modifying
    @Query(value = "UPDATE annotation_data_{} SET data = ?2 WHERE id = ?1", nativeQuery = true)
    void updateData(Long id, String data);
}}

"#, i, i, i, i, i, i, i, i, i));
    }
    
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("spring_annotations.java");
    std::fs::write(&test_file, &annotation_code)?;
    
    let start_time = Instant::now();
    
    let mut parser = CodeParser::new()?;
    parser.add_file(&test_file)?;
    let _context = parser.build_context_from_file(&test_file)?;
    
    let duration = start_time.elapsed();
    let lines = annotation_code.lines().count();
    let annotations_count = annotation_code.matches("@").count();
    let speed = lines as f64 / duration.as_secs_f64();
    
    println!("   📊 Spring Boot Annotation Analysis:");
    println!("      ├─ Lines: {} lines", lines);
    println!("      ├─ Annotations: {} annotations", annotations_count);
    println!("      ├─ Controllers: 25");
    println!("      ├─ Time: {:.3} seconds", duration.as_secs_f64());
    println!("      └─ Speed: {:.1} lines/second", speed);
    
    // Annotation processing should be efficient despite complexity
    assert!(
        speed > 80.0,
        "Spring annotation processing too slow: {:.1} lines/s (minimum: 80 lines/s)",
        speed
    );
    
    println!("   ✅ Spring Boot annotation processing performance acceptable");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    Ok(())
}

#[tokio::test]
async fn test_spring_boot_jpa_repository_performance() -> Result<()> {
    println!("\n🗃️ Spring Boot JPA Repository Performance Test (API-free)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Generate JPA repository interfaces with vulnerabilities
    let mut jpa_code = String::new();
    jpa_code.push_str(r#"
package com.example.jpa;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.repository.query.Param;

"#);
    
    for i in 0..30 {
        jpa_code.push_str(&format!(r#"
public interface Repository{i} extends JpaRepository<Entity{i}, Long> {{
    
    // Native query with SQL injection risk
    @Query(value = "SELECT * FROM table_{i} WHERE name = ?1 AND status = ?2 ORDER BY ?3", nativeQuery = true)
    List<Entity{i}> findByNameAndStatusWithSort(String name, String status, String sortColumn);
    
    // Dynamic query construction
    @Query(value = "SELECT * FROM table_{i} WHERE ?1 = ?2", nativeQuery = true)
    List<Entity{i}> findByDynamicColumn(String column, String value);
    
    // Modifying query with injection risk
    @Modifying
    @Query(value = "UPDATE table_{i} SET status = ?2 WHERE user_id IN (?1)", nativeQuery = true)
    void updateStatusForUsers(String userIds, String status);
    
    // JPQL with potential injection
    @Query("SELECT e FROM Entity{i} e WHERE e.data LIKE CONCAT('%', :search, '%') ORDER BY :sortBy")
    List<Entity{i}> searchEntitiesWithSort(@Param("search") String search, @Param("sortBy") String sortBy);
    
    // Bulk operation with risk
    @Modifying
    @Query(value = "DELETE FROM table_{i} WHERE created_at < ?1 AND type = ?2", nativeQuery = true)
    int bulkDeleteOldRecords(String date, String type);
}}

@Entity
@Table(name = "table_{i}")
public class Entity{i} {{
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;
    
    private String name;
    private String data;
    private String status;
    private String sensitiveInfo;
}}

"#, i=i));
    }
    
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("jpa_repositories.java");
    std::fs::write(&test_file, &jpa_code)?;
    
    let start_time = Instant::now();
    
    let mut parser = CodeParser::new()?;
    parser.add_file(&test_file)?;
    let _context = parser.build_context_from_file(&test_file)?;
    
    let duration = start_time.elapsed();
    let lines = jpa_code.lines().count();
    let repositories_count = jpa_code.matches("interface Repository").count();
    let queries_count = jpa_code.matches("@Query").count();
    let speed = lines as f64 / duration.as_secs_f64();
    
    println!("   📊 JPA Repository Analysis:");
    println!("      ├─ Lines: {} lines", lines);
    println!("      ├─ Repositories: {} interfaces", repositories_count);
    println!("      ├─ Queries: {} @Query annotations", queries_count);
    println!("      ├─ Time: {:.3} seconds", duration.as_secs_f64());
    println!("      └─ Speed: {:.1} lines/second", speed);
    
    // JPA repository analysis should handle complex query patterns efficiently
    assert!(
        speed > 100.0,
        "JPA repository analysis too slow: {:.1} lines/s (minimum: 100 lines/s)",
        speed
    );
    
    println!("   ✅ Spring Boot JPA repository analysis performance acceptable");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    Ok(())
}