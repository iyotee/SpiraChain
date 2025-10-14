import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Ensure proper hydration
  reactStrictMode: true,
  
  // Output configuration for Netlify
  output: 'export',
  trailingSlash: true,
  images: {
    unoptimized: true,
  },
  
  // Disable automatic favicon generation
  generateBuildId: async () => {
    return 'build-' + Date.now();
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