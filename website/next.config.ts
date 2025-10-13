import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  experimental: {
    // Disable problematic features that might cause className issues
    optimizePackageImports: [],
  },
  
  // Ensure proper hydration
  reactStrictMode: true,
  
  // Fix for className issues
  webpack: (config, { isServer }) => {
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
      };
    }
    
    // Add className polyfill
    config.module.rules.push({
      test: /\.js$/,
      use: {
        loader: 'string-replace-loader',
        options: {
          search: 'className.includes',
          replace: '(typeof className === "string" && className.includes)',
          flags: 'g'
        }
      }
    });
    
    return config;
  },
};

export default nextConfig;