'use client';

import React from 'react';
import { motion } from 'framer-motion';
import { ArrowRight, Github, Sparkles, TrendingUp, Users, Zap } from 'lucide-react';
import GradientButton from './GradientButton';
import StatsCounter from './StatsCounter';

export default function Hero() {
  const trustIndicators = [
    { icon: Users, value: 150, suffix: '+', label: 'Validators', gradient: 'from-blue-500 to-cyan-400' },
    { icon: Zap, value: 2.5, suffix: 'M', decimals: 1, label: 'Transactions', gradient: 'from-purple-500 to-pink-500' },
    { icon: TrendingUp, value: 99.9, suffix: '%', decimals: 1, label: 'Uptime', gradient: 'from-green-500 to-emerald-400' },
  ];

  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden pt-20 bg-white dark:bg-slate-950">
      <div className="absolute inset-0 bg-gradient-to-br from-purple-50 via-pink-50 to-blue-50 dark:from-slate-950 dark:via-purple-950 dark:to-slate-950">
        <div className="absolute inset-0">
          <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-purple-600/30 rounded-full blur-3xl animate-pulse" />
          <div className="absolute bottom-1/4 right-1/4 w-[600px] h-[600px] bg-blue-600/30 rounded-full blur-3xl animate-pulse" style={{ animationDelay: '1s' }} />
          <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-pink-600/20 rounded-full blur-3xl animate-pulse" style={{ animationDelay: '2s' }} />
        </div>

        <div className="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjAiIGhlaWdodD0iNjAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGRlZnM+PHBhdHRlcm4gaWQ9ImdyaWQiIHdpZHRoPSI2MCIgaGVpZ2h0PSI2MCIgcGF0dGVyblVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+PHBhdGggZD0iTSAxMCAwIEwgMCAwIDAgMTAiIGZpbGw9Im5vbmUiIHN0cm9rZT0icmdiYSgyNTUsMjU1LDI1NSwwLjAzKSIgc3Ryb2tlLXdpZHRoPSIxIi8+PC9wYXR0ZXJuPjwvZGVmcz48cmVjdCB3aWR0aD0iMTAwJSIgaGVpZ2h0PSIxMDAlIiBmaWxsPSJ1cmwoI2dyaWQpIi8+PC9zdmc+')] opacity-40" />

        {[...Array(50)].map((_, i) => (
          <motion.div
            key={i}
            className="absolute w-1 h-1 bg-white rounded-full"
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
            }}
            animate={{
              y: [0, -30, 0],
              opacity: [0.1, 0.8, 0.1],
              scale: [1, 1.5, 1],
            }}
            transition={{
              duration: 3 + Math.random() * 4,
              repeat: Infinity,
              delay: Math.random() * 5,
            }}
          />
        ))}
      </div>

      <div className="relative z-10 container mx-auto px-4 sm:px-6 lg:px-8 py-16 md:py-24">
        <div className="text-center max-w-6xl mx-auto">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            className="mb-8"
          >
            <motion.div
              initial={{ scale: 0.8, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              transition={{ duration: 0.6, delay: 0.2 }}
              className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-purple-600/10 via-pink-600/10 to-blue-600/10 dark:from-purple-600/20 dark:via-pink-600/20 dark:to-blue-600/20 border border-purple-500/30 rounded-full backdrop-blur-sm mb-8"
            >
              <Sparkles className="w-5 h-5 text-purple-600 dark:text-purple-400 animate-pulse" />
              <span className="text-purple-700 dark:text-purple-300 font-bold">Testnet Live - Join the Revolution</span>
            </motion.div>

            <motion.h1
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.8, delay: 0.3 }}
              className="text-5xl sm:text-6xl md:text-7xl lg:text-8xl font-black mb-6 leading-tight"
            >
              <span className="block text-slate-900 dark:text-white mb-2">
                The World&apos;s First
              </span>
              <span className="block gradient-text-hero mb-2">
                Post-Quantum
              </span>
              <span className="block gradient-text-blue">
                Semantic Blockchain
              </span>
            </motion.h1>

            <motion.p
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.8, delay: 0.5 }}
              className="text-xl sm:text-2xl md:text-3xl text-slate-700 dark:text-gray-300 mb-12 max-w-4xl mx-auto leading-relaxed"
            >
              <span className="text-slate-900 dark:text-white font-bold">Powered by AI</span>, secured by mathematics, accessible to everyone.
              <br className="hidden sm:block" />
              <span className="block mt-4 text-purple-600 dark:text-purple-400 font-black text-2xl sm:text-3xl md:text-4xl">
                Validate with a Raspberry Pi. Earn rewards for quality.
              </span>
            </motion.p>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.7 }}
            className="flex flex-col sm:flex-row gap-4 sm:gap-6 justify-center items-center mb-16"
          >
            <GradientButton href="#get-started" size="xl" icon={Sparkles} iconPosition="left">
              Get Started Now
            </GradientButton>

            <GradientButton 
              href="https://github.com/iyotee/SpiraChain" 
              variant="outline" 
              size="xl"
              icon={Github}
              iconPosition="left"
            >
              View on GitHub
            </GradientButton>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 40 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.9 }}
            className="grid grid-cols-1 sm:grid-cols-3 gap-6 md:gap-8 max-w-5xl mx-auto"
          >
            {trustIndicators.map((indicator, index) => {
              const IconComponent = indicator.icon;
              return (
                <div
                  key={indicator.label}
                  className="relative p-6 md:p-8 glass-card rounded-3xl shadow-2xl group hover:scale-105 transition-transform duration-300"
                >
                  <div className={`inline-flex p-3 rounded-2xl bg-gradient-to-r ${indicator.gradient} shadow-lg mb-4`}>
                    <IconComponent className="w-6 h-6 text-white" />
                  </div>
                  
                  <div className={`text-4xl md:text-5xl font-black bg-gradient-to-r ${indicator.gradient} bg-clip-text text-transparent mb-2`}>
                    <StatsCounter
                      end={indicator.value}
                      suffix={indicator.suffix}
                      decimals={indicator.decimals || 0}
                      gradient={indicator.gradient}
                      label=""
                      delay={1.1 + index * 0.2}
                    />
                  </div>
                  
                  <div className="text-sm font-semibold text-slate-600 dark:text-gray-300">
                    {indicator.label}
                  </div>

                  <div className={`absolute inset-0 bg-gradient-to-r ${indicator.gradient} opacity-0 group-hover:opacity-10 dark:group-hover:opacity-5 rounded-3xl transition-opacity duration-300 pointer-events-none`} />
                </div>
              );
            })}
          </motion.div>
        </div>
      </div>

      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 1, delay: 1.5 }}
        className="absolute bottom-8 left-1/2 -translate-x-1/2 z-20"
      >
        <motion.div
          animate={{ y: [0, 12, 0] }}
          transition={{ duration: 2, repeat: Infinity }}
          className="w-8 h-14 border-2 border-slate-400/40 dark:border-white/40 rounded-full flex justify-center cursor-pointer hover:border-slate-600/60 dark:hover:border-white/60 transition-colors"
        >
          <motion.div
            animate={{ y: [0, 16, 0] }}
            transition={{ duration: 2, repeat: Infinity }}
            className="w-1.5 h-4 bg-gradient-to-b from-purple-600 to-blue-600 dark:from-purple-400 dark:to-blue-400 rounded-full mt-3"
          />
        </motion.div>
      </motion.div>
    </section>
  );
}
