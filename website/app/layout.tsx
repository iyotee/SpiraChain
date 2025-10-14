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
  icons: {
    icon: '/favicon.svg',
  },
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
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
        <link rel="icon" href="/favicon.ico" sizes="any" />
      </head>
      <body className={inter.className}>
        <ThemeProvider>
          {children}
        </ThemeProvider>
      </body>
    </html>
  );
}
