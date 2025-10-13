'use client';

import { motion } from 'framer-motion';
import { CheckCircle2, Circle, Rocket, Shield } from 'lucide-react';

export default function Roadmap() {
  const phases = [
    {
      quarter: 'Q4 2025',
      status: 'Completed',
      title: 'Core Development',
      icon: CheckCircle2,
      iconColor: 'text-green-500',
      bgColor: 'bg-green-50 dark:bg-green-900/20',
      borderColor: 'border-green-500',
      items: [
        { text: 'Proof-of-Spiral consensus', done: true },
        { text: 'Post-quantum cryptography', done: true },
        { text: 'AI semantic layer', done: true },
        { text: 'RPC API complete', done: true },
      ],
      delay: 0.1,
    },
    {
      quarter: 'Q1 2026',
      status: 'In Progress',
      title: 'Public Testnet',
      icon: Circle,
      iconColor: 'text-indigo-500 animate-pulse',
      bgColor: 'bg-indigo-50 dark:bg-indigo-900/20',
      borderColor: 'border-indigo-500',
      items: [
        { text: 'Community testing', done: false },
        { text: '100+ validators', done: false },
        { text: 'Bug bounty program', done: false },
        { text: 'Discord & Telegram', done: false },
      ],
      delay: 0.2,
    },
    {
      quarter: 'Q2 2026',
      status: 'Planned',
      title: 'Infrastructure & Security',
      icon: Shield,
      iconColor: 'text-slate-400',
      bgColor: 'bg-slate-50 dark:bg-slate-800/50',
      borderColor: 'border-slate-300 dark:border-slate-700',
      items: [
        { text: 'Multi-region bootstrap nodes', done: false },
        { text: 'Professional security audit', done: false },
        { text: 'Blockchain explorer', done: false },
        { text: 'Mobile wallet', done: false },
      ],
      delay: 0.3,
    },
    {
      quarter: 'Q3 2026',
      status: 'Planned',
      title: 'MAINNET LAUNCH',
      icon: Rocket,
      iconColor: 'text-purple-500',
      bgColor: 'bg-gradient-to-br from-indigo-500 to-purple-600',
      borderColor: 'border-purple-500',
      textColor: 'text-white',
      items: [
        { text: 'Genesis block', done: false },
        { text: 'DEX listing', done: false },
        { text: '500+ validators', done: false },
        { text: '24/7 support', done: false },
      ],
      delay: 0.4,
      isMainnet: true,
    },
  ];

  return (
    <section id="roadmap" className="py-20 bg-white dark:bg-slate-950">
      <div className="container mx-auto px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="text-center mb-16"
        >
          <h2 className="text-5xl md:text-6xl font-black mb-6">
            Roadmap
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            Our journey to revolutionize blockchain technology
          </p>
        </motion.div>

        <div className="max-w-5xl mx-auto space-y-8">
          {phases.map((phase, index) => {
            const Icon = phase.icon;
            return (
              <motion.div
                key={phase.quarter}
                initial={{ opacity: 0, x: index % 2 === 0 ? -50 : 50 }}
                whileInView={{ opacity: 1, x: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: phase.delay }}
                className="flex items-start space-x-6"
              >
                {/* Timeline */}
                <div className="flex-shrink-0 w-32 text-right">
                  <div className="font-bold text-lg">{phase.quarter}</div>
                  <div className={`text-sm ${phase.isMainnet ? 'text-purple-600 font-semibold' : 'text-slate-600 dark:text-slate-400'}`}>
                    {phase.status}
                  </div>
                </div>

                {/* Icon */}
                <div className="flex-shrink-0 mt-1">
                  <div className={`w-12 h-12 rounded-full border-4 ${phase.borderColor} ${phase.bgColor} flex items-center justify-center`}>
                    <Icon className={phase.iconColor} size={24} />
                  </div>
                </div>

                {/* Content */}
                <div className={`flex-1 ${phase.bgColor} ${phase.isMainnet ? phase.textColor : ''} p-8 rounded-2xl shadow-xl border-2 ${phase.borderColor}`}>
                  <h4 className={`font-bold text-2xl mb-4 ${phase.isMainnet ? 'flex items-center space-x-2' : ''}`}>
                    {phase.isMainnet && <Rocket size={24} />}
                    <span>{phase.title}</span>
                  </h4>
                  <ul className="space-y-3">
                    {phase.items.map((item) => (
                      <li key={item.text} className="flex items-center space-x-3">
                        {item.done ? (
                          <CheckCircle2 className={phase.isMainnet ? 'text-white' : 'text-green-500'} size={20} />
                        ) : (
                          <Circle className={phase.isMainnet ? 'text-white/70' : 'text-slate-400'} size={20} />
                        )}
                        <span className={phase.isMainnet ? '' : 'text-slate-700 dark:text-slate-300'}>
                          {item.text}
                        </span>
                      </li>
                    ))}
                  </ul>
                </div>
              </motion.div>
            );
          })}
        </div>
      </div>
    </section>
  );
}

