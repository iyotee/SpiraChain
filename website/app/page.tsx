'use client';

import Navbar from '@/components/Navbar';
import Hero from '@/components/Hero';
import Features from '@/components/Features';
import Technology from '@/components/Technology';
import UseCases from '@/components/UseCases';
import Tokenomics from '@/components/Tokenomics';
import Roadmap from '@/components/Roadmap';
import Ecosystem from '@/components/Ecosystem';
import GetStarted from '@/components/GetStarted';
import Footer from '@/components/Footer';

export const dynamic = 'force-dynamic';
export const runtime = 'edge';

export default function Home() {
  return (
    <main className="min-h-screen overflow-x-hidden">
      <Navbar />
      <Hero />
      <Features />
      <Technology />
      <UseCases />
      <Tokenomics />
      <Roadmap />
      <Ecosystem />
      <GetStarted />
      <Footer />
    </main>
  );
}
