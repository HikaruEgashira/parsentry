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
      <section className="relative overflow-hidden bg-gradient-to-br from-blue-50 via-white to-purple-50 dark:from-gray-900 dark:via-gray-900 dark:to-blue-950">
        {/* Background decoration */}
        <div className="absolute inset-0 bg-grid-gray-100/50 dark:bg-grid-gray-800/50" 
             style={{
               backgroundImage: 'radial-gradient(circle at 1px 1px, rgba(255,255,255,0.15) 1px, transparent 0)',
               backgroundSize: '20px 20px'
             }}>
        </div>
        <div className="absolute top-0 right-0 -z-10 transform-gpu overflow-hidden blur-3xl">
          <div className="relative left-[calc(50%-11rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-blue-400 to-purple-600 opacity-20 sm:left-[calc(50%-30rem)] sm:w-[72.1875rem]"
               style={{
                 clipPath: 'polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)'
               }}>
          </div>
        </div>
        
        <div className="relative px-6 pt-14 lg:px-8">
          <div className="mx-auto max-w-4xl py-32 sm:py-48 lg:py-56">
            <div className="text-center">
              {/* Logo */}
              <div className="mb-12 flex justify-center">
                <Image
                  src="/vulnhuntrs/logo.png"
                  alt="Vulnhuntrs Logo"
                  width={120}
                  height={120}
                  priority
                />
              </div>

              {/* Badge */}
              <div className="mb-8 flex justify-center">
                <div className="relative rounded-full px-4 py-1.5 text-sm leading-6 text-gray-600 dark:text-gray-300 ring-1 ring-gray-900/10 dark:ring-gray-100/10 hover:ring-gray-900/20 dark:hover:ring-gray-100/20 transition-all">
                  <span className="font-semibold text-blue-600 dark:text-blue-400">AI-Powered</span> security analysis 
                  <span className="absolute inset-0 rounded-full bg-gradient-to-r from-blue-400 to-purple-600 opacity-5"></span>
                </div>
              </div>

              {/* Heading */}
              <h1 className="text-4xl font-bold tracking-tight text-gray-900 dark:text-white sm:text-6xl lg:text-7xl">
                <span className="block">Vulnhuntrs</span>
                <span className="block bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                  Security Scanner
                </span>
              </h1>

              {/* Description */}
              <p className="mt-6 text-lg leading-8 text-gray-600 dark:text-gray-300 max-w-3xl mx-auto">
                Combine the power of static code analysis with large language models to detect remotely exploitable vulnerabilities across multiple programming languages.
              </p>

              {/* CTA Buttons */}
              <div className="mt-10 flex flex-col sm:flex-row items-center justify-center gap-4">
                <button
                  onClick={scrollToQuickStart}
                  className="rounded-lg bg-gradient-to-r from-blue-600 to-purple-600 px-8 py-3 text-sm font-semibold text-white shadow-lg hover:shadow-xl transition-all duration-200 hover:scale-105"
                >
                  Get Started
                </button>
                <a
                  href="https://github.com/HikaruEgashira/vulnhuntrs"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="rounded-lg bg-white/10 backdrop-blur-sm px-8 py-3 text-sm font-semibold text-gray-900 dark:text-white ring-1 ring-gray-900/10 dark:ring-white/20 hover:ring-gray-900/20 dark:hover:ring-white/30 transition-all duration-200 hover:scale-105"
                >
                  <span className="flex items-center gap-2">
                    <svg className="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                    </svg>
                    View on GitHub
                  </span>
                </a>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Quick Start Section */}
      <section id="quick-start" className="py-24 bg-gradient-to-br from-blue-50 via-white to-purple-50 dark:from-gray-900 dark:via-gray-900 dark:to-blue-950">
        <div className="max-w-4xl mx-auto px-6 text-center">
          <div className="mb-16">
            <h2 className="text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-6">
              Quick Start
            </h2>
            <p className="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
              Get started with Vulnhuntrs in seconds. Just run this single Docker command:
            </p>
          </div>

          {/* Docker Command */}
          <div className="relative group max-w-4xl mx-auto">
            <div className="absolute -inset-1 bg-gradient-to-r from-blue-500 to-purple-500 rounded-lg blur opacity-20 group-hover:opacity-40 transition duration-1000"></div>
            <div className="relative bg-white dark:bg-gray-900 rounded-lg p-6 border border-gray-200 dark:border-gray-700 shadow-lg">
              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 bg-red-500 rounded-full"></div>
                  <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
                  <div className="w-3 h-3 bg-green-500 rounded-full"></div>
                  <span className="ml-4 text-gray-500 dark:text-gray-400 text-sm font-mono">terminal</span>
                </div>
                <button
                  onClick={copyToClipboard}
                  className="flex items-center gap-2 px-3 py-1.5 text-sm bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 rounded border border-gray-300 dark:border-gray-600 transition-colors"
                >
                  {copied ? (
                    <>
                      <svg className="w-4 h-4 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
              <pre className="text-left text-blue-600 dark:text-green-400 font-mono text-sm md:text-base leading-relaxed overflow-x-auto">
                <code>{`docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \\
  ghcr.io/hikaruegashira/vulnhuntrs:latest \\
  --repo your-org/your-repo`}</code>
              </pre>
            </div>
          </div>

          {/* Additional info */}
          <div className="mt-12 grid grid-cols-1 md:grid-cols-3 gap-8 text-left">
            <div className="bg-white dark:bg-gray-800/50 backdrop-blur-sm rounded-lg p-6 border border-gray-200 dark:border-gray-700 shadow-md">
              <div className="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center mb-4">
                <span className="text-white font-bold">1</span>
              </div>
              <h3 className="text-gray-900 dark:text-white font-semibold mb-2">Set API Key</h3>
              <p className="text-gray-600 dark:text-gray-400 text-sm">Export your OpenAI API key as an environment variable</p>
            </div>
            
            <div className="bg-white dark:bg-gray-800/50 backdrop-blur-sm rounded-lg p-6 border border-gray-200 dark:border-gray-700 shadow-md">
              <div className="w-10 h-10 bg-purple-600 rounded-lg flex items-center justify-center mb-4">
                <span className="text-white font-bold">2</span>
              </div>
              <h3 className="text-gray-900 dark:text-white font-semibold mb-2">Run Scanner</h3>
              <p className="text-gray-600 dark:text-gray-400 text-sm">Execute the Docker command with your target repository</p>
            </div>
            
            <div className="bg-white dark:bg-gray-800/50 backdrop-blur-sm rounded-lg p-6 border border-gray-200 dark:border-gray-700 shadow-md">
              <div className="w-10 h-10 bg-green-600 rounded-lg flex items-center justify-center mb-4">
                <span className="text-white font-bold">3</span>
              </div>
              <h3 className="text-gray-900 dark:text-white font-semibold mb-2">Get Results</h3>
              <p className="text-gray-600 dark:text-gray-400 text-sm">Review detailed vulnerability reports with remediation guidance</p>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-24 sm:py-32 bg-gradient-to-br from-blue-50 via-white to-purple-50 dark:from-gray-900 dark:via-gray-900 dark:to-blue-950">
        <div className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="mx-auto max-w-2xl text-center">
            <h2 className="text-base font-semibold leading-7 text-blue-600 dark:text-blue-400">Everything you need</h2>
            <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 dark:text-white sm:text-4xl">
              Advanced security analysis capabilities
            </p>
            <p className="mt-6 text-lg leading-8 text-gray-600 dark:text-gray-300">
              Built with modern technologies to provide comprehensive security coverage for your codebase.
            </p>
          </div>
          
          <div className="mx-auto mt-16 max-w-6xl">
            <dl className="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
              <div className="flex flex-col items-start p-8 bg-gray-50 dark:bg-gray-800/50 rounded-2xl backdrop-blur-sm">
                <div className="rounded-lg bg-blue-600 p-2 ring-1 ring-blue-600/20">
                  <svg className="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
                  </svg>
                </div>
                <dt className="mt-4 font-semibold text-gray-900 dark:text-white">Multi-Language Support</dt>
                <dd className="mt-2 leading-7 text-gray-600 dark:text-gray-300">
                  Native support for Rust, Python, JavaScript, TypeScript, Go, Java, and Ruby with advanced tree-sitter parsing.
                </dd>
              </div>

              <div className="flex flex-col items-start p-8 bg-gray-50 dark:bg-gray-800/50 rounded-2xl backdrop-blur-sm">
                <div className="rounded-lg bg-purple-600 p-2 ring-1 ring-purple-600/20">
                  <svg className="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456z" />
                  </svg>
                </div>
                <dt className="mt-4 font-semibold text-gray-900 dark:text-white">AI-Powered Analysis</dt>
                <dd className="mt-2 leading-7 text-gray-600 dark:text-gray-300">
                  Leverages large language models to identify complex security patterns beyond traditional static analysis.
                </dd>
              </div>

              <div className="flex flex-col items-start p-8 bg-gray-50 dark:bg-gray-800/50 rounded-2xl backdrop-blur-sm">
                <div className="rounded-lg bg-green-600 p-2 ring-1 ring-green-600/20">
                  <svg className="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0-1.125-.504-1.125-1.125V11.25a9 9 0 00-9-9z" />
                  </svg>
                </div>
                <dt className="mt-4 font-semibold text-gray-900 dark:text-white">Detailed Reports</dt>
                <dd className="mt-2 leading-7 text-gray-600 dark:text-gray-300">
                  Comprehensive vulnerability reports with proof-of-concept exploits and actionable remediation guidance.
                </dd>
              </div>
            </dl>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-100 dark:bg-black py-12">
        <div className="max-w-4xl mx-auto px-6 text-center">
          <div className="flex justify-center mb-6">
            <Image
              src="/vulnhuntrs/logo.png"
              alt="Vulnhuntrs Logo"
              width={60}
              height={60}
            />
          </div>
          <p className="text-gray-600 dark:text-gray-400 mb-4">
            Built with ❤️ by the security community
          </p>
          <div className="flex justify-center space-x-6">
            <a
              href="https://github.com/HikaruEgashira/vulnhuntrs"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white transition-colors"
            >
              GitHub
            </a>
            <a
              href="https://github.com/HikaruEgashira/vulnhuntrs/issues"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white transition-colors"
            >
              Issues
            </a>
          </div>
        </div>
      </footer>
    </div>
  );
}
