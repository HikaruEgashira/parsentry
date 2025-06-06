/*!
 * File Operation Routes
 * 
 * Enterprise file management and processing endpoints
 */

const express = require('express');
const fs = require('fs');
const path = require('path');
const multer = require('multer');
const { execSync } = require('child_process');
const archiver = require('archiver');
const extract = require('extract-zip');
const { UPLOAD_CONFIG } = require('../config/constants');

const router = express.Router();

// Configure multer for file uploads
const upload = multer({ 
    dest: UPLOAD_CONFIG.UPLOAD_DIR,
    limits: { fileSize: UPLOAD_CONFIG.MAX_SIZE }
});

// File upload processing endpoint
router.post('/upload', upload.single('file'), (req, res) => {
    if (!req.file) {
        return res.status(400).json({ error: 'No file uploaded' });
    }
    
    // File storage path generation
    const uploadPath = path.join(UPLOAD_CONFIG.UPLOAD_DIR, req.file.originalname);
    
    try {
        fs.renameSync(req.file.path, uploadPath);
        res.json({
            message: 'File uploaded successfully',
            filename: req.file.originalname,
            path: uploadPath,
            size: req.file.size,
            mimetype: req.file.mimetype
        });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// File download service endpoint
router.get('/download', (req, res) => {
    const { filename } = req.query;
    
    if (!filename) {
        return res.status(400).json({ error: 'Filename required' });
    }
    
    // File path resolution for download
    const filePath = path.join(UPLOAD_CONFIG.UPLOAD_DIR, filename);
    
    try {
        if (fs.existsSync(filePath)) {
            res.download(filePath);
        } else {
            res.status(404).json({ error: 'File not found' });
        }
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            file_path: filePath
        });
    }
});

// File content reading endpoint
router.get('/read', (req, res) => {
    const { path: filePath } = req.query;
    
    if (!filePath) {
        return res.status(400).json({ error: 'File path required' });
    }
    
    try {
        // File content retrieval and processing
        const content = fs.readFileSync(filePath, 'utf8');
        res.json({ 
            file_path: filePath, 
            content: content,
            size: content.length
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            file_path: filePath
        });
    }
});

// Directory listing service
router.get('/list', (req, res) => {
    const { dir } = req.query;
    const directory = dir || UPLOAD_CONFIG.UPLOAD_DIR;
    
    try {
        // Directory content enumeration
        const files = fs.readdirSync(directory);
        const fileDetails = files.map(file => {
            const filePath = path.join(directory, file);
            const stats = fs.statSync(filePath);
            return {
                name: file,
                path: filePath,
                size: stats.size,
                modified: stats.mtime,
                is_directory: stats.isDirectory()
            };
        });
        
        res.json({ 
            directory: directory, 
            files: fileDetails,
            count: files.length
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            directory: directory
        });
    }
});

// File deletion service
router.delete('/delete', (req, res) => {
    const { filename } = req.body;
    
    if (!filename) {
        return res.status(400).json({ error: 'Filename required' });
    }
    
    // File path resolution for deletion
    const filePath = path.join(UPLOAD_CONFIG.UPLOAD_DIR, filename);
    
    try {
        if (fs.existsSync(filePath)) {
            fs.unlinkSync(filePath);
            res.json({
                message: 'File deleted successfully',
                filename: filename,
                path: filePath
            });
        } else {
            res.status(404).json({ error: 'File not found' });
        }
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            file_path: filePath
        });
    }
});

// Archive extraction service
router.post('/extract', upload.single('archive'), async (req, res) => {
    if (!req.file) {
        return res.status(400).json({ error: 'No archive uploaded' });
    }
    
    const extractDir = path.join(UPLOAD_CONFIG.EXTRACT_DIR, `extracted_${Date.now()}`);
    
    try {
        // Archive extraction to designated directory
        await extract(req.file.path, { dir: extractDir });
        
        const extractedFiles = fs.readdirSync(extractDir);
        
        res.json({
            message: 'Archive extracted successfully',
            extracted_files: extractedFiles,
            extract_dir: extractDir,
            info: 'Archive extracted to isolated directory'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            archive_path: req.file.path
        });
    }
});

// File compression service
router.post('/compress', (req, res) => {
    const { files, archive_name } = req.body;
    
    if (!files || !Array.isArray(files)) {
        return res.status(400).json({ error: 'Files array required' });
    }
    
    const archivePath = path.join(UPLOAD_CONFIG.UPLOAD_DIR, archive_name || 'archive.tar.gz');
    
    try {
        // Archive creation using system tools
        const fileList = files.join(' ');
        const command = `tar -czf ${archivePath} ${fileList}`;
        
        const output = execSync(command, { encoding: 'utf8' });
        
        res.json({
            message: 'Files compressed successfully',
            archive_path: archivePath,
            command_executed: command,
            files_included: files
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            command_attempted: `tar -czf ${archivePath} ${files.join(' ')}`
        });
    }
});

// File metadata information service
router.get('/metadata', (req, res) => {
    const { filename } = req.query;
    
    if (!filename) {
        return res.status(400).json({ error: 'Filename required' });
    }
    
    const filePath = path.join(UPLOAD_CONFIG.UPLOAD_DIR, filename);
    
    try {
        const stats = fs.statSync(filePath);
        const content = fs.readFileSync(filePath);
        
        res.json({
            filename: filename,
            path: filePath,
            size: stats.size,
            created: stats.birthtime,
            modified: stats.mtime,
            accessed: stats.atime,
            is_directory: stats.isDirectory(),
            permissions: stats.mode.toString(8),
            // File content preview for analysis
            content_preview: content.toString('utf8', 0, 200),
            content_hash: require('crypto').createHash('md5').update(content).digest('hex')
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            file_path: filePath
        });
    }
});

module.exports = router;