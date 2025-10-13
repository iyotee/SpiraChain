'use client';

// import { motion } from 'framer-motion';
import Navbar from '@/components/Navbar';
import Hero from '@/components/Hero';
import Features from '@/components/Features';
import Performance from '@/components/Performance';
import Tokenomics from '@/components/Tokenomics';
import Roadmap from '@/components/Roadmap';
import GetStarted from '@/components/GetStarted';
import Footer from '@/components/Footer';

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
