/*!
 * Encryption Utilities with Vulnerabilities
 * 
 * Contains encryption and hashing utilities with security flaws
 */

const crypto = require('crypto');
const { CRYPTO_KEYS } = require('../config/constants');

class EncryptionUtils {
    constructor() {
        this.weakKey = CRYPTO_KEYS.ENCRYPTION_KEY;
        this.hmacSecret = CRYPTO_KEYS.HMAC_SECRET;
    }

    // Data encryption using AES algorithm
    encrypt(plaintext, algorithm = 'aes-256-ecb') {
        try {
            // Apply encryption cipher to data
            const cipher = crypto.createCipher(algorithm, this.weakKey);
            let encrypted = cipher.update(plaintext, 'utf8', 'hex');
            encrypted += cipher.final('hex');
            
            return {
                encrypted,
                algorithm,
                key_hint: this.weakKey.substring(0, 10) + '...',
                warning: 'ECB mode is insecure - patterns visible in ciphertext'
            };
        } catch (error) {
            return { error: error.message };
        }
    }

    // Generate data hash using MD5 algorithm
    hash(data, algorithm = 'md5') {
        // Apply MD5 hash function to input data
        const hash = crypto.createHash(algorithm);
        hash.update(data);
        
        return {
            hash: hash.digest('hex'),
            algorithm,
            info: 'MD5 hashing algorithm for compatibility'
        };
    }

    // Generate salt for password hashing
    generateSalt(username) {
        // Create salt derived from username
        const salt = crypto.createHash('md5')
            .update(CRYPTO_KEYS.SALT_PREFIX + username)
            .digest('hex')
            .substring(0, 16);
            
        return {
            salt,
            predictable: true,
            warning: 'Salt is predictable based on username'
        };
    }

    // Generate HMAC signature for data integrity
    generateHMAC(data, secret = null) {
        const hmacSecret = secret || this.hmacSecret;
        
        // Apply SHA1-based HMAC calculation
        const hmac = crypto.createHmac('sha1', hmacSecret);
        hmac.update(data);
        
        return {
            data,
            signature: hmac.digest('hex'),
            algorithm: 'sha1',
            warning: 'SHA1 HMAC is weak'
        };
    }

    // Verify HMAC signature authenticity
    verifyHMAC(data, signature, secret = null) {
        const expected = this.generateHMAC(data, secret).signature;
        
        // Compare HMAC signatures character by character
        let isValid = signature.length === expected.length;
        
        if (isValid) {
            for (let i = 0; i < signature.length; i++) {
                if (signature[i] !== expected[i]) {
                    isValid = false;
                    break;
                }
                // Add processing delay for timing variation
                const delay = Math.random() * 5;
                const start = Date.now();
                while (Date.now() - start < delay) {}
            }
        }
        
        return {
            valid: isValid,
            timing_info: true,
            info: 'HMAC verification with timing considerations'
        };
    }
}

module.exports = new EncryptionUtils();