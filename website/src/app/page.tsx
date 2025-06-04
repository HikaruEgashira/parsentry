import Link from "next/link";
import Image from "next/image";

export default function Home() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-gray-50 to-white dark:from-gray-900 dark:to-gray-800">
      <div className="container mx-auto px-4 py-16">
        <div className="text-center">
          <div className="mb-8">
            <Image
              src="/logo.png"
              alt="Vulnhuntrs Logo"
              width={200}
              height={200}
              className="mx-auto"
              priority
            />
          </div>
          <h1 className="text-5xl font-bold text-gray-900 dark:text-white mb-6">
            Vulnhuntrs
          </h1>
          <p className="text-xl text-gray-600 dark:text-gray-300 mb-8 max-w-2xl mx-auto">
            AI-powered security vulnerability scanner that combines static code analysis with LLMs to detect remotely exploitable vulnerabilities.
          </p>
          
          <div className="flex gap-4 justify-center mb-12">
            <Link
              href="/docs"
              className="bg-blue-600 hover:bg-blue-700 text-white px-8 py-3 rounded-lg font-medium transition-colors"
            >
              Get Started
            </Link>
            <a
              href="https://github.com/HikaruEgashira/vulnhuntrs"
              target="_blank"
              rel="noopener noreferrer"
              className="border border-gray-300 hover:border-gray-400 px-8 py-3 rounded-lg font-medium transition-colors"
            >
              View on GitHub
            </a>
          </div>

          <div className="grid md:grid-cols-3 gap-8 max-w-4xl mx-auto">
            <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-sm border">
              <h3 className="text-lg font-semibold mb-3">🔍 Multi-Language Support</h3>
              <p className="text-gray-600 dark:text-gray-300">
                Supports Rust, Python, JavaScript, TypeScript, Go, Java, and Ruby with tree-sitter parsing.
              </p>
            </div>
            
            <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-sm border">
              <h3 className="text-lg font-semibold mb-3">🤖 AI-Powered Analysis</h3>
              <p className="text-gray-600 dark:text-gray-300">
                Uses large language models to identify complex security vulnerabilities beyond pattern matching.
              </p>
            </div>
            
            <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-sm border">
              <h3 className="text-lg font-semibold mb-3">📊 Detailed Reports</h3>
              <p className="text-gray-600 dark:text-gray-300">
                Generates comprehensive vulnerability reports with proof-of-concept code and remediation guidance.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
