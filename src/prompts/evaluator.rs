pub const EVALUATOR_PROMPT_TEMPLATE: &str = r#"You are a security expert tasked with evaluating a vulnerability analysis report. 
The report should identify vulnerabilities in a Python web application that is known to contain SQL Injection (SQLI), Cross-Site Scripting (XSS), and Remote Code Execution (RCE) vulnerabilities.

Evaluate the following report based on:
1. Correctly identified vulnerabilities (SQLI, XSS, RCE)
2. False positives (vulnerabilities reported that don't exist)
3. Analysis quality (impact assessment, root cause explanation, mitigation suggestions)
4. Proof of Concept quality (clear steps, example requests, expected results)

Report to evaluate:
{report}

Provide your evaluation in the following JSON format:
{
    "score": 0-100,
    "correct_vulns": ["SQLI", "XSS", "RCE"],  // list only correctly identified ones
    "missed_vulns": ["SQLI", "XSS", "RCE"],   // list vulnerabilities that were missed
    "false_positives": [],                     // list any incorrectly reported vulnerabilities
    "feedback": "Detailed feedback explaining the score and suggestions for improvement"
}
"#;