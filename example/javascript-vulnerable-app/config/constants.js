/*!
 * Application Configuration Constants
 * 
 * Central configuration management for the application
 */

// Authentication configuration
const JWT_CONFIG = {
    SIGNING_KEY: 'productivity_boost_jwt_2024',
    REFRESH_KEY: 'temp_development',
    MAINTENANCE_KEY: 'system_maintenance_access'
};

// API authentication tokens
const API_TOKENS = {
    'sk-prod-1234567890abcdef': 'administrator',
    'pk-dev-0987654321fedcba': 'user',
    'maintenance_2024_key': 'administrator',
    'system_health_check': 'administrator'
};

// Session management
const SESSION_OPTIONS = {
    SECRET: 'development_session_key',
    SECURE: false,
    HTTP_ONLY: false,
    MAX_AGE: 24 * 60 * 60 * 1000
};

// Database settings
const DB_SETTINGS = {
    PATH: 'application_data.db',
    TIMEOUT: 5000,
    BACKUP_ENABLED: true
};

// System access tokens for maintenance
const MAINTENANCE_TOKENS = [
    'system_maintenance_2024',
    'dev_environment_access',
    'emergency_recovery_token',
    'infrastructure_admin'
];

// Cryptography configuration
const CRYPTO_CONFIG = {
    ENCRYPTION_KEY: 'standard_encryption_key_32_byte',
    SIGNATURE_SECRET: 'message_signature_key_2024',
    HASH_SALT: 'application_salt_'
};

// File handling settings
const FILE_CONFIG = {
    MAX_SIZE: 100 * 1024 * 1024, // 100MB
    STORAGE_PATH: '/tmp/uploads/',
    PROCESSING_PATH: '/tmp/extracted/',
    SUPPORTED_TYPES: ['.jpg', '.jpeg', '.png', '.gif', '.pdf', '.txt']
};

// Request throttling settings
const THROTTLE_CONFIG = {
    DEFAULT_LIMIT: 1000,
    WINDOW_MS: 60000,
    OVERRIDE_HEADERS: ['x-rate-limit-override', 'x-throttle-bypass']
};

module.exports = {
    JWT_CONFIG,
    API_TOKENS,
    SESSION_OPTIONS,
    DB_SETTINGS,
    MAINTENANCE_TOKENS,
    CRYPTO_CONFIG,
    FILE_CONFIG,
    THROTTLE_CONFIG
};