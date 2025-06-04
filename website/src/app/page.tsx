'use client';

import Image from "next/image";
import { useState } from "react";

export default function Home() {
  const [copied, setCopied] = useState(false);

  const scrollToQuickStart = () => {
    document.getElementById('quick-start')?.scrollIntoView({ 
      behavior: 'smooth',
      block: 'start'
    });
  };

  const copyToClipboard = async () => {
    const command = 'docker run -e OPENAI_API_KEY=$OPENAI_API_KEY ghcr.io/hikaruegashira/vulnhuntrs:latest --repo your-org/your-repo';
    try {
      await navigator.clipboard.writeText(command);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy: ', err);
    }
  };

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="relative min-h-screen flex items-center justify-center overflow-hidden bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
        {/* Animated background */}
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute -inset-10 opacity-50">
            <div className="absolute top-0 left-1/4 w-72 h-72 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl animate-blob"></div>
            <div className="absolute top-0 right-1/4 w-72 h-72 bg-yellow-500 rounded-full mix-blend-multiply filter blur-xl animate-blob animation-delay-2000"></div>
            <div className="absolute -bottom-8 left-1/3 w-72 h-72 bg-pink-500 rounded-full mix-blend-multiply filter blur-xl animate-blob animation-delay-4000"></div>
          </div>
        </div>
        
        {/* Grid pattern overlay */}
        <div className="absolute inset-0 bg-[url('data:image/svg+xml,%3Csvg width="60" height="60" viewBox="0 0 60 60" xmlns="http://www.w3.org/2000/svg"%3E%3Cg fill="none" fill-rule="evenodd"%3E%3Cg fill="%23ffffff" fill-opacity="0.03"%3E%3Ccircle cx="30" cy="30" r="1"/%3E%3C/g%3E%3C/g%3E%3C/svg%3E')]"></div>
        
        <div className="relative z-10 text-center px-6 max-w-6xl mx-auto">
          {/* Logo */}
          <div className="mb-12 flex justify-center">
            <div className="relative">
              <Image
                src="/vulnhuntrs/logo.png"
                alt="Vulnhuntrs Logo"
                width={140}
                height={140}
                priority
                className="drop-shadow-2xl"
              />
              <div className="absolute inset-0 bg-gradient-to-tr from-purple-500/30 to-pink-500/30 rounded-full blur-xl"></div>
            </div>
          </div>

          {/* Badge */}
          <div className="mb-8 flex justify-center">
            <div className="relative group">
              <div className="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-full blur opacity-75 group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-tilt"></div>
              <div className="relative px-6 py-2 bg-black rounded-full leading-none flex items-center">
                <span className="text-purple-400 font-semibold">⚡ AI-Powered</span>
                <span className="ml-2 text-gray-300">security analysis</span>
              </div>
            </div>
          </div>

          {/* Main heading */}
          <h1 className="text-5xl md:text-7xl lg:text-8xl font-bold text-white mb-8 leading-tight">
            <span className="block">Vulnhuntrs</span>
            <span className="block bg-gradient-to-r from-purple-400 via-pink-400 to-red-400 bg-clip-text text-transparent animate-gradient-x">
              Security Scanner
            </span>
          </h1>

          {/* Description */}
          <p className="text-xl md:text-2xl text-gray-300 max-w-4xl mx-auto mb-12 leading-relaxed">
            Unleash the power of AI to detect remotely exploitable vulnerabilities across 
            <span className="text-purple-400 font-semibold"> 7+ programming languages</span>. 
            Static analysis meets large language models for unparalleled security insights.
          </p>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row items-center justify-center gap-6 mb-20">
            <button
              onClick={scrollToQuickStart}
              className="group relative inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-white transition-all duration-200 bg-gradient-to-r from-purple-600 to-pink-600 rounded-xl hover:scale-105 hover:shadow-xl hover:shadow-purple-500/25"
            >
              <span className="relative z-10">Get Started</span>
              <div className="absolute inset-0 bg-gradient-to-r from-purple-600 to-pink-600 rounded-xl blur opacity-75 group-hover:opacity-100 transition duration-200"></div>
            </button>
            
            <a
              href="https://github.com/HikaruEgashira/vulnhuntrs"
              target="_blank"
              rel="noopener noreferrer"
              className="group flex items-center gap-3 px-8 py-4 text-lg font-semibold text-white bg-white/10 backdrop-blur-sm rounded-xl border border-white/20 hover:bg-white/20 hover:scale-105 transition-all duration-200"
            >
              <svg className="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              View on GitHub
            </a>
          </div>

          {/* Scroll indicator */}
          <div className="animate-bounce">
            <svg className="w-6 h-6 text-white/60 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 14l-7 7m0 0l-7-7m7 7V3" />
            </svg>
          </div>
        </div>
      </section>

      {/* Quick Start Section */}
      <section id="quick-start" className="py-24 bg-gradient-to-b from-slate-900 to-gray-900">
        <div className="max-w-4xl mx-auto px-6 text-center">
          <div className="mb-16">
            <h2 className="text-4xl md:text-5xl font-bold text-white mb-6">
              Quick Start
            </h2>
            <p className="text-xl text-gray-300 max-w-2xl mx-auto">
              Get started with Vulnhuntrs in seconds. Just run this single Docker command:
            </p>
          </div>

          {/* Docker Command */}
          <div className="relative group max-w-4xl mx-auto">
            <div className="absolute -inset-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-lg blur opacity-75 group-hover:opacity-100 transition duration-1000"></div>
            <div className="relative bg-gray-900 rounded-lg p-6 border border-gray-700">
              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 bg-red-500 rounded-full"></div>
                  <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
                  <div className="w-3 h-3 bg-green-500 rounded-full"></div>
                  <span className="ml-4 text-gray-400 text-sm font-mono">terminal</span>
                </div>
                <button
                  onClick={copyToClipboard}
                  className="flex items-center gap-2 px-3 py-1.5 text-sm bg-gray-800 hover:bg-gray-700 text-gray-300 rounded border border-gray-600 transition-colors"
                >
                  {copied ? (
                    <>
                      <svg className="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                      </svg>
                      Copied!
                    </>
                  ) : (
                    <>
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                      </svg>
                      Copy
                    </>
                  )}
                </button>
              </div>
              <pre className="text-left text-green-400 font-mono text-sm md:text-base leading-relaxed overflow-x-auto">
                <code>{`docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \\
  ghcr.io/hikaruegashira/vulnhuntrs:latest \\
  --repo your-org/your-repo`}</code>
              </pre>
            </div>
          </div>

          {/* Additional info */}
          <div className="mt-12 grid grid-cols-1 md:grid-cols-3 gap-8 text-left">
            <div className="bg-gray-800/50 backdrop-blur-sm rounded-lg p-6 border border-gray-700">
              <div className="w-10 h-10 bg-purple-600 rounded-lg flex items-center justify-center mb-4">
                <span className="text-white font-bold">1</span>
              </div>
              <h3 className="text-white font-semibold mb-2">Set API Key</h3>
              <p className="text-gray-400 text-sm">Export your OpenAI API key as an environment variable</p>
            </div>
            
            <div className="bg-gray-800/50 backdrop-blur-sm rounded-lg p-6 border border-gray-700">
              <div className="w-10 h-10 bg-pink-600 rounded-lg flex items-center justify-center mb-4">
                <span className="text-white font-bold">2</span>
              </div>
              <h3 className="text-white font-semibold mb-2">Run Scanner</h3>
              <p className="text-gray-400 text-sm">Execute the Docker command with your target repository</p>
            </div>
            
            <div className="bg-gray-800/50 backdrop-blur-sm rounded-lg p-6 border border-gray-700">
              <div className="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center mb-4">
                <span className="text-white font-bold">3</span>
              </div>
              <h3 className="text-white font-semibold mb-2">Get Results</h3>
              <p className="text-gray-400 text-sm">Review detailed vulnerability reports with remediation guidance</p>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-24 bg-gray-900">
        <div className="max-w-7xl mx-auto px-6">
          <div className="text-center mb-16">
            <h2 className="text-4xl md:text-5xl font-bold text-white mb-6">
              Why Choose Vulnhuntrs?
            </h2>
            <p className="text-xl text-gray-300 max-w-3xl mx-auto">
              Advanced AI-powered security analysis that goes beyond traditional static analysis tools
            </p>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            <div className="group bg-gradient-to-br from-gray-800 to-gray-900 rounded-2xl p-8 border border-gray-700 hover:border-purple-500/50 transition-all duration-300 hover:scale-105">
              <div className="w-14 h-14 bg-gradient-to-br from-purple-500 to-pink-500 rounded-xl flex items-center justify-center mb-6">
                <svg className="h-7 w-7 text-white" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
                </svg>
              </div>
              <h3 className="text-xl font-bold text-white mb-4">Multi-Language Support</h3>
              <p className="text-gray-300 leading-relaxed">
                Native support for Rust, Python, JavaScript, TypeScript, Go, Java, and Ruby with advanced tree-sitter parsing.
              </p>
            </div>

            <div className="group bg-gradient-to-br from-gray-800 to-gray-900 rounded-2xl p-8 border border-gray-700 hover:border-purple-500/50 transition-all duration-300 hover:scale-105">
              <div className="w-14 h-14 bg-gradient-to-br from-blue-500 to-cyan-500 rounded-xl flex items-center justify-center mb-6">
                <svg className="h-7 w-7 text-white" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456z" />
                </svg>
              </div>
              <h3 className="text-xl font-bold text-white mb-4">AI-Powered Analysis</h3>
              <p className="text-gray-300 leading-relaxed">
                Leverages large language models to identify complex security patterns beyond traditional static analysis.
              </p>
            </div>

            <div className="group bg-gradient-to-br from-gray-800 to-gray-900 rounded-2xl p-8 border border-gray-700 hover:border-purple-500/50 transition-all duration-300 hover:scale-105">
              <div className="w-14 h-14 bg-gradient-to-br from-green-500 to-emerald-500 rounded-xl flex items-center justify-center mb-6">
                <svg className="h-7 w-7 text-white" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0-1.125-.504-1.125-1.125V11.25a9 9 0 00-9-9z" />
                </svg>
              </div>
              <h3 className="text-xl font-bold text-white mb-4">Detailed Reports</h3>
              <p className="text-gray-300 leading-relaxed">
                Comprehensive vulnerability reports with proof-of-concept exploits and actionable remediation guidance.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-black py-12">
        <div className="max-w-4xl mx-auto px-6 text-center">
          <div className="flex justify-center mb-6">
            <Image
              src="/vulnhuntrs/logo.png"
              alt="Vulnhuntrs Logo"
              width={60}
              height={60}
            />
          </div>
          <p className="text-gray-400 mb-4">
            Built with ❤️ by the security community
          </p>
          <div className="flex justify-center space-x-6">
            <a
              href="https://github.com/HikaruEgashira/vulnhuntrs"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white transition-colors"
            >
              GitHub
            </a>
            <a
              href="https://github.com/HikaruEgashira/vulnhuntrs/issues"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white transition-colors"
            >
              Issues
            </a>
          </div>
        </div>
      </footer>
    </div>
  );
}
