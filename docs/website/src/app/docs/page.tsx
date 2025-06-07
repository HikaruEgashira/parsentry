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
            <li><strong>Multi-language support</strong>: Supports Rust, Python, JavaScript, TypeScript, Go, Java, and Ruby</li>
            <li><strong>AI-powered analysis</strong>: Uses large language models to identify complex security vulnerabilities</li>
            <li><strong>Detailed reports</strong>: Generates comprehensive vulnerability reports with proof-of-concept code</li>
            <li><strong>Static code analysis</strong>: Combines pattern matching with semantic analysis using tree-sitter</li>
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
parsentry -r /path/to/project

# Analyze a GitHub repository
parsentry --repo owner/repository

# Generate summary report
parsentry -r /path/to/project --summary`}</code>
          </pre>

          <h2>Raw Documentation for LLMs</h2>
          <p>Complete documentation is available in plain markdown format for LLM consumption:</p>
          <ul>
            <li><a href="/docs/raw/index.md" className="text-blue-600 hover:underline">Documentation Index</a></li>
            <li><a href="/docs/raw/getting-started.md" className="text-blue-600 hover:underline">Getting Started</a></li>
            <li><a href="/docs/raw/installation.md" className="text-blue-600 hover:underline">Installation Guide</a></li>
            <li><a href="/docs/raw/configuration.md" className="text-blue-600 hover:underline">Configuration</a></li>
            <li><a href="/docs/raw/usage.md" className="text-blue-600 hover:underline">Usage Examples</a></li>
            <li><a href="/docs/raw/examples.md" className="text-blue-600 hover:underline">Real-world Examples</a></li>
            <li><a href="/docs/raw/architecture.md" className="text-blue-600 hover:underline">Architecture</a></li>
            <li><a href="/docs/raw/api.md" className="text-blue-600 hover:underline">API Reference</a></li>
            <li><a href="/docs/raw/contributing.md" className="text-blue-600 hover:underline">Contributing Guide</a></li>
          </ul>

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