/*!
 * System Integration Routes
 * 
 * Handles system command execution and data processing
 */

const express = require('express');
const { execSync } = require('child_process');
const vm = require('vm');
const xml2js = require('xml2js');
const yaml = require('yaml');
const handlebars = require('handlebars');
const ejs = require('ejs');

const router = express.Router();

// System command execution endpoint
router.post('/system-exec', (req, res) => {
    const { operation, parameters } = req.body;
    
    if (!operation) {
        return res.status(400).json({ error: 'Operation required' });
    }
    
    try {
        const fullCommand = parameters ? `${operation} ${parameters.join(' ')}` : operation;
        const output = execSync(fullCommand, { encoding: 'utf8', timeout: 5000 });
        
        res.json({
            operation: fullCommand,
            output: output,
            status: 'executed'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            operation: operation
        });
    }
});

// XML document processing
router.post('/parse-xml', (req, res) => {
    const documentData = req.body;
    
    if (!documentData) {
        return res.status(400).json({ error: 'Document data required' });
    }
    
    const parser = new xml2js.Parser({
        explicitChildren: true,
        preserveChildrenOrder: true
    });
    
    parser.parseString(documentData, (err, result) => {
        if (err) {
            return res.status(500).json({ 
                error: err.message,
                document_data: documentData.toString()
            });
        }
        
        res.json({ 
            parsed_document: result,
            status: 'processing_complete'
        });
    });
});

// Configuration file processing
router.post('/config-loader', (req, res) => {
    const configData = req.body.toString();
    
    if (!configData) {
        return res.status(400).json({ error: 'Configuration data required' });
    }
    
    try {
        const parsed = yaml.parse(configData);
        res.json({ 
            configuration: parsed,
            status: 'loaded'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            config_data: configData
        });
    }
});

// Dynamic content generation
router.post('/render/handlebars', (req, res) => {
    const { content, data } = req.body;
    
    if (!content) {
        return res.status(400).json({ error: 'Content template required' });
    }
    
    try {
        const compiledTemplate = handlebars.compile(content);
        const rendered = compiledTemplate(data || {});
        
        res.json({ 
            output: rendered,
            template: content,
            status: 'rendered'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            template: content
        });
    }
});

// Email template processing
router.post('/render/email', (req, res) => {
    const { template, variables } = req.body;
    
    if (!template) {
        return res.status(400).json({ error: 'Email template required' });
    }
    
    try {
        const rendered = ejs.render(template, variables || {});
        
        res.json({ 
            email_content: rendered,
            template: template,
            status: 'processed'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            template: template
        });
    }
});

// Script execution engine
router.post('/execute-script', (req, res) => {
    const { script, timeout = 5000 } = req.body;
    
    if (!script) {
        return res.status(400).json({ error: 'Script required' });
    }
    
    try {
        const vmContext = {
            result: null,
            console: {
                log: (...args) => console.log('[Engine]', ...args)
            },
            Buffer,
            global: global
        };
        
        const vmScript = new vm.Script(`
            try {
                result = (function() {
                    ${script}
                })();
            } catch (e) {
                result = { error: e.message };
            }
        `);
        
        vmScript.runInNewContext(vmContext, { timeout });
        
        res.json({
            execution_result: vmContext.result,
            script: script,
            status: 'completed'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            help: 'Check script syntax'
        });
    }
});

// Mathematical expression calculator
router.post('/calculate', (req, res) => {
    const { formula } = req.body;
    
    if (!formula) {
        return res.status(400).json({ error: 'Mathematical formula required' });
    }
    
    try {
        const result = eval(formula);
        
        res.json({
            formula: formula,
            calculation: result,
            status: 'computed'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            formula: formula
        });
    }
});

// Dynamic function creator
router.post('/create-function', (req, res) => {
    const { implementation, parameters } = req.body;
    
    if (!implementation) {
        return res.status(400).json({ error: 'Function implementation required' });
    }
    
    try {
        const dynamicFunction = new Function(...(parameters || []), implementation);
        const result = dynamicFunction();
        
        res.json({
            implementation: implementation,
            parameters: parameters,
            execution_result: result,
            status: 'created_and_executed'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            implementation: implementation
        });
    }
});

// Directory service query
router.post('/directory-search', (req, res) => {
    const { user_id, search_attribute, additional_filter } = req.body;
    
    if (!user_id) {
        return res.status(400).json({ error: 'User ID required' });
    }
    
    let searchQuery = `(&(objectClass=person)(uid=${user_id})`;
    
    if (search_attribute) {
        searchQuery += `(${search_attribute}=*)`;
    }
    
    if (additional_filter) {
        searchQuery += additional_filter;
    }
    
    searchQuery += ')';
    
    const directoryResults = [
        { uid: 'admin', cn: 'Administrator', mail: 'admin@company.com' },
        { uid: 'user1', cn: 'Regular User', mail: 'user1@company.com' }
    ];
    
    res.json({
        search_query: searchQuery,
        directory_entries: directoryResults,
        status: 'search_completed'
    });
});

module.exports = router;