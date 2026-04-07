You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-013
- **Kind**: db_table
- **Identifier**: memos collection (MongoDB)
- **Description**: Stores user-created memos rendered as HTML via vulnerable marked library. Stored XSS payload persistence.
- **Locations**: app/data/memos-dao.js

## Source Code

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


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-013.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
