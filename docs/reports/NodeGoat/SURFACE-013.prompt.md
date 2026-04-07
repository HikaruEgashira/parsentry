You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-013
- **Kind**: db_table
- **Identifier**: MongoDB users collection
- **Description**: Users collection storing credentials. bcrypt-nodejs 0.0.3 is deprecated with known vulnerabilities. Password hashing may be weak. db-reset.js may contain seed data with hardcoded credentials.
- **Locations**: app/data/user-dao.js, artifacts/db-reset.js

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

### app/data/user-dao.js
```js
const bcrypt = require("bcrypt-nodejs");

/* The UserDAO must be constructed with a connected database object */
function UserDAO(db) {

    "use strict";

    /* If this constructor is called without the "new" operator, "this" points
     * to the global object. Log a warning and call it correctly. */
    if (false === (this instanceof UserDAO)) {
        console.log("Warning: UserDAO constructor called without 'new' operator");
        return new UserDAO(db);
    }

    const usersCol = db.collection("users");

    this.addUser = (userName, firstName, lastName, password, email, callback) => {

        // Create user document
        const user = {
            userName,
            firstName,
            lastName,
            benefitStartDate: this.getRandomFutureDate(),
            password //received from request param
            /*
            // Fix for A2-1 - Broken Auth
            // Stores password  in a safer way using one way encryption and salt hashing
            password: bcrypt.hashSync(password, bcrypt.genSaltSync())
            */
        };

        // Add email if set
        if (email) {
            user.email = email;
        }

        this.getNextSequence("userId", (err, id) => {
            if (err) {
                return callback(err, null);
            }
            console.log(typeof(id));

            user._id = id;
            usersCol.insert(user, (err, result) => !err ? callback(null, result.ops[0]) : callback(err, null));
        });
    };

    this.getRandomFutureDate = () => {
        const today = new Date();
        const day = (Math.floor(Math.random() * 10) + today.getDay()) % 29;
        const month = (Math.floor(Math.random() * 10) + today.getMonth()) % 12;
        const year = Math.ceil(Math.random() * 30) + today.getFullYear();
        return `${year}-${("0" + month).slice(-2)}-${("0" + day).slice(-2)}`;
    };

    this.validateLogin = (userName, password, callback) => {

        // Helper function to compare passwords
        const comparePassword = (fromDB, fromUser) => {
            return fromDB === fromUser;
            /*
            // Fix for A2-Broken Auth
            // compares decrypted password stored in this.addUser()
            return bcrypt.compareSync(fromDB, fromUser);
            */
        };

        // Callback to pass to MongoDB that validates a user document
        const validateUserDoc = (err, user) => {

            if (err) return callback(err, null);

            if (user) {
                if (comparePassword(password, user.password)) {
                    callback(null, user);
                } else {
                    const invalidPasswordError = new Error("Invalid password");
                    // Set an extra field so we can distinguish this from a db error
                    invalidPasswordError.invalidPassword = true;
                    callback(invalidPasswordError, null);
                }
            } else {
                const noSuchUserError = new Error("User: " + user + " does not exist");
                // Set an extra field so we can distinguish this from a db error
                noSuchUserError.noSuchUser = true;
                callback(noSuchUserError, null);
            }
        };

        usersCol.findOne({
            userName: userName
        }, validateUserDoc);
    };

    // This is the good one, see the next function
    this.getUserById = (userId, callback) => {
        usersCol.findOne({
            _id: parseInt(userId)
        }, callback);
    };

    this.getUserByUserName = (userName, callback) => {
        usersCol.findOne({
            userName: userName
        }, callback);
    };

    this.getNextSequence = (name, callback) => {
        db.collection("counters").findAndModify({
                _id: name
            }, [], {
                $inc: {
                    seq: 1
                }
            }, {
                new: true
            },
            (err, data) =>  err ? callback(err, null) : callback(null, data.value.seq));
    };
}

module.exports = { UserDAO };

```

### artifacts/db-reset.js
```js
#!/usr/bin/env nodejs

"use strict";

// This script initializes the database. You can set the environment variable
// before running it (default: development). ie:
// NODE_ENV=production node artifacts/db-reset.js

const { MongoClient } = require("mongodb");
const { db } = require("../config/config");

const USERS_TO_INSERT = [
    {
        "_id": 1,
        "userName": "admin",
        "firstName": "Node Goat",
        "lastName": "Admin",
        "password": "Admin_123",
        //"password" : "$2a$10$8Zo/1e8KM8QzqOKqbDlYlONBOzukWXrM.IiyzqHRYDXqwB3gzDsba", // Admin_123
        "isAdmin": true
    }, {
        "_id": 2,
        "userName": "user1",
        "firstName": "John",
        "lastName": "Doe",
        "benefitStartDate": "2030-01-10",
        "password": "User1_123"
        // "password" : "$2a$10$RNFhiNmt2TTpVO9cqZElb.LQM9e1mzDoggEHufLjAnAKImc6FNE86",// User1_123
    }, {
        "_id": 3,
        "userName": "user2",
        "firstName": "Will",
        "lastName": "Smith",
        "benefitStartDate": "2025-11-30",
        "password": "User2_123"
        //"password" : "$2a$10$Tlx2cNv15M0Aia7wyItjsepeA8Y6PyBYaNdQqvpxkIUlcONf1ZHyq", // User2_123
    }];

const tryDropCollection = (db, name) => {
    return new Promise((resolve, reject) => {
        db.dropCollection(name, (err, data) => {
            if (!err) {
                console.log(`Dropped collection: ${name}`);
            }
            resolve(undefined);
        });
    });
};

const parseResponse = (err, res, comm) => {
    if (err) {
        console.log("ERROR:");
        console.log(comm);
        console.log(JSON.stringify(err));
        process.exit(1);
    }
    console.log(comm);
    console.log(JSON.stringify(res));
};


// Starting here
MongoClient.connect(db, (err, db) =>  {
    if (err) {
        console.log("ERROR: connect");
        console.log(JSON.stringify(err));
        process.exit(1);
    }
    console.log("Connected to the database");

    const collectionNames = [
        "users",
        "allocations",
        "contributions",
        "memos",
        "counters"
    ];

    // remove existing data (if any), we don't want to look for errors here
    console.log("Dropping existing collections");
    const dropPromises = collectionNames.map((name) => tryDropCollection(db, name));

    // Wait for all drops to finish (or fail) before continuing
    Promise.all(dropPromises).then(() => {
        const usersCol = db.collection("users");
        const allocationsCol = db.collection("allocations");
        const countersCol = db.collection("counters");

        // reset unique id counter
        countersCol.insert({
            _id: "userId",
            seq: 3
        }, (err, data) => {
            parseResponse(err, data, "countersCol.insert");
        });

        // insert admin and test users
        console.log("Users to insert:");
        USERS_TO_INSERT.forEach((user) => console.log(JSON.stringify(user)));

        usersCol.insertMany(USERS_TO_INSERT, (err, data) => {
            const finalAllocations = [];

            // We can't continue if error here
            if (err) {
                console.log("ERROR: insertMany");
                console.log(JSON.stringify(err));
                process.exit(1);
            }
            parseResponse(err, data, "users.insertMany");

            data.ops.forEach((user) => {
                const stocks = Math.floor((Math.random() * 40) + 1);
                const funds = Math.floor((Math.random() * 40) + 1);

                finalAllocations.push({
                    userId: user._id,
                    stocks: stocks,
                    funds: funds,
                    bonds: 100 - (stocks + funds)
                });
            });

            console.log("Allocations to insert:");
            finalAllocations.forEach(allocation => console.log(JSON.stringify(allocation)));

            allocationsCol.insertMany(finalAllocations, (err, data) => {
                parseResponse(err, data, "allocations.insertMany");
                console.log("Database reset performed successfully");
                process.exit(0);
            });

        });
    });
});

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-013.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
