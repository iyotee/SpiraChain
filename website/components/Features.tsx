'use client';

import { motion } from 'framer-motion';
import { Shield, Brain, Infinity, Lock, Zap, TrendingUp } from 'lucide-react';

export default function Features() {
  const features = [
    {
      icon: Shield,
      title: 'Post-Quantum Security',
      description: 'Protected against quantum computers with XMSS signatures and McEliece encryption. Your assets are safe in the quantum era.',
      features: ['XMSS signatures', 'McEliece encryption', 'Kyber-1024 key exchange'],
      color: 'from-red-500 to-pink-600',
      delay: 0.1,
    },
    {
      icon: Brain,
      title: 'AI Semantic Layer',
      description: 'Built-in AI understands transaction meaning, detects fraud, and rewards quality. The first intelligent blockchain.',
      features: ['Natural language processing', 'Fraud detection', 'Pattern recognition'],
      color: 'from-indigo-500 to-purple-600',
      delay: 0.2,
    },
    {
      icon: Infinity,
      title: 'Proof-of-Spiral',
      description: 'Validators create mathematical spirals, not wasteful hashes. Energy-efficient and beautiful.',
      features: ['99.9% less energy than Bitcoin', 'Raspberry Pi compatible', 'Quality over quantity'],
      color: 'from-purple-500 to-pink-600',
      delay: 0.3,
    },
  ];

  return (
    <section id="features" className="py-20 bg-slate-50 dark:bg-slate-900/50">
      <div className="container mx-auto px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="text-center mb-16"
        >
          <h2 className="text-5xl md:text-6xl font-black mb-6">
            Why <span className="gradient-text">SpiraChain</span>?
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            Three critical innovations that make SpiraChain the blockchain of the future
          </p>
        </motion.div>

        <div className="grid md:grid-cols-3 gap-8">
          {features.map((feature, index) => {
            const Icon = feature.icon;
            return (
              <motion.div
                key={feature.title}
                initial={{ opacity: 0, y: 50 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: feature.delay }}
                whileHover={{ y: -10, transition: { duration: 0.2 } }}
                className="bg-white dark:bg-slate-800 p-8 rounded-3xl shadow-2xl border border-slate-100 dark:border-slate-700 group"
              >
                <div className={`w-16 h-16 bg-gradient-to-br ${feature.color} rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300`}>
                  <Icon className="text-white" size={32} />
                </div>
                <h3 className="text-2xl font-bold mb-4">{feature.title}</h3>
                <p className="text-slate-600 dark:text-slate-400 mb-6">
                  {feature.description}
                </p>
                <ul className="space-y-3">
                  {feature.features.map((item) => (
                    <li key={item} className="flex items-center text-sm">
                      <div className="w-5 h-5 bg-green-100 dark:bg-green-900/30 rounded-full flex items-center justify-center mr-3">
                        <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                      </div>
                      <span className="text-slate-700 dark:text-slate-300">{item}</span>
                    </li>
                  ))}
                </ul>
              </motion.div>
            );
          })}
        </div>
      </div>
    </section>
  );
}

