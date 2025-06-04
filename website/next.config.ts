import type { NextConfig } from 'next';

const isProd = process.env.NODE_ENV === 'production';

const config: NextConfig = {
  output: 'export',
  trailingSlash: true,
  basePath: isProd ? '/vulnhuntrs' : '',
  assetPrefix: isProd ? '/vulnhuntrs' : '',
  images: {
    unoptimized: true,
  },
  eslint: {
    ignoreDuringBuilds: true,
  },
  typescript: {
    ignoreBuildErrors: true,
  },
};

export default config;
