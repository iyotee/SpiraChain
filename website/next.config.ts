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
  
  // Skip static optimization for favicon
  skipTrailingSlashRedirect: true,
  skipMiddlewareUrlNormalize: true,
  
  // Disable automatic metadata generation completely
  experimental: {
    optimizePackageImports: [],
    metadataFileGeneration: false,
  },
  
  // Disable automatic static optimization for problematic routes
  // experimental: {
  //   missingSuspenseWithCSRBailout: false,
  // },
  
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
    
    // Completely ignore favicon-related imports
    config.module.rules.push({
      test: /favicon\.ico$/,
      use: 'null-loader',
    });
    
    config.module.rules.push({
      test: /next-metadata-image-loader/,
      use: 'null-loader',
    });
    
    config.module.rules.push({
      test: /__next_metadata__/,
      use: 'null-loader',
    });
    
    // Ignore all favicon-related modules
    config.resolve.alias = {
      ...config.resolve.alias,
      'favicon.ico': false,
      './favicon.ico': false,
      '../favicon.ico': false,
    };
    
    // Add ignore plugin for favicon
    config.plugins.push(
      new webpack.IgnorePlugin({
        resourceRegExp: /favicon\.ico$/,
      })
    );
    
    return config;
  },
};

export default nextConfig;