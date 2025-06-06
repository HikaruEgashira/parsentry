/*!
 * Injection Attack Routes
 * 
 * Contains various injection vulnerability endpoints
 */

const express = require('express');
const { execSync } = require('child_process');
const vm = require('vm');
const xml2js = require('xml2js');
const yaml = require('yaml');
const handlebars = require('handlebars');
const ejs = require('ejs');

const router = express.Router();

// Vulnerable: Command injection
router.post('/command', (req, res) => {
    const { command, args } = req.body;
    
    if (!command) {
        return res.status(400).json({ error: 'Command required' });
    }
    
    try {
        // Vulnerable: Direct command execution
        const fullCommand = args ? `${command} ${args.join(' ')}` : command;
        const output = execSync(fullCommand, { encoding: 'utf8', timeout: 5000 });
        
        res.json({
            command: fullCommand,
            output: output,
            warning: 'Command executed without sanitization'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            command: command
        });
    }
});

// Vulnerable: XXE (XML External Entity) injection
router.post('/xml', (req, res) => {
    const xmlData = req.body;
    
    if (!xmlData) {
        return res.status(400).json({ error: 'XML data required' });
    }
    
    // Vulnerable: XXE attack vector
    const parser = new xml2js.Parser({
        explicitChildren: true,
        preserveChildrenOrder: true
    });
    
    parser.parseString(xmlData, (err, result) => {
        if (err) {
            return res.status(500).json({ 
                error: err.message,
                xml_data: xmlData.toString()
            });
        }
        
        res.json({ 
            parsed_xml: result,
            warning: 'XML parsed without XXE protection'
        });
    });
});

// Vulnerable: YAML deserialization
router.post('/yaml', (req, res) => {
    const yamlData = req.body.toString();
    
    if (!yamlData) {
        return res.status(400).json({ error: 'YAML data required' });
    }
    
    try {
        // Vulnerable: YAML deserialization
        const parsed = yaml.parse(yamlData);
        res.json({ 
            parsed_yaml: parsed,
            warning: 'YAML parsed without deserialization protection'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            yaml_data: yamlData
        });
    }
});

// Vulnerable: Server-Side Template Injection (Handlebars)
router.post('/template/handlebars', (req, res) => {
    const { template, context } = req.body;
    
    if (!template) {
        return res.status(400).json({ error: 'Template required' });
    }
    
    try {
        // Vulnerable: Template injection
        const compiledTemplate = handlebars.compile(template);
        const rendered = compiledTemplate(context || {});
        
        res.json({ 
            rendered: rendered,
            template: template,
            warning: 'Template rendered without SSTI protection'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            template: template
        });
    }
});

// Vulnerable: Server-Side Template Injection (EJS)
router.post('/template/ejs', (req, res) => {
    const { template, context } = req.body;
    
    if (!template) {
        return res.status(400).json({ error: 'Template required' });
    }
    
    try {
        // Vulnerable: Template injection
        const rendered = ejs.render(template, context || {});
        
        res.json({ 
            rendered: rendered,
            template: template,
            warning: 'EJS template rendered without SSTI protection'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            template: template
        });
    }
});

// Vulnerable: Code injection via VM
router.post('/eval', (req, res) => {
    const { code, timeout = 5000 } = req.body;
    
    if (!code) {
        return res.status(400).json({ error: 'Code required' });
    }
    
    try {
        // Vulnerable: VM sandbox that can be escaped
        const vmContext = {
            result: null,
            console: {
                log: (...args) => console.log('[VM]', ...args)
            },
            Buffer,
            // Vulnerable: Exposing require indirectly
            global: global
        };
        
        const script = new vm.Script(`
            try {
                result = (function() {
                    ${code}
                })();
            } catch (e) {
                result = { error: e.message };
            }
        `);
        
        script.runInNewContext(vmContext, { timeout });
        
        res.json({
            result: vmContext.result,
            code: code,
            warning: 'Code executed in vulnerable VM context'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            hint: 'Try escaping the sandbox with constructor.constructor'
        });
    }
});

// Vulnerable: JavaScript evaluation
router.post('/js-eval', (req, res) => {
    const { expression } = req.body;
    
    if (!expression) {
        return res.status(400).json({ error: 'Expression required' });
    }
    
    try {
        // Vulnerable: Direct eval usage
        const result = eval(expression);
        
        res.json({
            expression: expression,
            result: result,
            warning: 'Expression evaluated with eval() - extremely dangerous'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            expression: expression
        });
    }
});

// Vulnerable: Function constructor injection
router.post('/function', (req, res) => {
    const { functionBody, args } = req.body;
    
    if (!functionBody) {
        return res.status(400).json({ error: 'Function body required' });
    }
    
    try {
        // Vulnerable: Function constructor with user input
        const userFunction = new Function(...(args || []), functionBody);
        const result = userFunction();
        
        res.json({
            function_body: functionBody,
            arguments: args,
            result: result,
            warning: 'Function created with user input - code injection possible'
        });
    } catch (error) {
        res.status(500).json({ 
            error: error.message,
            function_body: functionBody
        });
    }
});

// Vulnerable: LDAP injection simulation
router.post('/ldap', (req, res) => {
    const { username, attribute, filter } = req.body;
    
    if (!username) {
        return res.status(400).json({ error: 'Username required' });
    }
    
    // Vulnerable: LDAP injection in multiple parameters
    let ldapQuery = `(&(objectClass=person)(uid=${username})`;
    
    if (attribute) {
        ldapQuery += `(${attribute}=*)`;
    }
    
    if (filter) {
        ldapQuery += filter;
    }
    
    ldapQuery += ')';
    
    // Simulate LDAP response
    const mockUsers = [
        { uid: 'admin', cn: 'Administrator', mail: 'admin@company.com' },
        { uid: 'user1', cn: 'Regular User', mail: 'user1@company.com' }
    ];
    
    res.json({
        query: ldapQuery,
        results: mockUsers,
        warning: 'LDAP query built without injection protection',
        hint: 'Try LDAP injection: *)(uid=*'
    });
});

module.exports = router;