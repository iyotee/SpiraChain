'use client';

// IMPORT CSS FIRST to ensure it's included in the build
import './globals.css';

import dynamic from 'next/dynamic';

const HomeClient = dynamic(() => import('./page.client'), {
  ssr: false,
  loading: () => (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 flex items-center justify-center">
      <div className="text-center">
        <h1 className="text-8xl font-black text-white mb-8">
          <span className="bg-gradient-to-r from-purple-400 via-pink-500 to-red-400 bg-clip-text text-transparent">
            SpiraChain
          </span>
        </h1>
        <p className="text-2xl text-gray-300 animate-pulse">Loading...</p>
      </div>
    </div>
  ),
});

export default function Home() {
  return <HomeClient />;
}
