# Parsentry Website

This is the documentation website for Parsentry, an AI-powered security vulnerability scanner.

## About Parsentry

Parsentry is a PAR (Principal-Action-Resource) based security scanner that combines static code analysis with LLMs to detect vulnerabilities across multiple languages including IaC. It provides comprehensive multi-language security analysis.

## Website Development

This website is built with [Next.js](https://nextjs.org) and [Fumadocs](https://fumadocs.vercel.app/).

### Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

### Building for Production

```bash
npm run build
npm run start
```

### Features

- **Modern Design**: Clean, responsive design with dark mode support
- **Interactive Documentation**: Comprehensive guides and API documentation
- **Search**: Built-in search functionality
- **Responsive**: Works on desktop, tablet, and mobile devices

## Project Structure

- `src/app/` - Next.js app directory
- `src/app/page.tsx` - Homepage
- `src/app/docs/` - Documentation pages
- `public/` - Static assets

## Deployment

The website is automatically deployed to GitHub Pages when changes are pushed to the main branch.

## Contributing

When updating the website content, ensure that:

1. All information reflects the current Parsentry implementation
2. Code examples are accurate and tested
3. Feature descriptions match the actual capabilities
4. Links and references are up to date

## Learn More

- [Parsentry Repository](https://github.com/HikaruEgashira/parsentry)
- [Next.js Documentation](https://nextjs.org/docs)
- [Fumadocs Documentation](https://fumadocs.vercel.app/)
