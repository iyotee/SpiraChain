'use client';

import dynamic from 'next/dynamic';
import ClientWrapper from '@/components/ClientWrapper';

// Import components normally to ensure CSS is included
import Navbar from '@/components/Navbar';
import Footer from '@/components/Footer';

// Load only heavy animation components dynamically
const Hero = dynamic(() => import('@/components/Hero'), { ssr: false });
const Features = dynamic(() => import('@/components/Features'), { ssr: false });
const Performance = dynamic(() => import('@/components/Performance'), { ssr: false });
const Tokenomics = dynamic(() => import('@/components/Tokenomics'), { ssr: false });
const Roadmap = dynamic(() => import('@/components/Roadmap'), { ssr: false });
const GetStarted = dynamic(() => import('@/components/GetStarted'), { ssr: false });

export default function Home() {
  return (
    <ClientWrapper>
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
    </ClientWrapper>
  );
}
