You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-006
- **Kind**: endpoint
- **Identifier**: GET /memos
- **Description**: Memo listing endpoint rendering user-generated markdown content. Output encoding issues when displaying marked-parsed content in Swig templates.
- **Locations**: app/routes/index.js, app/routes/memos.js, app/data/memos-dao.js

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

### app/routes/index.js
```js
const SessionHandler = require("./session");
const ProfileHandler = require("./profile");
const BenefitsHandler = require("./benefits");
const ContributionsHandler = require("./contributions");
const AllocationsHandler = require("./allocations");
const MemosHandler = require("./memos");
const ResearchHandler = require("./research");
const tutorialRouter = require("./tutorial");
const ErrorHandler = require("./error").errorHandler;

const index = (app, db) => {

    "use strict";

    const sessionHandler = new SessionHandler(db);
    const profileHandler = new ProfileHandler(db);
    const benefitsHandler = new BenefitsHandler(db);
    const contributionsHandler = new ContributionsHandler(db);
    const allocationsHandler = new AllocationsHandler(db);
    const memosHandler = new MemosHandler(db);
    const researchHandler = new ResearchHandler(db);

    // Middleware to check if a user is logged in
    const isLoggedIn = sessionHandler.isLoggedInMiddleware;

    //Middleware to check if user has admin rights
    const isAdmin = sessionHandler.isAdminUserMiddleware;

    // The main page of the app
    app.get("/", sessionHandler.displayWelcomePage);

    // Login form
    app.get("/login", sessionHandler.displayLoginPage);
    app.post("/login", sessionHandler.handleLoginRequest);

    // Signup form
    app.get("/signup", sessionHandler.displaySignupPage);
    app.post("/signup", sessionHandler.handleSignup);

    // Logout page
    app.get("/logout", sessionHandler.displayLogoutPage);

    // The main page of the app
    app.get("/dashboard", isLoggedIn, sessionHandler.displayWelcomePage);

    // Profile page
    app.get("/profile", isLoggedIn, profileHandler.displayProfile);
    app.post("/profile", isLoggedIn, profileHandler.handleProfileUpdate);

    // Contributions Page
    app.get("/contributions", isLoggedIn, contributionsHandler.displayContributions);
    app.post("/contributions", isLoggedIn, contributionsHandler.handleContributionsUpdate);

    // Benefits Page
    app.get("/benefits", isLoggedIn, benefitsHandler.displayBenefits);
    app.post("/benefits", isLoggedIn, benefitsHandler.updateBenefits);
    /* Fix for A7 - checks user role to implement  Function Level Access Control
     app.get("/benefits", isLoggedIn, isAdmin, benefitsHandler.displayBenefits);
     app.post("/benefits", isLoggedIn, isAdmin, benefitsHandler.updateBenefits);
     */

    // Allocations Page
    app.get("/allocations/:userId", isLoggedIn, allocationsHandler.displayAllocations);

    // Memos Page
    app.get("/memos", isLoggedIn, memosHandler.displayMemos);
    app.post("/memos", isLoggedIn, memosHandler.addMemos);

    // Handle redirect for learning resources link
    app.get("/learn", isLoggedIn, (req, res) => {
        // Insecure way to handle redirects by taking redirect url from query string
        return res.redirect(req.query.url);
    });

    // Research Page
    app.get("/research", isLoggedIn, researchHandler.displayResearch);

    // Mount tutorial router
    app.use("/tutorial", tutorialRouter);

    // Error handling middleware
    app.use(ErrorHandler);
};

module.exports = index;

```

### app/routes/memos.js
```js
const MemosDAO = require("../data/memos-dao").MemosDAO;
const {
    environmentalScripts
} = require("../../config/config");

function MemosHandler(db) {
    "use strict";

    const memosDAO = new MemosDAO(db);

    this.addMemos = (req, res, next) => {

        memosDAO.insert(req.body.memo, (err, docs) => {
            if (err) return next(err);
            this.displayMemos(req, res, next);
        });
    };

    this.displayMemos = (req, res, next) => {

        const {
            userId
        } = req.session;

        memosDAO.getAllMemos((err, docs) => {
            if (err) return next(err);
            return res.render("memos", {
                memosList: docs,
                userId: userId,
                environmentalScripts
            });
        });
    };

}

module.exports = MemosHandler;

```

### app/data/memos-dao.js
```js
/* The MemosDAO must be constructed with a connected database object */
function MemosDAO(db) {

    "use strict";

    /* If this constructor is called without the "new" operator, "this" points
     * to the global object. Log a warning and call it correctly. */
    if (false === (this instanceof MemosDAO)) {
        console.log("Warning: MemosDAO constructor called without 'new' operator");
        return new MemosDAO(db);
    }

    const memosCol = db.collection("memos");

    this.insert = (memo, callback) => {

        // Create allocations document
        const memos = {
            memo,
            timestamp: new Date()
        };

        memosCol.insert(memos, (err, result) => !err ? callback(null, result) : callback(err, null));
    };

    this.getAllMemos = (callback) => {

        memosCol.find({}).sort({
            timestamp: -1
        }).toArray((err, memos) => {
            if (err) return callback(err, null);
            if (!memos) return callback("ERROR: No memos found", null);
            callback(null, memos);
        });
    };

}

module.exports = { MemosDAO };

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-006.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
