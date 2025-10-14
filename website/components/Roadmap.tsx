'use client';

import { motion } from 'framer-motion';
import { CheckCircle2, Circle, Rocket, Shield, Users, TrendingUp } from 'lucide-react';
import GradientButton from './GradientButton';

export default function Roadmap() {
  const phases = [
    {
      quarter: 'Q4 2024',
      status: 'Completed',
      completion: 100,
      title: 'Core Development',
      icon: CheckCircle2,
      iconColor: 'text-green-500',
      bgColor: 'bg-green-50 dark:bg-green-900/20',
      borderColor: 'border-green-500',
      gradient: 'from-green-500 to-emerald-500',
      items: [
        { text: 'Proof-of-Spiral consensus implementation', done: true },
        { text: 'Post-quantum cryptography integration', done: true },
        { text: 'AI semantic layer development', done: true },
        { text: 'RPC API complete with documentation', done: true },
      ],
      delay: 0.1,
    },
    {
      quarter: 'Q1 2025',
      status: 'In Progress',
      completion: 75,
      title: 'Public Testnet',
      icon: Users,
      iconColor: 'text-indigo-500 animate-pulse',
      bgColor: 'bg-indigo-50 dark:bg-indigo-900/20',
      borderColor: 'border-indigo-500',
      gradient: 'from-indigo-500 to-purple-500',
      items: [
        { text: 'Community testing program', done: true },
        { text: '150+ active validators', done: true },
        { text: 'Bug bounty program launch', done: true },
        { text: 'Discord & Telegram communities', done: false },
      ],
      delay: 0.2,
      isCurrent: true,
    },
    {
      quarter: 'Q2 2026',
      status: 'Planned',
      completion: 0,
      title: 'Infrastructure & Security',
      icon: Shield,
      iconColor: 'text-slate-400 dark:text-slate-500',
      bgColor: 'bg-slate-50 dark:bg-slate-800/50',
      borderColor: 'border-slate-300 dark:border-slate-700',
      gradient: 'from-slate-400 to-slate-500',
      items: [
        { text: 'Multi-region bootstrap nodes', done: false },
        { text: 'Professional security audit', done: false },
        { text: 'Advanced blockchain explorer', done: false },
        { text: 'Mobile wallet applications', done: false },
      ],
      delay: 0.3,
    },
    {
      quarter: 'Q3 2026',
      status: 'Planned',
      completion: 0,
      title: 'MAINNET LAUNCH',
      icon: Rocket,
      iconColor: 'text-purple-500',
      bgColor: 'bg-gradient-to-br from-purple-600 via-pink-600 to-blue-600',
      borderColor: 'border-purple-500',
      gradient: 'from-purple-600 to-pink-600',
      textColor: 'text-white',
      items: [
        { text: 'Genesis block creation', done: false },
        { text: 'DEX and CEX listings', done: false },
        { text: '500+ validator network', done: false },
        { text: '24/7 global support', done: false },
      ],
      delay: 0.4,
      isMainnet: true,
    },
  ];

  return (
    <section id="roadmap" className="py-24 md:py-32 relative overflow-hidden bg-white dark:bg-slate-950">
      <div className="absolute inset-0 opacity-30">
        <div className="absolute top-20 right-20 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl" />
        <div className="absolute bottom-20 left-20 w-96 h-96 bg-pink-500/10 rounded-full blur-3xl" />
      </div>

      <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="text-center mb-16 md:mb-20"
        >
          <motion.div
            initial={{ scale: 0.8, opacity: 0 }}
            whileInView={{ scale: 1, opacity: 1 }}
            transition={{ duration: 0.6, delay: 0.2 }}
            viewport={{ once: true }}
            className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-purple-600/10 via-pink-600/10 to-blue-600/10 border border-purple-500/20 rounded-full backdrop-blur-sm mb-8"
          >
            <TrendingUp className="w-5 h-5 text-purple-600 dark:text-purple-400" />
            <span className="text-sm font-bold text-purple-600 dark:text-purple-400">Our Journey</span>
          </motion.div>

          <h2 className="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black mb-6">
            <span className="block text-slate-900 dark:text-white">Roadmap to</span>
            <span className="block gradient-text">Mainnet Launch</span>
          </h2>
          <p className="text-lg sm:text-xl md:text-2xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            Our journey to revolutionize blockchain technology
          </p>
        </motion.div>

        <div className="max-w-6xl mx-auto">
          <div className="relative">
            <div className="absolute left-8 md:left-32 top-0 bottom-0 w-1 bg-gradient-to-b from-green-500 via-indigo-500 via-slate-300 to-purple-500 opacity-30" />
            
            <div className="space-y-12 md:space-y-16">
              {phases.map((phase, index) => {
                const Icon = phase.icon;
                return (
                  <motion.div
                    key={phase.quarter}
                    initial={{ opacity: 0, x: index % 2 === 0 ? -50 : 50 }}
                    whileInView={{ opacity: 1, x: 0 }}
                    viewport={{ once: true }}
                    transition={{ duration: 0.6, delay: phase.delay }}
                    className="relative"
                  >
                    <div className="flex items-start gap-6 md:gap-8">
                      <div className="flex-shrink-0 w-16 md:w-32 text-right pt-2">
                        <div className={`font-bold text-base md:text-lg ${phase.isMainnet ? 'text-purple-600 dark:text-purple-400' : 'text-slate-900 dark:text-white'}`}>
                          {phase.quarter}
                        </div>
                        <div className={`text-xs md:text-sm ${phase.isMainnet ? 'text-purple-600 dark:text-purple-400 font-semibold' : 'text-slate-600 dark:text-slate-400'}`}>
                          {phase.status}
                        </div>
                      </div>

                      <div className="relative flex-shrink-0 mt-1">
                        <div className={`w-14 h-14 md:w-16 md:h-16 rounded-2xl border-4 ${phase.borderColor} ${phase.bgColor} flex items-center justify-center shadow-lg z-10 relative`}>
                          <Icon className={`${phase.iconColor} w-7 h-7 md:w-8 md:h-8`} />
                        </div>
                        {phase.isCurrent && (
                          <div className={`absolute inset-0 rounded-2xl bg-gradient-to-r ${phase.gradient} animate-pulse-glow blur-md`} />
                        )}
                      </div>

                      <motion.div
                        whileHover={{ scale: 1.02, y: -4 }}
                        className={`flex-1 ${phase.bgColor} ${phase.isMainnet ? phase.textColor : ''} p-6 md:p-8 rounded-3xl shadow-xl border-2 ${phase.borderColor} relative overflow-hidden group`}
                      >
                        {phase.isCurrent && (
                          <div className="absolute top-4 right-4 z-20">
                            <span className="px-3 py-1 bg-gradient-to-r from-indigo-600 to-purple-600 text-white text-xs font-bold rounded-full animate-pulse">
                              ACTIVE
                            </span>
                          </div>
                        )}

                        <div className={`flex items-center gap-3 mb-4 ${phase.isMainnet ? 'text-white' : ''}`}>
                          {phase.isMainnet && <Rocket className="w-6 h-6" />}
                          <h4 className="font-black text-xl md:text-2xl">
                            {phase.title}
                          </h4>
                        </div>

                        {phase.completion > 0 && (
                          <div className="mb-4">
                            <div className="flex items-center justify-between mb-2">
                              <span className={`text-xs md:text-sm font-semibold ${phase.isMainnet ? 'text-white/80' : 'text-slate-600 dark:text-slate-400'}`}>
                                Progress
                              </span>
                              <span className={`text-sm md:text-base font-bold ${phase.isMainnet ? 'text-white' : 'text-slate-900 dark:text-white'}`}>
                                {phase.completion}%
                              </span>
                            </div>
                            <div className={`w-full ${phase.isMainnet ? 'bg-white/20' : 'bg-slate-200 dark:bg-slate-700'} rounded-full h-2 overflow-hidden`}>
                              <motion.div
                                initial={{ width: 0 }}
                                whileInView={{ width: `${phase.completion}%` }}
                                transition={{ duration: 1.5, delay: phase.delay + 0.3 }}
                                viewport={{ once: true }}
                                className={`h-2 rounded-full bg-gradient-to-r ${phase.gradient}`}
                              />
                            </div>
                          </div>
                        )}

                        <ul className="space-y-3">
                          {phase.items.map((item, idx) => (
                            <motion.li
                              key={item.text}
                              initial={{ opacity: 0, x: -20 }}
                              whileInView={{ opacity: 1, x: 0 }}
                              transition={{ duration: 0.5, delay: phase.delay + 0.2 + idx * 0.1 }}
                              viewport={{ once: true }}
                              className="flex items-center gap-3"
                            >
                              {item.done ? (
                                <CheckCircle2 className={`w-5 h-5 flex-shrink-0 ${phase.isMainnet ? 'text-white' : 'text-green-500'}`} />
                              ) : (
                                <Circle className={`w-5 h-5 flex-shrink-0 ${phase.isMainnet ? 'text-white/70' : 'text-slate-400 dark:text-slate-600'}`} />
                              )}
                              <span className={`${phase.isMainnet ? 'text-white' : 'text-slate-700 dark:text-slate-300'} text-sm md:text-base`}>
                                {item.text}
                              </span>
                            </motion.li>
                          ))}
                        </ul>

                        {phase.isCurrent && (
                          <div className="mt-6">
                            <GradientButton href="#get-started" size="sm">
                              Join Testnet Now
                            </GradientButton>
                          </div>
                        )}

                        {!phase.isMainnet && (
                          <div className={`absolute inset-0 bg-gradient-to-r ${phase.gradient} opacity-0 group-hover:opacity-5 transition-opacity duration-300 pointer-events-none`} />
                        )}
                      </motion.div>
                    </div>
                  </motion.div>
                );
              })}
            </div>
          </div>
        </div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.6 }}
          viewport={{ once: true }}
          className="text-center mt-16 md:mt-20"
        >
          <p className="text-lg text-slate-600 dark:text-slate-400 mb-6">
            Be part of the revolution
          </p>
          <GradientButton href="#get-started" size="lg" icon={Rocket}>
            Get Started Today
          </GradientButton>
        </motion.div>
      </div>
    </section>
  );
}
