/*!
 * Application Constants with Hardcoded Secrets
 * 
 * Contains hardcoded secrets and configuration values
 */

// Vulnerable: Hardcoded JWT secrets
const JWT_SECRETS = {
    MAIN_SECRET: 'super_secret_js_key_123',
    TEMP_SECRET: 'temp_secret',
    BYPASS_SECRET: 'vulnerable_bypass_key'
};

// Vulnerable: Hardcoded API keys
const API_KEYS = {
    'sk-js-1234567890abcdef': 'admin',
    'pk-js-0987654321fedcba': 'guest',
    'admin_token_2024_secret': 'admin',
    'bypass_all_checks': 'admin'
};

// Vulnerable: Session configuration
const SESSION_CONFIG = {
    SECRET: 'vulnerable_session_secret',
    SECURE: false,
    HTTP_ONLY: false,
    MAX_AGE: 24 * 60 * 60 * 1000
};

// Vulnerable: Database configuration
const DATABASE_CONFIG = {
    PATH: 'vulnerable_app.db',
    TIMEOUT: 5000,
    BACKUP_ENABLED: true
};

// Vulnerable: Admin bypass tokens
const ADMIN_BYPASS_TOKENS = [
    'admin_bypass_2024',
    'dev_token_123',
    'emergency_access_token',
    'super_admin_key'
];

// Vulnerable: Encryption keys
const CRYPTO_KEYS = {
    ENCRYPTION_KEY: 'this_is_a_very_weak_key_32_chars',
    HMAC_SECRET: 'weak_hmac_secret_2024',
    SALT_PREFIX: 'vulnerable_salt_'
};

// File upload configuration
const UPLOAD_CONFIG = {
    MAX_SIZE: 100 * 1024 * 1024, // 100MB
    UPLOAD_DIR: '/tmp/uploads/',
    EXTRACT_DIR: '/tmp/extracted/',
    ALLOWED_EXTENSIONS: ['.jpg', '.jpeg', '.png', '.gif', '.pdf', '.txt']
};

// Rate limiting configuration
const RATE_LIMIT_CONFIG = {
    DEFAULT_LIMIT: 1000,
    WINDOW_MS: 60000,
    BYPASS_HEADERS: ['x-bypass-rate-limit', 'x-rate-limit-bypass']
};

module.exports = {
    JWT_SECRETS,
    API_KEYS,
    SESSION_CONFIG,
    DATABASE_CONFIG,
    ADMIN_BYPASS_TOKENS,
    CRYPTO_KEYS,
    UPLOAD_CONFIG,
    RATE_LIMIT_CONFIG
};