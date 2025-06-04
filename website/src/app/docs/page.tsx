export default function DocsPage() {
  return (
    <div className="min-h-screen bg-white">
      <div className="container mx-auto px-4 py-16 max-w-4xl">
        <h1 className="text-4xl font-bold mb-8">Vulnhuntrs Documentation</h1>
        
        <div className="prose prose-lg max-w-none">
          <p className="text-xl text-gray-600 mb-8">
            Vulnhuntrs is an AI-powered security vulnerability scanner that combines static code analysis with LLMs to detect remotely exploitable vulnerabilities.
          </p>

          <h2>Features</h2>
          <ul>
            <li><strong>Multi-language support</strong>: Supports Rust, Python, JavaScript, TypeScript, Go, Java, and Ruby</li>
            <li><strong>AI-powered analysis</strong>: Uses large language models to identify complex security vulnerabilities</li>
            <li><strong>Detailed reports</strong>: Generates comprehensive vulnerability reports with proof-of-concept code</li>
            <li><strong>Static code analysis</strong>: Combines pattern matching with semantic analysis using tree-sitter</li>
          </ul>

          <h2>Quick Start</h2>
          
          <h3>Installation</h3>
          <pre className="bg-gray-900 text-green-400 p-4 rounded-lg overflow-x-auto">
            <code>{`# Using Docker (recommended)
docker pull ghcr.io/hikaruegashira/vulnhuntrs:latest

# Or build from source
git clone https://github.com/HikaruEgashira/vulnhuntrs
cd vulnhuntrs
cargo build --release`}</code>
          </pre>

          <h3>Basic Usage</h3>
          <pre className="bg-gray-900 text-green-400 p-4 rounded-lg overflow-x-auto">
            <code>{`# Analyze a local directory
vulnhuntrs -r /path/to/project

# Analyze a GitHub repository
vulnhuntrs --repo owner/repository

# Generate summary report
vulnhuntrs -r /path/to/project --summary`}</code>
          </pre>

          <h2>Docker Usage</h2>
          <pre className="bg-gray-900 text-green-400 p-4 rounded-lg overflow-x-auto">
            <code>{`docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \\
  -v $(pwd)/reports:/reports \\
  --user $(id -u):$(id -g) \\
  ghcr.io/hikaruegashira/vulnhuntrs:latest \\
  --repo PentesterLab/cr-go --output-dir /reports --summary`}</code>
          </pre>

          <h2>Configuration Options</h2>
          <ul>
            <li><code>-r, --root &lt;ROOT&gt;</code>: Specify the root directory of the project to scan</li>
            <li><code>--repo &lt;REPO&gt;</code>: Specify GitHub repository URL for analysis</li>
            <li><code>-a, --analyze &lt;ANALYZE&gt;</code>: Specify specific file or directory for analysis</li>
            <li><code>-v</code>: Display verbose logs (use multiple times for more detail)</li>
            <li><code>--min-confidence &lt;MIN_CONFIDENCE&gt;</code>: Specify minimum confidence level for vulnerabilities (default: 0)</li>
            <li><code>--vuln-types &lt;TYPES&gt;</code>: Filter by specific vulnerability types (comma-separated)</li>
            <li><code>--summary</code>: Display summary report</li>
          </ul>

          <h2>Example Output</h2>
          <pre className="bg-gray-50 border p-4 rounded-lg overflow-x-auto text-sm">
            <code>{`🔍 Vulnhuntrs - Security Analysis Tool
📁 Found source files (1)
  [1] example/python-vulnerable-app/app.py
🔎 Found security pattern matches (1)
  [P1] example/python-vulnerable-app/app.py
📄 Analyzing: example/python-vulnerable-app/app.py (1 / 1)

📝 Analysis Report
================================================================================

🔍 Analysis Results:
This application contains 3 major vulnerabilities. First, the /sqli endpoint 
directly embeds user-provided 'username' parameter into SQL queries without 
sanitization, enabling SQL injection attacks...

🔨 PoC (Proof of Concept):
【SQLインジェクション】
URL: /sqli?username=' OR '1'='1

【XSS】
URL: /xss?name=<script>alert(1)</script>

【コマンドインジェクション(RCE)】
URL: /cmdi?hostname=localhost;whoami`}</code>
          </pre>

          <h2>Architecture</h2>
          <p>The codebase follows a pipeline architecture:</p>
          <ol>
            <li><strong>File Discovery</strong>: Identifies source files to analyze</li>
            <li><strong>Pattern Matching</strong>: Filters files using regex patterns</li>
            <li><strong>Code Parsing</strong>: Uses tree-sitter to parse code and extract semantic information</li>
            <li><strong>Context Building</strong>: Collects function definitions and references for context</li>
            <li><strong>LLM Analysis</strong>: Sends code + context to LLM for vulnerability detection</li>
            <li><strong>Response Handling</strong>: Formats and validates LLM responses</li>
          </ol>

          <h2>Security</h2>
          <p>This tool is intended for security research and educational purposes only. Do not use the example vulnerable applications in production environments.</p>

          <h2>License</h2>
          <p>AGPL 3.0</p>

          <h2>Links</h2>
          <ul>
            <li><a href="https://github.com/HikaruEgashira/vulnhuntrs" className="text-blue-600 hover:underline">GitHub Repository</a></li>
            <li><a href="https://hub.docker.com/r/hikaruegashira/vulnhuntrs" className="text-blue-600 hover:underline">Docker Hub</a></li>
          </ul>
        </div>
      </div>
    </div>
  );
}