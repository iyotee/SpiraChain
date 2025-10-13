'use client';

import { motion } from 'framer-motion';
import { Shield, Brain, Infinity, Zap, Cpu, Network, Sparkles, ArrowRight, CheckCircle } from 'lucide-react';

export default function Features() {
  const mainFeatures = [
    {
      icon: Shield,
      title: 'Post-Quantum Security',
      description: 'Protected against quantum computers with XMSS signatures and McEliece encryption. Your assets are safe in the quantum era.',
      features: ['XMSS signatures', 'McEliece encryption', 'Kyber-1024 key exchange'],
      color: 'from-red-500 to-pink-500',
      bgColor: 'from-red-500/10 to-pink-500/10',
      borderColor: 'border-red-500/20',
      delay: 0.1
    },
    {
      icon: Brain,
      title: 'AI Semantic Layer',
      description: 'Built-in AI understands transaction meaning, detects fraud, and rewards quality. The first intelligent blockchain.',
      features: ['Natural language processing', 'Fraud detection', 'Pattern recognition'],
      color: 'from-blue-500 to-cyan-500',
      bgColor: 'from-blue-500/10 to-cyan-500/10',
      borderColor: 'border-blue-500/20',
      delay: 0.2
    },
    {
      icon: Infinity,
      title: 'Proof-of-Spiral',
      description: 'Validators create mathematical spirals, not wasteful hashes. Energy-efficient and beautiful.',
      features: ['99.9% less energy than Bitcoin', 'Raspberry Pi compatible', 'Quality over quantity'],
      color: 'from-purple-500 to-indigo-500',
      bgColor: 'from-purple-500/10 to-indigo-500/10',
      borderColor: 'border-purple-500/20',
      delay: 0.3
    }
  ];

  const additionalFeatures = [
    {
      icon: Cpu,
      title: 'Raspberry Pi Optimized',
      description: 'Run a full validator node on a $80 Raspberry Pi',
      color: 'from-green-500 to-emerald-500'
    },
    {
      icon: Network,
      title: 'Decentralized Network',
      description: 'No central authority, truly peer-to-peer',
      color: 'from-orange-500 to-yellow-500'
    },
    {
      icon: Zap,
      title: 'Lightning Fast',
      description: '30-second block times with instant finality',
      color: 'from-pink-500 to-rose-500'
    }
  ];

  return (
    <section className="py-32 relative overflow-hidden bg-gradient-to-br from-slate-50 to-blue-50 dark:from-slate-900 dark:to-slate-800">
      {/* Background Elements */}
      <div className="absolute inset-0">
        <div className="absolute top-20 left-20 w-96 h-96 bg-purple-500/5 rounded-full blur-3xl" />
        <div className="absolute bottom-20 right-20 w-96 h-96 bg-blue-500/5 rounded-full blur-3xl" />
        <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-gradient-to-r from-purple-500/5 to-blue-500/5 rounded-full blur-3xl" />
      </div>

      <div className="container mx-auto px-6 relative z-10">
        {/* Section Header */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8 }}
          viewport={{ once: true }}
          className="text-center mb-20"
        >
          <motion.div
            initial={{ scale: 0.8, opacity: 0 }}
            whileInView={{ scale: 1, opacity: 1 }}
            transition={{ duration: 0.6, delay: 0.2 }}
            viewport={{ once: true }}
            className="inline-block mb-6"
          >
            <div className="inline-flex items-center space-x-2 bg-gradient-to-r from-purple-600/10 to-blue-600/10 px-6 py-3 rounded-full border border-purple-500/20 backdrop-blur-sm">
              <Sparkles className="w-5 h-5 text-purple-600" />
              <span className="text-sm font-bold text-purple-600 dark:text-purple-400">Why SpiraChain?</span>
            </div>
          </motion.div>

          <h2 className="text-6xl md:text-8xl font-black mb-8">
            <span className="bg-gradient-to-r from-slate-900 to-slate-700 dark:from-white dark:to-gray-300 bg-clip-text text-transparent">
              Three Critical
            </span>
            <br />
            <span className="bg-gradient-to-r from-purple-600 via-pink-600 to-blue-600 bg-clip-text text-transparent">
              Innovations
            </span>
          </h2>
          
          <p className="text-2xl md:text-3xl text-slate-600 dark:text-slate-400 max-w-4xl mx-auto leading-relaxed">
            That make SpiraChain the blockchain of the future
          </p>
        </motion.div>

        {/* Main Features Grid */}
        <div className="grid lg:grid-cols-3 gap-8 mb-24">
          {mainFeatures.map((feature) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, y: 50 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.8, delay: feature.delay }}
              viewport={{ once: true }}
              whileHover={{ scale: 1.02, y: -10 }}
              className="group relative"
            >
              <div className={`relative p-10 bg-white/80 dark:bg-slate-800/80 backdrop-blur-sm border-2 ${feature.borderColor} rounded-3xl shadow-2xl hover:shadow-3xl transition-all duration-300 h-full`}>
                {/* Icon */}
                <div className={`inline-flex p-4 rounded-2xl bg-gradient-to-r ${feature.color} shadow-lg mb-8`}>
                  <feature.icon className="w-10 h-10 text-white" />
                </div>

                {/* Content */}
                <h3 className="text-3xl font-black text-slate-900 dark:text-white mb-6">
                  {feature.title}
                </h3>
                
                <p className="text-lg text-slate-600 dark:text-slate-400 mb-8 leading-relaxed">
                  {feature.description}
                </p>

                {/* Features List */}
                <ul className="space-y-4 mb-8">
                  {feature.features.map((item, idx) => (
                    <motion.li
                      key={item}
                      initial={{ opacity: 0, x: -20 }}
                      whileInView={{ opacity: 1, x: 0 }}
                      transition={{ duration: 0.5, delay: feature.delay + 0.1 + idx * 0.1 }}
                      viewport={{ once: true }}
                      className="flex items-center space-x-3"
                    >
                      <CheckCircle className={`w-6 h-6 text-transparent bg-gradient-to-r ${feature.color} bg-clip-text`} />
                      <span className="text-slate-700 dark:text-slate-300 font-medium">{item}</span>
                    </motion.li>
                  ))}
                </ul>

                {/* Learn More Button */}
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className={`inline-flex items-center space-x-2 px-6 py-3 bg-gradient-to-r ${feature.color} text-white font-bold rounded-xl hover:shadow-lg transition-all duration-300`}
                >
                  <span>Learn More</span>
                  <ArrowRight className="w-5 h-5" />
                </motion.button>

                {/* Background Gradient */}
                <div className={`absolute inset-0 bg-gradient-to-r ${feature.bgColor} opacity-0 group-hover:opacity-100 rounded-3xl transition-opacity duration-300 -z-10`} />
              </div>
            </motion.div>
          ))}
        </div>

        {/* Additional Features */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.4 }}
          viewport={{ once: true }}
          className="text-center mb-16"
        >
          <h3 className="text-4xl md:text-5xl font-black text-slate-900 dark:text-white mb-8">
            And Much More
          </h3>
        </motion.div>

        <div className="grid md:grid-cols-3 gap-8">
          {additionalFeatures.map((feature, index) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, scale: 0.8 }}
              whileInView={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.6, delay: 0.5 + index * 0.1 }}
              viewport={{ once: true }}
              whileHover={{ scale: 1.05, y: -5 }}
              className="group"
            >
              <div className="p-8 bg-white/60 dark:bg-slate-800/60 backdrop-blur-sm border border-slate-200/50 dark:border-slate-700/50 rounded-2xl shadow-xl hover:shadow-2xl transition-all duration-300 text-center">
                <div className={`inline-flex p-3 rounded-xl bg-gradient-to-r ${feature.color} shadow-lg mb-6`}>
                  <feature.icon className="w-8 h-8 text-white" />
                </div>
                
                <h4 className="text-2xl font-bold text-slate-900 dark:text-white mb-4">
                  {feature.title}
                </h4>
                
                <p className="text-slate-600 dark:text-slate-400">
                  {feature.description}
                </p>
              </div>
            </motion.div>
          ))}
        </div>

        {/* Call to Action */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.6 }}
          viewport={{ once: true }}
          className="text-center mt-20"
        >
          <div className="inline-block p-1 bg-gradient-to-r from-purple-600 to-blue-600 rounded-2xl">
            <motion.a
              href="#get-started"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="block px-12 py-6 bg-white dark:bg-slate-900 text-slate-900 dark:text-white font-black text-xl rounded-xl hover:bg-gray-50 dark:hover:bg-slate-800 transition-colors duration-300"
            >
              Start Building on SpiraChain
            </motion.a>
          </div>
        </motion.div>
      </div>
    </section>
  );
}