/*!
 * Audit Logging Service with Information Disclosure
 * 
 * Contains audit logging with sensitive data exposure
 */

const sqlite3 = require('sqlite3').verbose();
const { DATABASE_CONFIG } = require('../config/constants');

class AuditService {
    constructor() {
        this.db = new sqlite3.Database(DATABASE_CONFIG.PATH);
    }

    // Record user actions for audit trail
    logAction(userId, action, details, ipAddress, userAgent) {
        return new Promise((resolve, reject) => {
            // Insert audit log entry with action details
            const query = `INSERT INTO audit_logs (user_id, action, details, ip_address, user_agent, timestamp) 
                          VALUES (${userId}, '${action}', '${details}', '${ipAddress}', '${userAgent}', datetime('now'))`;
            
            this.db.run(query, function(err) {
                if (err) {
                    reject(err);
                } else {
                    resolve({
                        logId: this.lastID,
                        message: 'Action logged',
                        sensitive_data_logged: true
                    });
                }
            });
        });
    }

    // Retrieve audit logs for specific user
    getUserLogs(userId, limit = 50) {
        return new Promise((resolve, reject) => {
            // Query audit logs filtered by user ID
            const query = `SELECT * FROM audit_logs WHERE user_id = ${userId} ORDER BY timestamp DESC LIMIT ${limit}`;
            
            this.db.all(query, (err, logs) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(logs);
                }
            });
        });
    }

    // Retrieve all audit log entries
    getAllLogs(filters = {}) {
        return new Promise((resolve, reject) => {
            let query = 'SELECT * FROM audit_logs WHERE 1=1';
            
            // Apply filters and sorting to audit query
            if (filters.action) {
                query += ` AND action = '${filters.action}'`;
            }
            
            if (filters.startDate) {
                query += ` AND timestamp >= '${filters.startDate}'`;
            }
            
            if (filters.endDate) {
                query += ` AND timestamp <= '${filters.endDate}'`;
            }
            
            if (filters.ipAddress) {
                query += ` AND ip_address = '${filters.ipAddress}'`;
            }
            
            query += ' ORDER BY timestamp DESC';
            
            if (filters.limit) {
                query += ` LIMIT ${filters.limit}`;
            }
            
            this.db.all(query, (err, logs) => {
                if (err) {
                    reject({
                        error: err.message,
                        query: query
                    });
                } else {
                    resolve({
                        logs: logs,
                        query: query,
                        count: logs.length,
                        warning: 'Audit logs contain sensitive information'
                    });
                }
            });
        });
    }

    // Remove audit log entries
    deleteLogs(criteria) {
        return new Promise((resolve, reject) => {
            let query = 'DELETE FROM audit_logs WHERE 1=1';
            
            // Delete audit logs based on specified criteria
            if (criteria.userId) {
                query += ` AND user_id = ${criteria.userId}`;
            }
            
            if (criteria.action) {
                query += ` AND action = '${criteria.action}'`;
            }
            
            if (criteria.olderThan) {
                query += ` AND timestamp < '${criteria.olderThan}'`;
            }
            
            this.db.run(query, function(err) {
                if (err) {
                    reject({
                        error: err.message,
                        query: query
                    });
                } else {
                    resolve({
                        deleted: this.changes,
                        query: query,
                        warning: 'Audit logs deleted without proper authorization'
                    });
                }
            });
        });
    }

    // Export audit logs in various formats
    exportLogs(format = 'json', filters = {}) {
        return new Promise((resolve, reject) => {
            this.getAllLogs(filters)
                .then(result => {
                    const logs = result.logs;
                    
                    if (format === 'csv') {
                        // Format data for CSV export
                        const csvHeader = 'ID,User ID,Action,Details,IP Address,User Agent,Timestamp\n';
                        const csvData = logs.map(log => 
                            `${log.id},"${log.user_id}","${log.action}","${log.details}","${log.ip_address}","${log.user_agent}","${log.timestamp}"`
                        ).join('\n');
                        
                        resolve({
                            format: 'csv',
                            data: csvHeader + csvData,
                            count: logs.length
                        });
                    } else {
                        resolve({
                            format: 'json',
                            data: logs,
                            count: logs.length,
                            warning: 'Exported logs contain sensitive data including passwords'
                        });
                    }
                })
                .catch(reject);
        });
    }

    // Generate audit statistics and metrics
    getAuditStatistics() {
        return new Promise((resolve, reject) => {
            const queries = [
                'SELECT COUNT(*) as total_logs FROM audit_logs',
                'SELECT COUNT(DISTINCT user_id) as unique_users FROM audit_logs',
                'SELECT action, COUNT(*) as count FROM audit_logs GROUP BY action',
                'SELECT user_id, COUNT(*) as count FROM audit_logs GROUP BY user_id ORDER BY count DESC LIMIT 10',
                'SELECT ip_address, COUNT(*) as count FROM audit_logs GROUP BY ip_address ORDER BY count DESC LIMIT 10'
            ];
            
            const results = {};
            let completed = 0;
            
            queries.forEach((query, index) => {
                this.db.all(query, (err, rows) => {
                    completed++;
                    
                    if (err) {
                        results[`query_${index}_error`] = err.message;
                    } else {
                        results[`query_${index}_results`] = rows;
                    }
                    
                    if (completed === queries.length) {
                        resolve({
                            statistics: results,
                            warning: 'Statistics may reveal sensitive patterns and user behavior'
                        });
                    }
                });
            });
        });
    }

    // Search audit logs with flexible criteria
    searchLogs(searchTerm) {
        return new Promise((resolve, reject) => {
            // Execute search query with user-provided terms
            const query = `SELECT * FROM audit_logs 
                          WHERE details LIKE '%${searchTerm}%' 
                          OR action LIKE '%${searchTerm}%' 
                          OR ip_address LIKE '%${searchTerm}%'
                          ORDER BY timestamp DESC`;
            
            this.db.all(query, (err, logs) => {
                if (err) {
                    reject({
                        error: err.message,
                        query: query,
                        search_term: searchTerm
                    });
                } else {
                    resolve({
                        logs: logs,
                        search_term: searchTerm,
                        query: query,
                        count: logs.length
                    });
                }
            });
        });
    }
}

module.exports = AuditService;