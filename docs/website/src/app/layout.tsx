import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { RootProvider } from 'fumadocs-ui/provider';

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
  display: "swap",
});

export const metadata: Metadata = {
  title: {
    template: "%s | Vulnhuntrs",
    default: "Vulnhuntrs - AI-Powered Security Scanner",
  },
  description: "AI-powered security vulnerability scanner that combines static code analysis with LLMs to detect remotely exploitable vulnerabilities.",
  keywords: ["security", "vulnerability", "scanner", "AI", "static analysis", "LLM"],
  authors: [{ name: "Hikaru Egashira" }],
  creator: "Hikaru Egashira",
  metadataBase: new URL("https://hikaruegashira.github.io"),
  openGraph: {
    type: "website",
    locale: "en_US",
    url: "https://hikaruegashira.github.io/vulnhuntrs/",
    title: "Vulnhuntrs - AI-Powered Security Scanner",
    description: "AI-powered security vulnerability scanner that combines static code analysis with LLMs to detect remotely exploitable vulnerabilities.",
    siteName: "Vulnhuntrs",
  },
  twitter: {
    card: "summary_large_image",
    title: "Vulnhuntrs - AI-Powered Security Scanner",
    description: "AI-powered security vulnerability scanner that combines static code analysis with LLMs to detect remotely exploitable vulnerabilities.",
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={`${inter.variable} font-sans antialiased`}>
        <RootProvider
          search={{
            enabled: true,
          }}
          theme={{
            enabled: true,
            defaultTheme: "system",
          }}
        >
          {children}
        </RootProvider>
      </body>
    </html>
  );
}
