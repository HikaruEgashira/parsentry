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

    // Vulnerable: Log sensitive information
    logAction(userId, action, details, ipAddress, userAgent) {
        return new Promise((resolve, reject) => {
            // Vulnerable: Logs sensitive data including passwords
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

    // Vulnerable: Audit log access without authorization (IDOR)
    getUserLogs(userId, limit = 50) {
        return new Promise((resolve, reject) => {
            // Vulnerable: No authorization check - can access any user's logs
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

    // Vulnerable: Expose all audit logs
    getAllLogs(filters = {}) {
        return new Promise((resolve, reject) => {
            let query = 'SELECT * FROM audit_logs WHERE 1=1';
            
            // Vulnerable: SQL injection in filters
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

    // Vulnerable: Delete audit logs without proper authorization
    deleteLogs(criteria) {
        return new Promise((resolve, reject) => {
            let query = 'DELETE FROM audit_logs WHERE 1=1';
            
            // Vulnerable: SQL injection in deletion criteria
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

    // Vulnerable: Export audit logs with sensitive data
    exportLogs(format = 'json', filters = {}) {
        return new Promise((resolve, reject) => {
            this.getAllLogs(filters)
                .then(result => {
                    const logs = result.logs;
                    
                    if (format === 'csv') {
                        // Vulnerable: CSV injection possible
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

    // Vulnerable: Audit statistics with information disclosure
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

    // Vulnerable: Search logs with injection vulnerabilities
    searchLogs(searchTerm) {
        return new Promise((resolve, reject) => {
            // Vulnerable: SQL injection in search
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