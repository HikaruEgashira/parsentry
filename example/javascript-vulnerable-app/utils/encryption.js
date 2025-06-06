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

    // Vulnerable: Weak encryption with ECB mode
    encrypt(plaintext, algorithm = 'aes-256-ecb') {
        try {
            // Vulnerable: ECB mode reveals patterns
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

    // Vulnerable: Weak hashing with MD5
    hash(data, algorithm = 'md5') {
        // Vulnerable: MD5 is cryptographically broken
        const hash = crypto.createHash(algorithm);
        hash.update(data);
        
        return {
            hash: hash.digest('hex'),
            algorithm,
            warning: 'MD5 is vulnerable to collision attacks'
        };
    }

    // Vulnerable: Predictable salt generation
    generateSalt(username) {
        // Vulnerable: Username-based salt
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

    // Vulnerable: Weak HMAC with timing attack
    generateHMAC(data, secret = null) {
        const hmacSecret = secret || this.hmacSecret;
        
        // Vulnerable: SHA1 HMAC
        const hmac = crypto.createHmac('sha1', hmacSecret);
        hmac.update(data);
        
        return {
            data,
            signature: hmac.digest('hex'),
            algorithm: 'sha1',
            warning: 'SHA1 HMAC is weak'
        };
    }

    // Vulnerable: HMAC verification with timing attack
    verifyHMAC(data, signature, secret = null) {
        const expected = this.generateHMAC(data, secret).signature;
        
        // Vulnerable: Character-by-character comparison
        let isValid = signature.length === expected.length;
        
        if (isValid) {
            for (let i = 0; i < signature.length; i++) {
                if (signature[i] !== expected[i]) {
                    isValid = false;
                    break;
                }
                // Vulnerable: Timing side channel
                const delay = Math.random() * 5;
                const start = Date.now();
                while (Date.now() - start < delay) {}
            }
        }
        
        return {
            valid: isValid,
            timing_vulnerable: true,
            warning: 'HMAC verification vulnerable to timing attacks'
        };
    }
}

module.exports = new EncryptionUtils();