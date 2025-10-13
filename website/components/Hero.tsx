'use client';

import React from 'react';
import { motion } from 'framer-motion';
import { ArrowRight, Github, Shield, Brain, Sparkles, Infinity } from 'lucide-react';

export default function Hero() {
  const FeatureIcon = ({ icon, className }: { icon: React.ComponentType<{className?: string}>, className: string }) => {
    const IconComponent = icon;
    return <IconComponent className={className} />;
  };

  const features = [
    { name: 'Post-Quantum Security', icon: Shield, color: 'from-red-500 to-pink-500', description: 'XMSS & McEliece' },
    { name: 'AI Semantic Layer', icon: Brain, color: 'from-blue-500 to-cyan-500', description: 'Smart Transactions' },
    { name: 'Proof-of-Spiral', icon: Infinity, color: 'from-purple-500 to-indigo-500', description: 'Energy Efficient' }
  ];

  const stats = [
    { value: '21M', label: 'Total Supply', color: 'from-yellow-400 to-orange-500' },
    { value: '99.9%', label: 'Less Energy', color: 'from-green-400 to-emerald-500' },
    { value: '30s', label: 'Block Time', color: 'from-blue-400 to-cyan-500' },
    { value: '$80', label: 'RPi Validator', color: 'from-purple-400 to-pink-500' }
  ];

  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
      {/* Animated background elements */}
      <div className="absolute inset-0">
        <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-purple-500/30 rounded-full blur-3xl animate-pulse" />
        <div className="absolute bottom-1/4 right-1/4 w-[500px] h-[500px] bg-blue-500/30 rounded-full blur-3xl animate-pulse delay-1000" />
        <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[1000px] h-[1000px] bg-gradient-to-r from-purple-600/20 to-blue-600/20 rounded-full blur-3xl animate-spin" style={{animationDuration: '40s'}} />
        
        {/* Floating particles */}
        {[...Array(30)].map((_, i) => (
          <motion.div
            key={i}
            className="absolute w-1 h-1 bg-white/30 rounded-full"
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
            }}
            animate={{
              y: [0, -30, 0],
              opacity: [0.1, 0.8, 0.1],
            }}
            transition={{
              duration: 4 + Math.random() * 3,
              repeat: -1,
              delay: Math.random() * 3,
            }}
          />
        ))}
      </div>

      <div className="relative z-10 text-center px-6 max-w-7xl mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 50 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 1, ease: "easeOut" }}
          className="mb-12"
        >
          <motion.div
            initial={{ scale: 0.8, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            transition={{ duration: 0.8, delay: 0.2 }}
            className="inline-block px-6 py-3 bg-gradient-to-r from-purple-600/20 to-blue-600/20 border border-purple-500/30 rounded-full text-purple-300 text-lg font-medium mb-8 backdrop-blur-sm"
          >
            <Sparkles className="inline w-5 h-5 mr-2" />
            Testnet Live - Join the Revolution
          </motion.div>

          <h1 className="text-7xl md:text-9xl lg:text-[12rem] font-black mb-8 leading-[0.8]">
            <motion.span 
              className="block text-white mb-4"
              initial={{ opacity: 0, x: -100 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ duration: 1, delay: 0.4 }}
            >
              The World&apos;s First
            </motion.span>
            <motion.span 
              className="block bg-gradient-to-r from-purple-400 via-pink-500 to-red-400 bg-clip-text text-transparent mb-4"
              initial={{ opacity: 0, x: 100 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ duration: 1, delay: 0.6 }}
            >
              Post-Quantum
            </motion.span>
            <motion.span 
              className="block bg-gradient-to-r from-blue-400 via-cyan-500 to-green-400 bg-clip-text text-transparent"
              initial={{ opacity: 0, x: -100 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ duration: 1, delay: 0.8 }}
            >
              Semantic Blockchain
            </motion.span>
          </h1>
          
          <motion.p 
            className="text-2xl md:text-4xl text-gray-300 mb-12 max-w-6xl mx-auto leading-relaxed"
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 1 }}
          >
            <span className="text-white font-bold">Powered by AI</span>, secured by mathematics, accessible to everyone.
            <br />
            <span className="text-purple-400 font-black text-4xl md:text-5xl">Validate with a Raspberry Pi.</span>
            <br />
            <span className="text-blue-400 font-black text-4xl md:text-5xl">Earn rewards for quality, not quantity.</span>
          </motion.p>
        </motion.div>

        {/* Feature highlights with icons */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 1.2 }}
          className="flex flex-wrap justify-center gap-8 mb-20"
        >
          {features.map((feature, index) => (
            <motion.div
              key={feature.name}
              initial={{ opacity: 0, scale: 0.8, y: 20 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 1.4 + index * 0.2 }}
              whileHover={{ scale: 1.05, y: -10 }}
              className="group relative"
            >
              <div className="px-10 py-6 bg-white/10 backdrop-blur-md border border-white/20 rounded-3xl text-white font-bold hover:bg-white/20 transition-all duration-300 shadow-2xl hover:shadow-3xl flex flex-col items-center gap-4 min-w-[320px]">
                <div className={`p-4 rounded-2xl bg-gradient-to-r ${feature.color} shadow-lg`}>
                  <FeatureIcon icon={feature.icon} className="w-8 h-8 text-white" />
                </div>
                <div className="text-xl">{feature.name}</div>
                <div className="text-sm text-gray-300 font-normal">{feature.description}</div>
              </div>
              <div className={`absolute inset-0 bg-gradient-to-r ${feature.color} opacity-0 group-hover:opacity-20 rounded-3xl blur-xl transition-opacity duration-300 -z-10`} />
            </motion.div>
          ))}
        </motion.div>

        {/* CTA Buttons */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 1.8 }}
          className="flex flex-col sm:flex-row gap-8 justify-center items-center mb-24"
        >
          <motion.a
            href="#get-started"
            whileHover={{ scale: 1.05, y: -3 }}
            whileTap={{ scale: 0.95 }}
            className="group relative flex items-center gap-4 px-16 py-8 bg-gradient-to-r from-purple-600 via-pink-600 to-blue-600 text-white font-black text-2xl rounded-3xl hover:from-purple-700 hover:via-pink-700 hover:to-blue-700 transition-all duration-300 shadow-2xl hover:shadow-purple-500/25"
          >
            <div className="absolute inset-0 bg-gradient-to-r from-purple-600 via-pink-600 to-blue-600 rounded-3xl blur opacity-0 group-hover:opacity-50 transition-opacity duration-300 -z-10" />
            <Sparkles className="w-8 h-8" />
            Get Started
            <ArrowRight className="w-8 h-8 group-hover:translate-x-2 transition-transform duration-300" />
          </motion.a>
          
          <motion.a
            href="https://github.com/iyotee/SpiraChain"
            target="_blank"
            rel="noopener noreferrer"
            whileHover={{ scale: 1.05, y: -3 }}
            whileTap={{ scale: 0.95 }}
            className="group flex items-center gap-4 px-16 py-8 bg-white/10 backdrop-blur-md border-2 border-white/30 text-white font-black text-2xl rounded-3xl hover:bg-white/20 hover:border-white/50 transition-all duration-300 shadow-2xl"
          >
            <Github className="w-8 h-8" />
            View on GitHub
          </motion.a>
        </motion.div>

        {/* Enhanced Stats */}
        <motion.div
          initial={{ opacity: 0, y: 50 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 2 }}
          className="grid grid-cols-2 md:grid-cols-4 gap-8 max-w-6xl mx-auto"
        >
          {stats.map((stat, index) => (
            <motion.div
              key={stat.label}
              initial={{ opacity: 0, scale: 0.8 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.6, delay: 2.2 + index * 0.1 }}
              whileHover={{ scale: 1.05, y: -8 }}
              className="text-center group cursor-pointer"
            >
              <div className="relative p-8 bg-white/5 backdrop-blur-sm border border-white/10 rounded-3xl hover:bg-white/10 transition-all duration-300 shadow-xl hover:shadow-2xl">
                <div className={`text-5xl md:text-7xl font-black bg-gradient-to-r ${stat.color} bg-clip-text text-transparent mb-3 group-hover:scale-110 transition-transform duration-300`}>
                  {stat.value}
                </div>
                <div className="text-gray-400 font-semibold text-xl">{stat.label}</div>
                <div className={`absolute inset-0 bg-gradient-to-r ${stat.color} opacity-0 group-hover:opacity-10 rounded-3xl transition-opacity duration-300`} />
              </div>
            </motion.div>
          ))}
        </motion.div>
      </div>

      {/* Enhanced scroll indicator */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 1, delay: 2.5 }}
        className="absolute bottom-12 left-1/2 transform -translate-x-1/2"
      >
        <motion.div
          animate={{ y: [0, 12, 0] }}
          transition={{ duration: 2, repeat: -1 }}
          className="w-10 h-16 border-2 border-white/40 rounded-full flex justify-center cursor-pointer hover:border-white/60 transition-colors duration-300"
        >
          <motion.div
            animate={{ y: [0, 20, 0] }}
            transition={{ duration: 2, repeat: -1 }}
            className="w-2 h-6 bg-gradient-to-b from-purple-400 to-blue-400 rounded-full mt-4"
          />
        </motion.div>
      </motion.div>
    </section>
  );
}