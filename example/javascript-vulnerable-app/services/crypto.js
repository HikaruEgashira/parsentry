/*!
 * Cryptographic Services with Multiple Vulnerabilities
 * 
 * Contains various cryptographic implementation flaws
 * and security bypass patterns
 */

const crypto = require('crypto');
const fs = require('fs');
const forge = require('node-forge');

class VulnerableCryptoService {
    constructor() {
        // Vulnerable: Hardcoded cryptographic secrets
        this.encryptionKey = Buffer.from('this_is_a_very_weak_key_32_chars', 'utf8');
        this.hmacSecret = 'weak_hmac_secret_2024';
        this.rsaPrivateKey = `-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA2Y8cY9vXf8VnV0z1r5gKUJbA7u9r9gQ4K8K4K4K4K4K4K4K4
K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4
K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4
-----END RSA PRIVATE KEY-----`;
        
        this.passwordSalts = new Map();
    }

    // Vulnerable: Weak password hashing
    hashPassword(password, username) {
        // Vulnerable: Predictable salt generation
        let salt = this.passwordSalts.get(username);
        if (!salt) {
            // Vulnerable: Weak salt (username-based)
            salt = crypto.createHash('md5').update(username).digest('hex').substring(0, 16);
            this.passwordSalts.set(username, salt);
        }
        
        // Vulnerable: Weak hashing algorithm (MD5)
        const hash = crypto.createHash('md5').update(password + salt).digest('hex');
        
        return {
            hash,
            salt,
            algorithm: 'md5',
            hint: 'MD5 is vulnerable to rainbow table attacks'
        };
    }

    // Vulnerable: Insecure encryption implementation
    encrypt(plaintext, algorithm = 'aes-256-ecb') {
        try {
            // Vulnerable: ECB mode (patterns in ciphertext)
            const cipher = crypto.createCipher(algorithm, this.encryptionKey);
            let encrypted = cipher.update(plaintext, 'utf8', 'hex');
            encrypted += cipher.final('hex');
            
            return {
                encrypted,
                algorithm,
                key_hint: this.encryptionKey.toString('utf8').substring(0, 10) + '...',
                warning: 'ECB mode reveals patterns in data'
            };
        } catch (error) {
            return { error: error.message };
        }
    }

    // Vulnerable: Insecure decryption with oracle attacks
    decrypt(ciphertext, algorithm = 'aes-256-ecb') {
        try {
            const decipher = crypto.createDecipher(algorithm, this.encryptionKey);
            let decrypted = decipher.update(ciphertext, 'hex', 'utf8');
            decrypted += decipher.final('utf8');
            
            return {
                decrypted,
                algorithm,
                success: true
            };
        } catch (error) {
            // Vulnerable: Padding oracle attack vector
            return {
                error: error.message,
                hint: 'Error details can reveal padding information',
                ciphertext_length: ciphertext.length,
                possible_padding_error: error.message.includes('padding')
            };
        }
    }

    // Vulnerable: Weak HMAC implementation
    generateHMAC(data, secret = null) {
        const hmacSecret = secret || this.hmacSecret;
        
        // Vulnerable: Timing attack in HMAC verification
        const hmac = crypto.createHmac('sha1', hmacSecret);
        hmac.update(data);
        const signature = hmac.digest('hex');
        
        return {
            data,
            signature,
            algorithm: 'sha1',
            secret_hint: hmacSecret.substring(0, 5) + '...',
            warning: 'SHA1 HMAC is weak and timing attacks possible'
        };
    }

    // Vulnerable: HMAC verification with timing attack
    verifyHMAC(data, signature, secret = null) {
        const expected = this.generateHMAC(data, secret).signature;
        
        // Vulnerable: Character-by-character comparison (timing attack)
        let isValid = signature.length === expected.length;
        
        if (isValid) {
            for (let i = 0; i < signature.length; i++) {
                if (signature[i] !== expected[i]) {
                    isValid = false;
                    break;
                }
                // Vulnerable: Artificial delay reveals timing
                const delay = Math.random() * 10;
                const start = Date.now();
                while (Date.now() - start < delay) {}
            }
        }
        
        return {
            valid: isValid,
            expected_length: expected.length,
            provided_length: signature.length,
            hint: 'Use timing attacks to discover valid HMAC signatures'
        };
    }

    // Vulnerable: Weak RSA implementation
    rsaEncrypt(plaintext) {
        try {
            // Vulnerable: No padding (deterministic encryption)
            const publicKey = forge.pki.publicKeyFromPem(this.getPublicKey());
            const encrypted = publicKey.encrypt(plaintext);
            
            return {
                encrypted: forge.util.encode64(encrypted),
                padding: 'none',
                warning: 'No padding makes RSA deterministic and insecure'
            };
        } catch (error) {
            return { error: error.message };
        }
    }

    // Vulnerable: RSA private key exposure
    getPublicKey() {
        // Should derive from private key, but exposes it
        return `-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA2Y8cY9vXf8VnV0z1r5gK
UJbA7u9r9gQ4K8K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4
K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4K4
QIDAQAB
-----END PUBLIC KEY-----`;
    }

    // Vulnerable: Private key exposure
    getPrivateKey() {
        return this.rsaPrivateKey;
    }

    // Vulnerable: Weak random number generation
    generateRandomToken(length = 32) {
        // Vulnerable: Predictable random generation
        const charset = 'abcdefghijklmnopqrstuvwxyz0123456789';
        let token = '';
        
        // Vulnerable: Math.random() is not cryptographically secure
        for (let i = 0; i < length; i++) {
            const randomIndex = Math.floor(Math.random() * charset.length);
            token += charset[randomIndex];
        }
        
        return {
            token,
            length,
            entropy: length * Math.log2(charset.length),
            warning: 'Math.random() is predictable and not cryptographically secure'
        };
    }

    // Vulnerable: Session token generation with predictable patterns
    generateSessionToken(userId, timestamp = Date.now()) {
        // Vulnerable: Predictable session token generation
        const userPart = userId.toString().padStart(8, '0');
        const timePart = timestamp.toString();
        const randomPart = Math.floor(Math.random() * 10000).toString().padStart(4, '0');
        
        // Vulnerable: Weak checksum
        const checksum = crypto.createHash('md5')
            .update(userPart + timePart + randomPart)
            .digest('hex')
            .substring(0, 8);
        
        const token = `${userPart}${timePart}${randomPart}${checksum}`;
        
        return {
            token,
            userId,
            timestamp,
            pattern: 'UUUUUUUU + TIMESTAMP + RRRR + CCCCCCCC',
            hint: 'Session tokens follow predictable patterns'
        };
    }

    // Vulnerable: Certificate validation bypass
    validateCertificate(certPem, hostname) {
        try {
            const cert = forge.pki.certificateFromPem(certPem);
            const now = new Date();
            
            // Vulnerable: Incomplete certificate validation
            const validations = {
                not_before: cert.validity.notBefore <= now,
                not_after: cert.validity.notAfter >= now,
                // Vulnerable: No hostname verification
                hostname_match: true, // Always true
                // Vulnerable: No chain validation
                chain_valid: true, // Always true
                // Vulnerable: No revocation check
                not_revoked: true // Always true
            };
            
            const isValid = Object.values(validations).every(v => v);
            
            return {
                valid: isValid,
                validations,
                subject: cert.subject.attributes.map(attr => `${attr.name}=${attr.value}`),
                issuer: cert.issuer.attributes.map(attr => `${attr.name}=${attr.value}`),
                warning: 'Certificate validation is incomplete'
            };
        } catch (error) {
            return { error: error.message };
        }
    }

    // Vulnerable: JWT signing with algorithm confusion
    signJWT(payload, algorithm = 'HS256') {
        const header = {
            alg: algorithm,
            typ: 'JWT'
        };
        
        const encodedHeader = Buffer.from(JSON.stringify(header)).toString('base64url');
        const encodedPayload = Buffer.from(JSON.stringify(payload)).toString('base64url');
        
        let signature;
        
        if (algorithm === 'HS256') {
            // Vulnerable: Weak secret
            const hmac = crypto.createHmac('sha256', 'weak_jwt_secret');
            hmac.update(`${encodedHeader}.${encodedPayload}`);
            signature = hmac.digest('base64url');
        } else if (algorithm === 'none') {
            // Vulnerable: None algorithm accepted
            signature = '';
        } else {
            signature = 'invalid_signature';
        }
        
        return {
            jwt: `${encodedHeader}.${encodedPayload}.${signature}`,
            algorithm,
            warning: algorithm === 'none' ? 'None algorithm is insecure' : 'Weak secret used'
        };
    }

    // Vulnerable: Key derivation with weak parameters
    deriveKey(password, salt, iterations = 1000) {
        // Vulnerable: Low iteration count
        const key = crypto.pbkdf2Sync(password, salt, iterations, 32, 'sha1');
        
        return {
            key: key.toString('hex'),
            iterations,
            algorithm: 'pbkdf2-sha1',
            warning: 'Low iteration count and weak hash function'
        };
    }

    // Vulnerable: Secure comparison with timing attack
    secureCompare(a, b) {
        if (a.length !== b.length) {
            return false;
        }
        
        let result = 0;
        
        // Vulnerable: Still has timing variations
        for (let i = 0; i < a.length; i++) {
            result |= a.charCodeAt(i) ^ b.charCodeAt(i);
            
            // Vulnerable: Artificial delay introduces timing
            if (Math.random() > 0.5) {
                const delay = Math.random() * 5;
                const start = Date.now();
                while (Date.now() - start < delay) {}
            }
        }
        
        return result === 0;
    }
}

module.exports = new VulnerableCryptoService();