'use client';

import { motion } from 'framer-motion';
import { DollarSign, Cpu, Gamepad2, Shield, TrendingUp, Zap } from 'lucide-react';

export default function UseCases() {
  const useCases = [
    {
      icon: DollarSign,
      title: 'Decentralized Finance',
      description: 'Build next-generation DeFi applications with quantum-secure transactions, instant settlements, and AI-powered fraud detection.',
      features: ['Secure lending protocols', 'Automated market makers', 'AI risk assessment'],
      gradient: 'from-green-500 to-emerald-500',
      bgImage: 'linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(5, 150, 105, 0.1))',
    },
    {
      icon: Cpu,
      title: 'Internet of Things',
      description: 'Connect billions of IoT devices with low-energy consensus, micro-transactions, and semantic data interpretation.',
      features: ['Machine-to-machine payments', 'Edge computing integration', 'Smart city infrastructure'],
      gradient: 'from-blue-500 to-cyan-500',
      bgImage: 'linear-gradient(135deg, rgba(59, 130, 246, 0.1), rgba(6, 182, 212, 0.1))',
    },
    {
      icon: Shield,
      title: 'Enterprise Solutions',
      description: 'Deploy private or consortium chains with military-grade post-quantum cryptography for sensitive business operations.',
      features: ['Supply chain tracking', 'Document verification', 'Regulatory compliance'],
      gradient: 'from-purple-500 to-indigo-500',
      bgImage: 'linear-gradient(135deg, rgba(168, 85, 247, 0.1), rgba(99, 102, 241, 0.1))',
    },
    {
      icon: Gamepad2,
      title: 'Gaming & Metaverse',
      description: 'Create immersive gaming economies with true digital ownership, fast transactions, and cross-platform asset portability.',
      features: ['NFT gaming assets', 'Play-to-earn mechanisms', 'Virtual world economies'],
      gradient: 'from-pink-500 to-rose-500',
      bgImage: 'linear-gradient(135deg, rgba(236, 72, 153, 0.1), rgba(244, 63, 94, 0.1))',
    },
    {
      icon: TrendingUp,
      title: 'AI & Machine Learning',
      description: 'Leverage our native AI layer for decentralized machine learning models, data marketplaces, and intelligent automation.',
      features: ['Federated learning', 'AI model training', 'Data monetization'],
      gradient: 'from-orange-500 to-yellow-500',
      bgImage: 'linear-gradient(135deg, rgba(249, 115, 22, 0.1), rgba(234, 179, 8, 0.1))',
    },
    {
      icon: Zap,
      title: 'Energy & Sustainability',
      description: 'Revolutionize energy grids with peer-to-peer trading, carbon credit tracking, and transparent renewable energy certificates.',
      features: ['P2P energy trading', 'Carbon offsetting', 'Green certificate tracking'],
      gradient: 'from-teal-500 to-green-500',
      bgImage: 'linear-gradient(135deg, rgba(20, 184, 166, 0.1), rgba(34, 197, 94, 0.1))',
    },
  ];

  return (
    <section className="py-24 md:py-32 relative overflow-hidden section-gradient-dark">
      <div className="absolute inset-0">
        <div className="absolute top-20 left-20 w-96 h-96 bg-purple-600/10 rounded-full blur-3xl animate-pulse" />
        <div className="absolute bottom-20 right-20 w-96 h-96 bg-blue-600/10 rounded-full blur-3xl animate-pulse" style={{ animationDelay: '1s' }} />
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
            className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-purple-600/20 via-pink-600/20 to-blue-600/20 border border-purple-500/30 rounded-full backdrop-blur-sm mb-8"
          >
            <Zap className="w-5 h-5 text-purple-400" />
            <span className="text-sm font-bold text-purple-300">Real-World Applications</span>
          </motion.div>

          <h2 className="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black mb-6">
            <span className="block text-white">
              Built for
            </span>
            <span className="block gradient-text-hero">
              Every Industry
            </span>
          </h2>
          
          <p className="text-lg sm:text-xl md:text-2xl text-gray-300 max-w-3xl mx-auto leading-relaxed">
            From finance to gaming, SpiraChain powers the future of decentralized applications
          </p>
        </motion.div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6 md:gap-8">
          {useCases.map((useCase, index) => (
            <motion.div
              key={useCase.title}
              initial={{ opacity: 0, y: 50, scale: 0.95 }}
              whileInView={{ opacity: 1, y: 0, scale: 1 }}
              transition={{ duration: 0.6, delay: index * 0.1 }}
              viewport={{ once: true }}
              whileHover={{ y: -8, scale: 1.02 }}
              className="group relative"
            >
              <div
                className="relative h-full p-6 md:p-8 glass-card rounded-3xl shadow-2xl hover:shadow-3xl transition-all duration-300 overflow-hidden"
                style={{
                  background: useCase.bgImage,
                }}
              >
                <div className={`inline-flex p-4 rounded-2xl bg-gradient-to-r ${useCase.gradient} shadow-lg mb-6 group-hover:scale-110 transition-transform duration-300`}>
                  <useCase.icon className="w-8 h-8 text-white" />
                </div>

                <h3 className="text-2xl font-black text-white mb-4">
                  {useCase.title}
                </h3>

                <p className="text-gray-300 mb-6 leading-relaxed">
                  {useCase.description}
                </p>

                <ul className="space-y-2">
                  {useCase.features.map((feature, idx) => (
                    <motion.li
                      key={feature}
                      initial={{ opacity: 0, x: -20 }}
                      whileInView={{ opacity: 1, x: 0 }}
                      transition={{ duration: 0.4, delay: index * 0.1 + idx * 0.1 }}
                      viewport={{ once: true }}
                      className="flex items-center gap-2 text-sm text-gray-400"
                    >
                      <div className={`w-1.5 h-1.5 rounded-full bg-gradient-to-r ${useCase.gradient}`} />
                      {feature}
                    </motion.li>
                  ))}
                </ul>

                <div className={`absolute inset-0 bg-gradient-to-r ${useCase.gradient} opacity-0 group-hover:opacity-10 transition-opacity duration-300 pointer-events-none`} />
                <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-br from-white/5 to-transparent rounded-full blur-2xl group-hover:scale-150 transition-transform duration-500" />
              </div>

              <div className={`absolute -inset-1 bg-gradient-to-r ${useCase.gradient} rounded-3xl blur-xl opacity-0 group-hover:opacity-30 transition-opacity duration-300 -z-10`} />
            </motion.div>
          ))}
        </div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.6 }}
          viewport={{ once: true }}
          className="text-center mt-16 md:mt-20"
        >
          <p className="text-lg text-gray-300 mb-8">
            Ready to build the future?
          </p>
          <div className="inline-block p-1 bg-gradient-to-r from-purple-600 to-pink-600 rounded-2xl">
            <motion.a
              href="#get-started"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="block px-8 md:px-12 py-4 md:py-6 bg-slate-900 text-white font-black text-lg md:text-xl rounded-xl hover:bg-slate-800 transition-colors duration-300"
            >
              Start Building Now
            </motion.a>
          </div>
        </motion.div>
      </div>
    </section>
  );
}

