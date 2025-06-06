/*!
 * Clean Architecture Main Application
 * 
 * Enterprise-level Node.js application with Clean Architecture
 * Contains intentional security vulnerabilities for testing
 */

const express = require('express');
const bodyParser = require('body-parser');
const cookieParser = require('cookie-parser');
const session = require('express-session');
const cors = require('cors');

// Infrastructure Layer
const DatabaseInitializer = require('./config/database');
const UserRepository = require('./infrastructure/database/UserRepository');

// Domain Layer
const AuthenticationService = require('./domain/services/AuthenticationService');

// Application Layer
const AuthenticateUser = require('./application/usecases/AuthenticateUser');
const CreateUser = require('./application/usecases/CreateUser');

// Presentation Layer
const AuthController = require('./presentation/controllers/AuthController');

// Services
const AuditService = require('./services/audit');

// Middleware
const jwtHandler = require('./middleware/jwt');
const sessionManager = require('./middleware/session');
const rateLimiter = require('./middleware/ratelimit');

// Routes
const authRoutes = require('./routes/auth');
const fileRoutes = require('./routes/files');
const injectionRoutes = require('./routes/injection');

// Utils
const validators = require('./utils/validators');
const encryption = require('./utils/encryption');

// Configuration
const { SESSION_CONFIG, RATE_LIMIT_CONFIG } = require('./config/constants');

class VulnerableApplication {
    constructor() {
        this.app = express();
        this.port = process.env.PORT || 3000;
        this.setupDependencies();
        this.setupMiddleware();
        this.setupRoutes();
        this.setupErrorHandling();
    }

    // Dependency Injection Container (with vulnerabilities)
    setupDependencies() {
        console.log('Setting up dependencies...');

        // Infrastructure
        this.dbInitializer = new DatabaseInitializer();
        this.userRepository = new UserRepository();

        // Services
        this.auditService = new AuditService();
        this.authenticationService = new AuthenticationService(
            this.userRepository, 
            this.auditService
        );

        // Use Cases
        this.authenticateUserUseCase = new AuthenticateUser(
            this.userRepository,
            this.authenticationService,
            this.auditService
        );

        this.createUserUseCase = new CreateUser(
            this.userRepository,
            this.auditService,
            validators
        );

        // Controllers
        this.authController = new AuthController(
            this.authenticateUserUseCase,
            this.createUserUseCase,
            validators
        );

        console.log('Dependencies initialized with vulnerabilities intact');
    }

    // Vulnerable middleware configuration
    setupMiddleware() {
        console.log('Configuring middleware...');

        // Vulnerable: CORS allows all origins
        this.app.use(cors({
            origin: true,
            credentials: true,
            methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
            allowedHeaders: ['*']
        }));

        // Vulnerable: Large payload limits
        this.app.use(bodyParser.json({ limit: '50mb' }));
        this.app.use(bodyParser.urlencoded({ extended: true, limit: '50mb' }));
        this.app.use(cookieParser());

        // Vulnerable: Session configuration
        this.app.use(session({
            secret: SESSION_CONFIG.SECRET,
            resave: false,
            saveUninitialized: true,
            cookie: {
                secure: SESSION_CONFIG.SECURE,
                httpOnly: SESSION_CONFIG.HTTP_ONLY,
                maxAge: SESSION_CONFIG.MAX_AGE
            }
        }));

        // Vulnerable: Rate limiting middleware
        this.app.use(rateLimiter.rateLimitMiddleware(
            RATE_LIMIT_CONFIG.DEFAULT_LIMIT,
            RATE_LIMIT_CONFIG.WINDOW_MS
        ));

        // Vulnerable: Request logging with sensitive data
        this.app.use((req, res, next) => {
            console.log(`${req.method} ${req.path}`, {
                body: req.body,
                query: req.query,
                headers: req.headers,
                session: req.session
            });
            next();
        });

        console.log('Middleware configured with security vulnerabilities');
    }

    // Route configuration with Clean Architecture
    setupRoutes() {
        console.log('Setting up routes...');

        // Health check with information disclosure
        this.app.get('/health', (req, res) => {
            res.json({
                status: 'healthy',
                timestamp: new Date().toISOString(),
                // Vulnerable: Expose system information
                system: {
                    memory: process.memoryUsage(),
                    uptime: process.uptime(),
                    version: process.version,
                    environment: process.env
                },
                dependencies: {
                    database: 'connected',
                    userRepository: 'initialized',
                    authService: 'running'
                }
            });
        });

        // Clean Architecture API endpoints
        this.app.post('/api/v1/auth/login', this.authController.login.bind(this.authController));
        this.app.post('/api/v1/auth/register', this.authController.register.bind(this.authController));
        this.app.post('/api/v1/auth/reset-password', this.authController.resetPassword.bind(this.authController));
        this.app.post('/api/v1/auth/change-password', this.authController.changePassword.bind(this.authController));
        this.app.post('/api/v1/auth/admin-login', this.authController.adminLogin.bind(this.authController));
        this.app.post('/api/v1/auth/logout', this.authController.logout.bind(this.authController));

        // Debug endpoint (should not be in production)
        this.app.get('/api/v1/debug', this.authController.debug.bind(this.authController));

        // Legacy routes (maintaining backward compatibility)
        this.app.use('/api/auth', authRoutes);
        this.app.use('/api/files', fileRoutes);
        this.app.use('/api/injection', injectionRoutes);

        // Vulnerable: Direct service access endpoints
        this.app.get('/api/v1/services/encryption/demo', (req, res) => {
            const { action, data } = req.query;
            
            switch (action) {
                case 'encrypt':
                    res.json(encryption.encrypt(data || 'test data'));
                    break;
                case 'hash':
                    res.json(encryption.hash(data || 'test', 'md5'));
                    break;
                case 'hmac':
                    res.json(encryption.generateHMAC(data || 'test'));
                    break;
                default:
                    res.json({
                        message: 'Encryption service demo',
                        actions: ['encrypt', 'hash', 'hmac']
                    });
            }
        });

        // Vulnerable: Validation service exposure
        this.app.post('/api/v1/services/validation/test', (req, res) => {
            const { type, input } = req.body;
            
            let result;
            switch (type) {
                case 'xss':
                    result = validators.validateXSS(input);
                    break;
                case 'sql':
                    result = validators.validateSQL(input);
                    break;
                case 'email':
                    result = validators.validateEmail(input);
                    break;
                case 'password':
                    result = validators.validatePassword(input);
                    break;
                default:
                    result = { error: 'Invalid validation type' };
            }
            
            res.json(result);
        });

        // Vulnerable: Direct database access
        this.app.post('/api/v1/db/query', async (req, res) => {
            const { query, params } = req.body;
            
            try {
                const result = await this.userRepository.executeQuery(query, params);
                res.json({
                    query: query,
                    results: result,
                    warning: 'Direct database access - SQL injection possible'
                });
            } catch (error) {
                res.status(500).json({
                    error: error.message,
                    query: query
                });
            }
        });

        // Main application route
        this.app.get('/', (req, res) => {
            res.send(`
                <h1>🏢 Enterprise Vulnerable Application</h1>
                <h2>Clean Architecture Implementation</h2>
                <p>This application demonstrates Clean Architecture with intentional security vulnerabilities.</p>
                
                <h3>Architecture Layers:</h3>
                <ul>
                    <li><strong>Domain Layer:</strong> Entities, Repositories, Services</li>
                    <li><strong>Application Layer:</strong> Use Cases, Business Logic</li>
                    <li><strong>Infrastructure Layer:</strong> Database, External Services</li>
                    <li><strong>Presentation Layer:</strong> Controllers, HTTP Handlers</li>
                </ul>
                
                <h3>API Endpoints:</h3>
                <ul>
                    <li>POST /api/v1/auth/login - User authentication</li>
                    <li>POST /api/v1/auth/register - User registration</li>
                    <li>GET /api/v1/debug - Debug information</li>
                    <li>POST /api/v1/db/query - Direct database access</li>
                    <li>GET /health - System health check</li>
                </ul>
                
                <h3>⚠️ Security Warning:</h3>
                <p style="color: red;">This application contains intentional security vulnerabilities for testing purposes only.</p>
            `);
        });

        console.log('Routes configured with Clean Architecture pattern');
    }

    // Vulnerable error handling
    setupErrorHandling() {
        // Vulnerable: Detailed error disclosure
        this.app.use((error, req, res, next) => {
            console.error('Application error:', error);

            res.status(500).json({
                error: error.message,
                stack: error.stack,
                request: {
                    method: req.method,
                    url: req.url,
                    body: req.body,
                    headers: req.headers
                },
                timestamp: new Date().toISOString()
            });
        });

        // Vulnerable: 404 handler with request disclosure
        this.app.use((req, res) => {
            res.status(404).json({
                error: 'Not Found',
                requestedUrl: req.url,
                method: req.method,
                headers: req.headers,
                body: req.body,
                suggestions: [
                    'Try /api/v1/auth/login',
                    'Try /api/v1/debug',
                    'Try SQL injection in endpoints'
                ]
            });
        });
    }

    // Initialize and start application
    async initialize() {
        try {
            console.log('Initializing application...');

            // Initialize database
            await this.dbInitializer.initializeTables();
            await this.dbInitializer.insertDefaultData();

            console.log('Database initialized with vulnerable default data');

            // Start server
            this.server = this.app.listen(this.port, () => {
                console.log(`🏢 Enterprise Vulnerable Application (Clean Architecture)`);
                console.log(`⚠️  Contains intentional security vulnerabilities for testing!`);
                console.log(`🌐 Server running at http://localhost:${this.port}`);
                console.log(`🔗 Health check: http://localhost:${this.port}/health`);
                console.log(`🔍 Debug endpoint: http://localhost:${this.port}/api/v1/debug`);
                console.log(`📊 Architecture: Domain → Application → Infrastructure → Presentation`);
            });

        } catch (error) {
            console.error('Failed to initialize application:', error);
            process.exit(1);
        }
    }

    // Graceful shutdown
    async shutdown() {
        console.log('Shutting down application...');
        if (this.server) {
            this.server.close();
        }
        process.exit(0);
    }
}

// Export for testing
module.exports = VulnerableApplication;

// Start application if run directly
if (require.main === module) {
    const app = new VulnerableApplication();
    
    // Handle shutdown signals
    process.on('SIGTERM', () => app.shutdown());
    process.on('SIGINT', () => app.shutdown());
    
    app.initialize().catch(console.error);
}