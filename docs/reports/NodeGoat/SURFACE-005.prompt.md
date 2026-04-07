You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-005
- **Kind**: endpoint
- **Identifier**: POST /contributions
- **Description**: Financial contributions endpoint writing to MongoDB. Warrants review for authorization bypass and input validation on monetary values.
- **Locations**: app/routes/index.js, app/data/contributions-dao.js

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

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-005.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
