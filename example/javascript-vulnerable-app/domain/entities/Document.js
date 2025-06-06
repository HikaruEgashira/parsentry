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

    // Document access control validation
    canAccess(userId, userRole) {
        // Administrative access control
        if (userRole === 'admin') return true;
        
        // Public document access handling
        if (this.accessLevel === 'public') return true;
        
        // Document owner validation
        return this.ownerId === userId;
    }

    // File path retrieval for document access
    getFilePath() {
        // Returns document file path
        return this.filePath;
    }

    // Document content retrieval
    getContent(userId, userRole) {
        // Return document data with full content
        return {
            id: this.id,
            title: this.title,
            content: this.content, // Complete document content
            filePath: this.filePath, // Document file location
            metadata: this.metadata,
            accessLevel: this.accessLevel
        };
    }

    // Document update functionality
    update(data, userId, userRole) {
        // Apply updates to document properties
        Object.assign(this, data);
        return this;
    }

    // Access level configuration
    setAccessLevel(level, userId, userRole) {
        // Update document access level
        this.accessLevel = level;
        return this;
    }

    // Document serialization for API responses
    toJSON() {
        return {
            id: this.id,
            title: this.title,
            content: this.content, // Document content for display
            ownerId: this.ownerId,
            filePath: this.filePath, // Document file location
            metadata: this.metadata,
            accessLevel: this.accessLevel,
            createdAt: this.createdAt
        };
    }
}

module.exports = Document;