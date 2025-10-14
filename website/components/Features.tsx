'use client';

import { motion } from 'framer-motion';
import { Shield, Brain, Infinity, Cpu, CheckCircle, X } from 'lucide-react';
import FeatureCard from './FeatureCard';
import GradientButton from './GradientButton';

export default function Features() {
  const mainFeatures = [
    {
      icon: Shield,
      title: 'Post-Quantum Security',
      description: 'Protected against quantum computers with XMSS signatures and McEliece encryption. Your assets remain secure in the quantum era with cutting-edge cryptographic algorithms.',
      gradient: 'from-red-500 to-pink-500',
      delay: 0.1
    },
    {
      icon: Brain,
      title: 'AI Semantic Layer',
      description: 'Built-in AI understands transaction meaning, detects fraud patterns, and rewards quality contributions. The first truly intelligent blockchain network.',
      gradient: 'from-blue-500 to-cyan-500',
      delay: 0.2
    },
    {
      icon: Infinity,
      title: 'Proof-of-Spiral',
      description: 'Validators create mathematical spirals, not wasteful hashes. 99.9% less energy than Bitcoin while maintaining security and decentralization.',
      gradient: 'from-purple-500 to-indigo-500',
      delay: 0.3
    }
  ];

  const additionalFeatures = [
    {
      icon: Cpu,
      title: 'Raspberry Pi Ready',
      description: 'Run a full validator on $80 hardware',
      gradient: 'from-green-500 to-emerald-500'
    },
    {
      icon: CheckCircle,
      title: '30s Block Time',
      description: 'Lightning-fast transactions with instant finality',
      gradient: 'from-orange-500 to-yellow-500'
    },
    {
      icon: CheckCircle,
      title: 'True Decentralization',
      description: 'No central authority, peer-to-peer network',
      gradient: 'from-pink-500 to-rose-500'
    }
  ];

  const comparisonData = [
    {
      feature: 'Energy Consumption',
      bitcoin: { value: '150 TWh/year', status: 'bad' },
      ethereum: { value: '78 TWh/year', status: 'medium' },
      spirachain: { value: '0.15 TWh/year', status: 'good' }
    },
    {
      feature: 'Hardware Cost',
      bitcoin: { value: '$10,000+', status: 'bad' },
      ethereum: { value: '$2,000+', status: 'medium' },
      spirachain: { value: '$80', status: 'good' }
    },
    {
      feature: 'Block Time',
      bitcoin: { value: '10 minutes', status: 'bad' },
      ethereum: { value: '12 seconds', status: 'medium' },
      spirachain: { value: '30 seconds', status: 'good' }
    },
    {
      feature: 'Quantum Secure',
      bitcoin: { value: 'No', status: 'bad' },
      ethereum: { value: 'No', status: 'bad' },
      spirachain: { value: 'Yes', status: 'good' }
    },
    {
      feature: 'AI Integration',
      bitcoin: { value: 'No', status: 'bad' },
      ethereum: { value: 'Limited', status: 'medium' },
      spirachain: { value: 'Native', status: 'good' }
    }
  ];

  return (
    <section id="features" className="py-24 md:py-32 relative overflow-hidden section-gradient-light">
      <div className="absolute inset-0 opacity-40 dark:opacity-30">
        <div className="absolute top-20 left-20 w-96 h-96 bg-purple-500/20 dark:bg-purple-500/10 rounded-full blur-3xl" />
        <div className="absolute bottom-20 right-20 w-96 h-96 bg-blue-500/20 dark:bg-blue-500/10 rounded-full blur-3xl" />
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
            className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-purple-600/10 via-pink-600/10 to-blue-600/10 border border-purple-500/20 rounded-full backdrop-blur-sm mb-8"
          >
            <span className="text-sm font-bold text-purple-600 dark:text-purple-400">Why SpiraChain?</span>
          </motion.div>

          <h2 className="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black mb-6">
            <span className="block text-slate-900 dark:text-white">
              Three Revolutionary
            </span>
            <span className="block gradient-text">
              Innovations
            </span>
          </h2>
          
          <p className="text-lg sm:text-xl md:text-2xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto leading-relaxed">
            That make SpiraChain the blockchain of the future
          </p>
        </motion.div>

        <div className="grid lg:grid-cols-3 gap-6 md:gap-8 mb-20 md:mb-24">
          {mainFeatures.map((feature) => (
            <FeatureCard
              key={feature.title}
              icon={feature.icon}
              title={feature.title}
              description={feature.description}
              gradient={feature.gradient}
              delay={feature.delay}
            />
          ))}
        </div>

        <div className="grid md:grid-cols-3 gap-6 md:gap-8 mb-20 md:mb-24">
          {additionalFeatures.map((feature, index) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, y: 30 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: 0.4 + index * 0.1 }}
              viewport={{ once: true }}
              whileHover={{ scale: 1.05, y: -5 }}
              className="p-6 md:p-8 glass-card rounded-2xl shadow-xl hover:shadow-2xl transition-all duration-300 text-center group"
            >
              <div className={`inline-flex p-3 rounded-xl bg-gradient-to-r ${feature.gradient} shadow-lg mb-4 group-hover:scale-110 transition-transform duration-300`}>
                <feature.icon className="w-6 h-6 text-white" />
              </div>
              
              <h4 className="text-xl font-bold text-slate-900 dark:text-white mb-3">
                {feature.title}
              </h4>
              
              <p className="text-slate-600 dark:text-slate-400">
                {feature.description}
              </p>
            </motion.div>
          ))}
        </div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.6 }}
          viewport={{ once: true }}
          className="mb-12"
        >
          <h3 className="text-3xl sm:text-4xl md:text-5xl font-black text-center text-slate-900 dark:text-white mb-12">
            How We Compare
          </h3>

          <div className="glass-card rounded-3xl shadow-2xl p-4 sm:p-6 md:p-8 overflow-x-auto">
            <table className="w-full min-w-[600px]">
              <thead>
                <tr className="border-b-2 border-slate-200 dark:border-slate-700">
                  <th className="text-left py-4 px-2 sm:px-4 text-slate-700 dark:text-slate-300 font-bold">Feature</th>
                  <th className="text-center py-4 px-2 sm:px-4 text-orange-600 dark:text-orange-400 font-bold">Bitcoin</th>
                  <th className="text-center py-4 px-2 sm:px-4 text-blue-600 dark:text-blue-400 font-bold">Ethereum</th>
                  <th className="text-center py-4 px-2 sm:px-4">
                    <span className="bg-gradient-to-r from-purple-600 to-pink-500 bg-clip-text text-transparent font-black">
                      SpiraChain
                    </span>
                  </th>
                </tr>
              </thead>
              <tbody>
                {comparisonData.map((row, index) => (
                  <motion.tr
                    key={row.feature}
                    initial={{ opacity: 0, x: -20 }}
                    whileInView={{ opacity: 1, x: 0 }}
                    transition={{ duration: 0.5, delay: 0.7 + index * 0.1 }}
                    viewport={{ once: true }}
                    className="border-b border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
                  >
                    <td className="py-4 px-2 sm:px-4 font-semibold text-slate-900 dark:text-white">{row.feature}</td>
                    <td className="py-4 px-2 sm:px-4 text-center">
                      <span className="text-slate-600 dark:text-slate-400">{row.bitcoin.value}</span>
                      {row.bitcoin.status === 'bad' && <X className="w-4 h-4 text-red-500 inline ml-2" />}
                    </td>
                    <td className="py-4 px-2 sm:px-4 text-center">
                      <span className="text-slate-600 dark:text-slate-400">{row.ethereum.value}</span>
                      {row.ethereum.status === 'bad' && <X className="w-4 h-4 text-red-500 inline ml-2" />}
                    </td>
                    <td className="py-4 px-2 sm:px-4 text-center">
                      <span className="font-bold bg-gradient-to-r from-purple-600 to-pink-500 bg-clip-text text-transparent">
                        {row.spirachain.value}
                      </span>
                      <CheckCircle className="w-4 h-4 text-green-500 inline ml-2" />
                    </td>
                  </motion.tr>
                ))}
              </tbody>
            </table>
          </div>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.8 }}
          viewport={{ once: true }}
          className="text-center"
        >
          <GradientButton href="#get-started" size="lg">
            Start Building on SpiraChain
          </GradientButton>
        </motion.div>
      </div>
    </section>
  );
}
