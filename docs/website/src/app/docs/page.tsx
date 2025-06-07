export default function DocsPage() {
  return (
    <div className="min-h-screen bg-white">
      <div className="container mx-auto px-4 py-16 max-w-4xl">
        <h1 className="text-4xl font-bold mb-8">Parsentry Documentation</h1>
        
        <div className="prose prose-lg max-w-none">
          <p className="text-xl text-gray-600 mb-8">
            Parsentry is an AI-powered security vulnerability scanner that combines static code analysis with LLMs to detect remotely exploitable vulnerabilities.
          </p>

          <h2>Features</h2>
          <ul>
            <li><strong>Multi-language support</strong>: Supports Rust, Python, JavaScript, TypeScript, Go, Java, Ruby, C/C++, and Terraform</li>
            <li><strong>PAR Classification</strong>: Uses Principal-Action-Resource framework to categorize security patterns</li>
            <li><strong>MITRE ATT&CK Integration</strong>: Maps vulnerabilities to MITRE ATT&CK tactics and techniques</li>
            <li><strong>AI-powered analysis</strong>: Uses large language models to identify complex security vulnerabilities with context-aware analysis</li>
            <li><strong>Detailed reports</strong>: Generates comprehensive vulnerability reports with confidence scoring and proof-of-concept code</li>
            <li><strong>Tree-sitter parsing</strong>: Combines pattern matching with semantic analysis using tree-sitter for accurate code understanding</li>
          </ul>

          <h2>Quick Start</h2>
          
          <h3>Installation</h3>
          <pre className="bg-gray-900 text-green-400 p-4 rounded-lg overflow-x-auto">
            <code>{`# Using Docker (recommended)
docker pull ghcr.io/hikaruegashira/parsentry:latest

# Or build from source
git clone https://github.com/HikaruEgashira/parsentry
cd parsentry
cargo build --release`}</code>
          </pre>

          <h3>Basic Usage</h3>
          <pre className="bg-gray-900 text-green-400 p-4 rounded-lg overflow-x-auto">
            <code>{`# Analyze a local directory
cargo run -- -r /path/to/project

# Analyze a GitHub repository
cargo run -- --repo owner/repository

# Generate summary report with markdown output
cargo run -- -r /path/to/project --output-dir ./reports --summary

# Specify LLM model
cargo run -- -r /path/to/project --model gpt-4

# Set minimum confidence threshold
cargo run -- -r /path/to/project --min-confidence 70`}</code>
          </pre>

          <h2>Supported Vulnerability Types</h2>
          <p>Parsentry can detect the following vulnerability types:</p>
          <ul>
            <li><strong>LFI</strong> - Local File Inclusion</li>
            <li><strong>RCE</strong> - Remote Code Execution</li>
            <li><strong>SSRF</strong> - Server-Side Request Forgery</li>
            <li><strong>AFO</strong> - Arbitrary File Operation</li>
            <li><strong>SQLI</strong> - SQL Injection</li>
            <li><strong>XSS</strong> - Cross-Site Scripting</li>
            <li><strong>IDOR</strong> - Insecure Direct Object Reference</li>
          </ul>

          <h2>Environment Variables</h2>
          <p>Configure Parsentry using these environment variables:</p>
          <ul>
            <li><code>OPENAI_API_KEY</code> - OpenAI API key for GPT models</li>
            <li><code>ANTHROPIC_API_KEY</code> - Anthropic API key for Claude models</li>
            <li><code>GOOGLE_API_KEY</code> - Google API key for Gemini models</li>
            <li><code>GROQ_API_KEY</code> - Groq API key for fast inference models</li>
          </ul>

          <h2>Architecture</h2>
          <p>Parsentry follows a pipeline architecture:</p>
          <ol>
            <li><strong>File Discovery</strong> - Identifies source files to analyze</li>
            <li><strong>Pattern Matching</strong> - Filters files using PAR classification</li>
            <li><strong>Code Parsing</strong> - Uses tree-sitter for semantic analysis</li>
            <li><strong>Context Building</strong> - Collects function definitions and references</li>
            <li><strong>LLM Analysis</strong> - Sends code + context to LLM for vulnerability detection</li>
            <li><strong>Response Handling</strong> - Formats and validates LLM responses</li>
          </ol>

          <h2>Links</h2>
          <ul>
            <li><a href="https://github.com/HikaruEgashira/parsentry" className="text-blue-600 hover:underline">GitHub Repository</a></li>
            <li><a href="https://hub.docker.com/r/hikaruegashira/parsentry" className="text-blue-600 hover:underline">Docker Hub</a></li>
          </ul>
        </div>
      </div>
    </div>
  );
}