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
              // PROTECTION CONTRE LES EXTENSIONS BROWSER
              (function() {
                if (typeof window !== 'undefined') {
                  // 1. PROTECTION className
                  const originalCreateElement = document.createElement;
                  document.createElement = function(tagName) {
                    const element = originalCreateElement.call(this, tagName);
                    if (!element.className || typeof element.className !== 'string') {
                      element.className = '';
                    }
                    return element;
                  };
                  
                  // 2. PROTECTION contre les extensions
                  const protectClassName = function(node) {
                    if (node && node.className && typeof node.className !== 'string') {
                      node.className = '';
                    }
                    if (node && node.children) {
                      for (let child of node.children) {
                        protectClassName(child);
                      }
                    }
                  };
                  
                  // 3. SURVEILLANCE continue
                  const observer = new MutationObserver(function(mutations) {
                    mutations.forEach(function(mutation) {
                      mutation.addedNodes.forEach(function(node) {
                        if (node.nodeType === 1) { // Element node
                          protectClassName(node);
                        }
                      });
                    });
                  });
                  
                  // 4. DÉMARRAGE
                  if (document.readyState === 'loading') {
                    document.addEventListener('DOMContentLoaded', function() {
                      protectClassName(document.body);
                      observer.observe(document.body, { childList: true, subtree: true });
                    });
                  } else {
                    protectClassName(document.body);
                    observer.observe(document.body, { childList: true, subtree: true });
                  }
                  
                  // 5. FORCER LE CSS
                  const forceCSS = function() {
                    const style = document.createElement('style');
                    style.innerHTML = \`
                      body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif !important; }
                      .gradient-text { background: linear-gradient(135deg, #6366f1, #a855f7, #ec4899) !important; -webkit-background-clip: text !important; -webkit-text-fill-color: transparent !important; }
                    \`;
                    document.head.appendChild(style);
                  };
                  
                  forceCSS();
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
