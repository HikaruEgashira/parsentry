You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-007
- **Kind**: endpoint
- **Identifier**: POST /memos
- **Description**: Memo creation endpoint that renders user-supplied markdown via marked 0.3.5 (known XSS vulnerabilities). Stored XSS attack vector.
- **Locations**: app/routes/index.js, app/data/memos-dao.js

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


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-007.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
