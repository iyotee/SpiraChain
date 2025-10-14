'use client';

import dynamic from 'next/dynamic';

// Load all components with SSR disabled to avoid window/Framer Motion issues
const Navbar = dynamic(() => import('@/components/Navbar'), { ssr: false });
const Hero = dynamic(() => import('@/components/Hero'), { ssr: false });
const Features = dynamic(() => import('@/components/Features'), { ssr: false });
const Performance = dynamic(() => import('@/components/Performance'), { ssr: false });
const Tokenomics = dynamic(() => import('@/components/Tokenomics'), { ssr: false });
const Roadmap = dynamic(() => import('@/components/Roadmap'), { ssr: false });
const GetStarted = dynamic(() => import('@/components/GetStarted'), { ssr: false });
const Footer = dynamic(() => import('@/components/Footer'), { ssr: false });

export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-b from-white to-slate-50 dark:from-slate-950 dark:to-slate-900">
      <Navbar />
      <Hero />
      <Features />
      <Performance />
      <Tokenomics />
      <Roadmap />
      <GetStarted />
      <Footer />
    </main>
  );
}
