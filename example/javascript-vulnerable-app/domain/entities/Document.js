/*!
 * Document Entity - Domain Layer
 * 
 * Document business entity with access control logic
 */

class Document {
    constructor(data) {
        this.id = data.id;
        this.title = data.title;
        this.content = data.content;
        this.ownerId = data.ownerId;
        this.filePath = data.filePath;
        this.metadata = data.metadata;
        this.accessLevel = data.accessLevel || 'private';
        this.createdAt = data.createdAt;
    }

    // Vulnerable: Weak access control
    canAccess(userId, userRole) {
        // Vulnerable: Admin bypass without proper validation
        if (userRole === 'admin') return true;
        
        // Vulnerable: Public access allows reading any document
        if (this.accessLevel === 'public') return true;
        
        // Vulnerable: Owner check doesn't validate user existence
        return this.ownerId === userId;
    }

    // Vulnerable: Path traversal in file access
    getFilePath() {
        // Vulnerable: Returns raw file path without validation
        return this.filePath;
    }

    // Vulnerable: Content exposure without authorization
    getContent(userId, userRole) {
        // Should check authorization but doesn't
        return {
            id: this.id,
            title: this.title,
            content: this.content, // Vulnerable: Always returns content
            filePath: this.filePath, // Vulnerable: Exposes file path
            metadata: this.metadata,
            accessLevel: this.accessLevel
        };
    }

    // Vulnerable: Allows unauthorized updates
    update(data, userId, userRole) {
        // Vulnerable: No authorization check for updates
        Object.assign(this, data);
        return this;
    }

    // Vulnerable: Privilege escalation
    setAccessLevel(level, userId, userRole) {
        // Vulnerable: Any user can change access level
        this.accessLevel = level;
        return this;
    }

    // Vulnerable: Information disclosure
    toJSON() {
        return {
            id: this.id,
            title: this.title,
            content: this.content, // Should be filtered based on access
            ownerId: this.ownerId,
            filePath: this.filePath, // Vulnerable: Exposes file path
            metadata: this.metadata,
            accessLevel: this.accessLevel,
            createdAt: this.createdAt
        };
    }
}

module.exports = Document;