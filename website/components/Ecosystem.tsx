'use client';

import { motion } from 'framer-motion';
import { Network, Users, TrendingUp, Globe, Code, Blocks } from 'lucide-react';
import StatsCounter from './StatsCounter';

export default function Ecosystem() {
  const ecosystemStats = [
    {
      icon: Users,
      value: 150,
      suffix: '+',
      label: 'Active Validators',
      description: 'Global network nodes',
      gradient: 'from-blue-500 to-cyan-500',
    },
    {
      icon: TrendingUp,
      value: 2.5,
      suffix: 'M',
      decimals: 1,
      label: 'Transactions',
      description: 'Total processed',
      gradient: 'from-purple-500 to-pink-500',
    },
    {
      icon: Globe,
      value: 35,
      suffix: '+',
      label: 'Countries',
      description: 'Worldwide coverage',
      gradient: 'from-green-500 to-emerald-500',
    },
    {
      icon: Code,
      value: 50,
      suffix: '+',
      label: 'Developers',
      description: 'Building on SpiraChain',
      gradient: 'from-orange-500 to-yellow-500',
    },
  ];

  const integrations = [
    {
      icon: Blocks,
      title: 'Developer Tools',
      description: 'Comprehensive SDKs for Rust, Python, JavaScript, and more',
      items: ['RPC APIs', 'GraphQL endpoints', 'WebSocket support', 'CLI tools'],
      gradient: 'from-indigo-500 to-purple-500',
    },
    {
      icon: Network,
      title: 'Infrastructure',
      description: 'Enterprise-grade infrastructure for validators and developers',
      items: ['Bootstrap nodes', 'RPC endpoints', 'Archive nodes', 'Testnet faucet'],
      gradient: 'from-cyan-500 to-blue-500',
    },
    {
      icon: Globe,
      title: 'Community',
      description: 'Join thousands of developers, validators, and enthusiasts',
      items: ['Discord server', 'Telegram groups', 'GitHub discussions', 'Forum'],
      gradient: 'from-pink-500 to-rose-500',
    },
  ];

  return (
    <section className="py-24 md:py-32 relative overflow-hidden section-gradient-light">
      <div className="absolute inset-0 opacity-40 dark:opacity-30">
        <div className="absolute top-20 right-20 w-96 h-96 bg-blue-500/20 dark:bg-blue-500/10 rounded-full blur-3xl" />
        <div className="absolute bottom-20 left-20 w-96 h-96 bg-purple-500/20 dark:bg-purple-500/10 rounded-full blur-3xl" />
      </div>

      <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8 }}
          viewport={{ once: true }}
          className="text-center mb-16 md:mb-20"
        >
          <motion.div
            initial={{ scale: 0.8, opacity: 0 }}
            whileInView={{ scale: 1, opacity: 1 }}
            transition={{ duration: 0.6, delay: 0.2 }}
            viewport={{ once: true }}
            className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-blue-600/10 via-purple-600/10 to-pink-600/10 border border-blue-500/20 rounded-full backdrop-blur-sm mb-8"
          >
            <Network className="w-5 h-5 text-blue-600 dark:text-blue-400" />
            <span className="text-sm font-bold text-blue-600 dark:text-blue-400">Growing Ecosystem</span>
          </motion.div>

          <h2 className="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black mb-6">
            <span className="block text-slate-900 dark:text-white">
              Join Our
            </span>
            <span className="block gradient-text">
              Thriving Ecosystem
            </span>
          </h2>
          
          <p className="text-lg sm:text-xl md:text-2xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto leading-relaxed">
            A rapidly growing network of validators, developers, and innovators
          </p>
        </motion.div>

        <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6 md:gap-8 mb-16 md:mb-24">
          {ecosystemStats.map((stat, index) => (
            <StatsCounter
              key={stat.label}
              end={stat.value}
              suffix={stat.suffix}
              decimals={stat.decimals || 0}
              gradient={stat.gradient}
              label={stat.label}
              description={stat.description}
              delay={0.2 + index * 0.1}
            />
          ))}
        </div>

        <div className="grid md:grid-cols-3 gap-6 md:gap-8 mb-16 md:mb-20">
          {integrations.map((integration, index) => (
            <motion.div
              key={integration.title}
              initial={{ opacity: 0, y: 50 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.4 + index * 0.1 }}
              viewport={{ once: true }}
              whileHover={{ y: -8, scale: 1.02 }}
              className="group"
            >
              <div className="h-full p-6 md:p-8 glass-card rounded-3xl shadow-xl hover:shadow-2xl transition-all duration-300">
                <div className={`inline-flex p-4 rounded-2xl bg-gradient-to-r ${integration.gradient} shadow-lg mb-6 group-hover:scale-110 transition-transform duration-300`}>
                  <integration.icon className="w-8 h-8 text-white" />
                </div>

                <h3 className="text-2xl font-black text-slate-900 dark:text-white mb-4">
                  {integration.title}
                </h3>

                <p className="text-slate-600 dark:text-slate-400 mb-6 leading-relaxed">
                  {integration.description}
                </p>

                <ul className="space-y-3">
                  {integration.items.map((item, idx) => (
                    <motion.li
                      key={item}
                      initial={{ opacity: 0, x: -20 }}
                      whileInView={{ opacity: 1, x: 0 }}
                      transition={{ duration: 0.4, delay: 0.5 + index * 0.1 + idx * 0.1 }}
                      viewport={{ once: true }}
                      className="flex items-center gap-3"
                    >
                      <div className={`w-2 h-2 rounded-full bg-gradient-to-r ${integration.gradient}`} />
                      <span className="text-sm text-slate-700 dark:text-slate-300 font-medium">
                        {item}
                      </span>
                    </motion.li>
                  ))}
                </ul>

                <div className={`absolute inset-0 bg-gradient-to-r ${integration.gradient} opacity-0 group-hover:opacity-5 rounded-3xl transition-opacity duration-300 pointer-events-none`} />
              </div>
            </motion.div>
          ))}
        </div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.6 }}
          viewport={{ once: true }}
          className="glass-card rounded-3xl p-8 md:p-12 shadow-2xl text-center"
        >
          <h3 className="text-3xl sm:text-4xl font-black text-slate-900 dark:text-white mb-4">
            Want to Integrate with SpiraChain?
          </h3>
          <p className="text-lg text-slate-600 dark:text-slate-400 mb-8 max-w-2xl mx-auto">
            Whether you&apos;re building DeFi, NFTs, or enterprise solutions, our team is here to help you succeed
          </p>
          
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <motion.a
              href="https://github.com/iyotee/SpiraChain"
              target="_blank"
              rel="noopener noreferrer"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="px-8 py-4 bg-gradient-to-r from-purple-600 to-pink-600 text-white font-bold rounded-2xl hover:shadow-xl transition-all duration-300"
            >
              View Documentation
            </motion.a>
            <motion.a
              href="mailto:dev@spirachain.org"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="px-8 py-4 bg-white dark:bg-slate-800 border-2 border-purple-600 text-purple-600 dark:text-purple-400 font-bold rounded-2xl hover:shadow-xl transition-all duration-300"
            >
              Contact Team
            </motion.a>
          </div>
        </motion.div>
      </div>
    </section>
  );
}

