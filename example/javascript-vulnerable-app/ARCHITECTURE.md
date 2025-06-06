# Enterprise Vulnerable Application - Clean Architecture

This application demonstrates Clean Architecture implementation with intentional security vulnerabilities for testing advanced security analysis tools.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Presentation Layer                      │
│  ┌─────────────────┐ ┌─────────────────┐ ┌───────────────┐  │
│  │   Controllers   │ │   Middleware    │ │    Routes     │  │
│  │                 │ │                 │ │               │  │
│  │ - AuthController│ │ - JWT Handler   │ │ - auth.js     │  │
│  │ - FileController│ │ - Session Mgmt  │ │ - files.js    │  │
│  │ - UserController│ │ - Rate Limiter  │ │ - injection.js│  │
│  └─────────────────┘ └─────────────────┘ └───────────────┘  │
└─────────────────────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                       │
│  ┌─────────────────┐ ┌─────────────────┐ ┌───────────────┐  │
│  │   Use Cases     │ │   Services      │ │   DTOs        │  │
│  │                 │ │                 │ │               │  │
│  │ - AuthenticateUser│ │ - ValidationSvc│ │ - UserDto     │  │
│  │ - CreateUser    │ │ - AuditService  │ │ - AuthDto     │  │
│  │ - UpdateUser    │ │ - CryptoService │ │ - ErrorDto    │  │
│  └─────────────────┘ └─────────────────┘ └───────────────┘  │
└─────────────────────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                      Domain Layer                           │
│  ┌─────────────────┐ ┌─────────────────┐ ┌───────────────┐  │
│  │    Entities     │ │   Repositories  │ │   Services    │  │
│  │                 │ │   (Interfaces)  │ │               │  │
│  │ - User          │ │ - IUserRepo     │ │ - AuthService │  │
│  │ - Document      │ │ - IDocumentRepo │ │ - CryptoSvc   │  │
│  │ - AuditLog      │ │ - IAuditRepo    │ │ - BusinessSvc │  │
│  └─────────────────┘ └─────────────────┘ └───────────────┘  │
└─────────────────────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                   Infrastructure Layer                      │
│  ┌─────────────────┐ ┌─────────────────┐ ┌───────────────┐  │
│  │   Repositories  │ │    Database     │ │   External    │  │
│  │ (Implementations)│ │                 │ │   Services    │  │
│  │                 │ │ - SQLite        │ │               │  │
│  │ - UserRepository│ │ - Connection    │ │ - FileSystem  │  │
│  │ - DocumentRepo  │ │ - Migrations    │ │ - HTTP Client │  │
│  │ - AuditRepo     │ │ - Seeds         │ │ - External API│  │
│  └─────────────────┘ └─────────────────┘ └───────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
javascript-vulnerable-app/
├── app.js                          # Legacy monolithic application
├── app-clean.js                    # Clean Architecture application
├── package.json                    # Dependencies
├── ARCHITECTURE.md                 # This file
│
├── config/                         # Configuration Layer
│   ├── constants.js                # Application constants
│   └── database.js                 # Database configuration
│
├── domain/                         # Domain Layer (Business Logic)
│   ├── entities/                   # Business Entities
│   │   ├── User.js                 # User entity with business rules
│   │   └── Document.js             # Document entity
│   ├── repositories/               # Repository Interfaces
│   │   └── IUserRepository.js      # User repository contract
│   └── services/                   # Domain Services
│       └── AuthenticationService.js # Authentication business logic
│
├── application/                    # Application Layer (Use Cases)
│   └── usecases/                   # Application Use Cases
│       ├── AuthenticateUser.js     # User authentication use case
│       └── CreateUser.js           # User creation use case
│
├── infrastructure/                 # Infrastructure Layer (External Concerns)
│   └── database/                   # Database Implementations
│       └── UserRepository.js       # User repository implementation
│
├── presentation/                   # Presentation Layer (UI/API)
│   └── controllers/                # HTTP Controllers
│       └── AuthController.js       # Authentication HTTP handlers
│
├── middleware/                     # Cross-cutting Concerns
│   ├── auth.js                     # Authentication middleware
│   ├── jwt.js                      # JWT handling
│   ├── session.js                  # Session management
│   ├── ratelimit.js               # Rate limiting
│   └── validation.js              # Input validation
│
├── routes/                        # Route Definitions
│   ├── auth.js                    # Authentication routes
│   ├── files.js                   # File operation routes
│   ├── injection.js               # Injection test routes
│   ├── advanced.js                # Advanced vulnerability routes
│   └── bypass.js                  # Bypass demonstration routes
│
├── services/                      # Service Layer (Legacy)
│   ├── audit.js                   # Audit logging service
│   ├── crypto.js                  # Cryptographic service
│   ├── database.js                # Database service
│   └── user.js                    # User service
│
└── utils/                         # Utility Functions
    ├── validators.js              # Input validation utilities
    └── encryption.js              # Encryption utilities
```

## Layer Responsibilities

### 🎯 Domain Layer (Core Business Logic)
- **Entities**: Core business objects with behavior
- **Repository Interfaces**: Data access contracts
- **Domain Services**: Business logic that doesn't fit in entities
- **Business Rules**: Validation and business constraints

**Vulnerabilities**: Business logic flaws, weak validation rules

### 📋 Application Layer (Use Cases)
- **Use Cases**: Application-specific business rules
- **Application Services**: Orchestrate domain objects
- **DTOs**: Data transfer between layers
- **Workflow Management**: Complex business processes

**Vulnerabilities**: Use case logic flaws, insufficient authorization

### 🏗️ Infrastructure Layer (External Concerns)
- **Repository Implementations**: Data persistence
- **Database Access**: SQL execution and data mapping
- **External Services**: File system, HTTP clients
- **Framework Adapters**: Technology-specific implementations

**Vulnerabilities**: SQL injection, data exposure, insecure connections

### 🌐 Presentation Layer (User Interface)
- **Controllers**: HTTP request/response handling
- **Middleware**: Cross-cutting concerns
- **Routes**: URL mapping and request routing
- **Input Validation**: Request data validation

**Vulnerabilities**: Input validation bypass, information disclosure

## Security Testing Focus Areas

### 🔍 Cross-Layer Vulnerability Analysis
1. **Data Flow**: Trace vulnerabilities across architectural layers
2. **Boundary Violations**: Dependencies between layers
3. **Abstraction Leaks**: Business logic in wrong layers
4. **Security Boundaries**: Authentication and authorization at each layer

### 🎯 Enterprise Patterns
1. **Dependency Injection**: Service resolution vulnerabilities
2. **Repository Pattern**: Data access security flaws
3. **Use Case Pattern**: Business logic vulnerabilities
4. **Controller Pattern**: Input handling security issues

### 🛡️ Advanced Testing Scenarios
1. **Multi-layer SQL Injection**: From presentation to infrastructure
2. **Business Logic Bypass**: Use case and domain rule violations
3. **Authentication Flow**: Cross-layer authentication vulnerabilities
4. **Data Validation**: Validation bypass across architectural boundaries

## Running the Application

### Clean Architecture Version
```bash
node app-clean.js
```

### Legacy Monolithic Version
```bash
node app.js
```

## API Endpoints

### Clean Architecture Endpoints
- `POST /api/v1/auth/login` - User authentication
- `POST /api/v1/auth/register` - User registration
- `GET /api/v1/debug` - Debug information
- `POST /api/v1/db/query` - Direct database access
- `GET /health` - System health check

### Legacy Endpoints
- `POST /api/auth/login` - Legacy authentication
- `GET /api/files/read` - File operations
- `POST /api/injection/command` - Injection testing

## Vulnerability Categories

### 🔓 Authentication & Authorization
- JWT vulnerabilities across layers
- Session management flaws
- Role-based access control bypass

### 💉 Injection Attacks
- SQL injection in repository layer
- Command injection in infrastructure
- Template injection in presentation

### 📊 Information Disclosure
- Sensitive data logging across layers
- Error message information leakage
- Debug endpoint exposure

### 🏢 Business Logic Flaws
- Use case authorization bypass
- Domain rule violations
- Workflow manipulation

## Security Analysis Recommendations

1. **Layer Isolation**: Test security boundaries between layers
2. **Data Flow Analysis**: Trace vulnerabilities through the architecture
3. **Dependency Analysis**: Check for security violations in dependencies
4. **Pattern Analysis**: Test enterprise patterns for security flaws

⚠️ **Warning**: This application contains severe security vulnerabilities by design. Use only in isolated testing environments.