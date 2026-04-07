You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-017
- **Kind**: db_table
- **Identifier**: MongoDB contributions collection
- **Description**: Contributions collection with potential for NoSQL injection through unvalidated query parameters.
- **Locations**: app/data/contributions-dao.js, config/config.js

## Repository Context

## Directory Structure
```
CODE_OF_CONDUCT.md
CONTRIBUTING.md
Dockerfile
Gruntfile.js
LICENSE
Procfile
README.md
app/ 
  assets/ 
    images/ 
    js/ 
      chart/ 
      tour/ 
  data/ 
  routes/ 
  views/ 
    tutorial/ 
app.json
artifacts/ 
  cert/ 
  db-reset.js
config/ 
  config.js
  env/ 
cypress.json
docker-compose.yml
nodemon.json
package-lock.json
package.json
server.js
test/ 
  e2e/ 
    fixtures/ 
      users/ 
    integration/ 
    plugins/ 
    support/ 
  security/ 

```

## Languages
- JavaScript: 50 files
- Yaml: 4 files

## Dependencies
### package.json
```
{
  "name": "owasp-nodejs-goat",
  "private": true,
  "version": "1.3.0",
  "description": "A tool to learn OWASP Top 10 for node.js developers",
  "main": "server.js",
  "dependencies": {
    "bcrypt-nodejs": "0.0.3",
    "body-parser": "^1.15.1",
    "consolidate": "^0.14.1",
    "csurf": "^1.8.3",
    "dont-sniff-mimetype": "^1.0.0",
    "express": "^4.13.4",
    "express-session": "^1.13.0",
    "forever": "^2.0.0",
    "helmet": "^2.0.0",
    "marked": "0.3.5",
    "mongodb": "^2.1.18",
    "needle": "2.2.4",
    "node-esapi": "0.0.1",
    "serve-favicon": "^2.3.0",
    "swig": "^1.4.2",
    "underscore": "^1.8.3"
  },
  "comments": {
    "//": "a9 insecure components"
  },
  "scripts": {
    "start": "node server.js",
    "dev": "cross-env PORT=5000 nodemon",
    "test:e2e": "cross-env NODE_ENV=test cypress open",
    "test:ci": "cross-env NODE_ENV=test cypress run",
    "test": "node node_modules/grunt-cli/bin/grunt test",
    "db:seed": "cross-env NODE_ENV=test grunt db-reset",
    "precommit": "grunt precommit",
    "docker-mongo": "docker run -p 27017:27017 --name mongo  mongo:latest",
    "start-infra": "docker-compose up",
    "stop-infra": "docker-compose down",
    "cy:verify": "cypress verify"
  },
  "devDependencies": {
    "async": "^2.0.0-rc.4",
    "cross-env": "^7.0.2",
    "cypress": "^3.3.1",
    "grunt": "^1.0.3",
    "grunt-cli": "^1.2.0",
    "grunt-concurrent": "^2.3.0",
    "grunt-contrib-jshint": "^3.0.0",
    "grunt-contrib-watch": "^1.0.0",
    "grunt-env": "latest",
    "grunt-if": "https://github.com/binarymist/grunt-if/tarball/master",
    "grunt-jsbeautifier": "^0.2.12",
    "grunt-mocha-test": "^0.12.7",
    "grunt-npm-install": "^0.3.0",
    "grunt-retire": "^0.3.12",
    "jshint": "2.12.0",
    "mocha": "^2.4.5",
    "nodemon": "^1.19.1",
    "selenium-webdriver": "^2.53.2",
    "should": "^8.3.1",
    "zaproxy": "^0.2.0"
  },
  "repository": "https://github.com/OWASP/NodejsGoat",
  "license": "Apache 2.0"
}

```

## Entry Points
- app/routes/index.js
- test/e2e/plugins/index.js
- test/e2e/support/index.js
- server.js

Total source files: 54


## Source Code

### app/data/contributions-dao.js
```js
const UserDAO = require("./user-dao").UserDAO;

/* The ContributionsDAO must be constructed with a connected database object */
function ContributionsDAO(db) {
    "use strict";

    /* If this constructor is called without the "new" operator, "this" points
     * to the global object. Log a warning and call it correctly. */
    if (false === (this instanceof ContributionsDAO)) {
        console.log("Warning: ContributionsDAO constructor called without 'new' operator");
        return new ContributionsDAO(db);
    }

    const contributionsDB = db.collection("contributions");
    const userDAO = new UserDAO(db);

    this.update = (userId, preTax, afterTax, roth, callback) => {
        const parsedUserId = parseInt(userId);

        // Create contributions document
        const contributions = {
            userId: parsedUserId,
            preTax: preTax,
            afterTax: afterTax,
            roth: roth
        };

        contributionsDB.update({
            userId
            },
            contributions, {
                upsert: true
            },
            err => {
                if (!err) {
                    console.log("Updated contributions");
                    // add user details
                    userDAO.getUserById(parsedUserId, (err, user) => {

                        if (err) return callback(err, null);

                        contributions.userName = user.userName;
                        contributions.firstName = user.firstName;
                        contributions.lastName = user.lastName;
                        contributions.userId = userId;

                        return callback(null, contributions);
                    });
                } else {
                    return callback(err, null);
                }
            }
        );
    };

    this.getByUserId = (userId, callback) => {
        contributionsDB.findOne({
                userId: userId
            },
            (err, contributions) => {
                if (err) return callback(err, null);

                // Set defualt contributions if not set
                contributions = contributions || {
                    preTax: 2,
                    afterTax: 2,
                    roth: 2
                };

                // add user details
                userDAO.getUserById(userId, (err, user) => {

                    if (err) return callback(err, null);
                    contributions.userName = user.userName;
                    contributions.firstName = user.firstName;
                    contributions.lastName = user.lastName;
                    contributions.userId = userId;

                    callback(null, contributions);
                });
            }
        );
    };
}

module.exports = { ContributionsDAO };

```

### config/config.js
```js
const _ = require("underscore");
const path = require("path");
const util = require("util");

const finalEnv = process.env.NODE_ENV || "development";

const allConf = require(path.resolve(__dirname + "/../config/env/all.js"));
const envConf = require(path.resolve(__dirname + "/../config/env/" + finalEnv.toLowerCase() + ".js")) || {};

const config = { ...allConf, ...envConf };

console.log(`Current Config:`);
console.log(util.inspect(config, false, null));

module.exports = config;

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0
