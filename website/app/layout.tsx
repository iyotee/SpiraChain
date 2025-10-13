import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';
import { ThemeProvider } from '@/components/ThemeProvider';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'SpiraChain - Post-Quantum Semantic Blockchain',
  description: 'The world\'s first post-quantum semantic blockchain. Powered by AI, secured by mathematics, accessible to everyone.',
  keywords: ['blockchain', 'post-quantum', 'cryptocurrency', 'AI', 'semantic', 'SpiraChain', 'Qubitum', 'QBT'],
  authors: [{ name: 'SpiraChain Team' }],
  openGraph: {
    title: 'SpiraChain - Post-Quantum Semantic Blockchain',
    description: 'Validate with a Raspberry Pi. Earn rewards for quality, not quantity.',
    type: 'website',
    url: 'https://spirachain.org',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'SpiraChain - Post-Quantum Semantic Blockchain',
    description: 'The world\'s first post-quantum semantic blockchain',
  }
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        <script
          dangerouslySetInnerHTML={{
            __html: `
              // Netlify className fix
              (function() {
                // Ensure className is always a string
                if (typeof window !== 'undefined') {
                  const originalCreateElement = document.createElement;
                  document.createElement = function(tagName) {
                    const element = originalCreateElement.call(this, tagName);
                    if (!element.className || typeof element.className !== 'string') {
                      element.className = '';
                    }
                    return element;
                  };
                  
                  // Fix existing elements
                  const fixClassName = function(node) {
                    if (node.className && typeof node.className !== 'string') {
                      node.className = '';
                    }
                    for (let child of node.children) {
                      fixClassName(child);
                    }
                  };
                  
                  // Run on DOM ready
                  if (document.readyState === 'loading') {
                    document.addEventListener('DOMContentLoaded', () => fixClassName(document.body));
                  } else {
                    fixClassName(document.body);
                  }
                }
              })();
            `,
          }}
        />
      </head>
      <body className={inter.className}>
        <ThemeProvider>
          {children}
        </ThemeProvider>
      </body>
    </html>
  );
}
