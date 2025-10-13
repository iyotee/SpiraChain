import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Disable Turbopack for Netlify compatibility
  experimental: {
    turbo: {
      // Disable Turbopack to avoid conflicts
    },
  },
  
  // Ensure proper hydration
  reactStrictMode: true,
  
  // Output configuration for Netlify
  output: 'export',
  trailingSlash: true,
  images: {
    unoptimized: true,
  },
  
  // Fix for className issues - Netlify specific
  webpack: (config, { isServer, webpack }) => {
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        net: false,
        tls: false,
      };
    }
    
    // Add global className polyfill
    config.plugins.push(
      new webpack.DefinePlugin({
        'typeof window': JSON.stringify('object'),
      })
    );
    
    return config;
  },
};

export default nextConfig;