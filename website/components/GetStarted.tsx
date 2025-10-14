'use client';

import { motion } from 'framer-motion';
import { Terminal, Copy, Check, Download, Rocket, BookOpen } from 'lucide-react';
import { useState } from 'react';
import GradientButton from './GradientButton';

export default function GetStarted() {
  const [copiedLinux, setCopiedLinux] = useState(false);
  const [copiedWindows, setCopiedWindows] = useState(false);
  const [activeTab, setActiveTab] = useState<'quickstart' | 'manual'>('quickstart');

  const linuxCommand = 'curl -sSL https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install_validator.sh | bash';
  const windowsCommand = 'iwr -useb https://raw.githubusercontent.com/iyotee/SpiraChain/main/scripts/install_validator.ps1 | iex';

  const copyToClipboard = (text: string, type: 'linux' | 'windows') => {
    navigator.clipboard.writeText(text);
    if (type === 'linux') {
      setCopiedLinux(true);
      setTimeout(() => setCopiedLinux(false), 2000);
    } else {
      setCopiedWindows(true);
      setTimeout(() => setCopiedWindows(false), 2000);
    }
  };

  const manualSteps = [
    {
      number: 1,
      title: 'Clone Repository',
      command: 'git clone https://github.com/iyotee/SpiraChain.git && cd SpiraChain',
      description: 'Download the latest SpiraChain source code'
    },
    {
      number: 2,
      title: 'Build',
      command: 'cargo build --release',
      description: 'Compile the validator node (takes 5-10 minutes)'
    },
    {
      number: 3,
      title: 'Create Wallet',
      command: './target/release/spira wallet new --output wallet.json',
      description: 'Generate your validator wallet and keys'
    },
    {
      number: 4,
      title: 'Start Validator',
      command: './target/release/spira node --validator --wallet wallet.json',
      description: 'Launch your validator and start earning rewards'
    },
  ];

  const quickGuide = [
    { step: '1', title: 'Install', time: '30s' },
    { step: '2', title: 'Configure', time: '15s' },
    { step: '3', title: 'Start Earning', time: '15s' },
  ];

  return (
    <section id="get-started" className="py-24 md:py-32 relative overflow-hidden section-gradient-dark">
      <div className="absolute inset-0">
        <div className="absolute top-20 left-20 w-96 h-96 bg-green-600/10 rounded-full blur-3xl animate-pulse" />
        <div className="absolute bottom-20 right-20 w-96 h-96 bg-blue-600/10 rounded-full blur-3xl animate-pulse" style={{ animationDelay: '1s' }} />
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
            className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-green-600/20 via-emerald-600/20 to-teal-600/20 border border-green-500/30 rounded-full backdrop-blur-sm mb-8"
          >
            <Rocket className="w-5 h-5 text-green-400 animate-pulse" />
            <span className="text-sm font-bold text-green-300">Setup in 60 Seconds</span>
          </motion.div>

          <h2 className="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black mb-6">
            <span className="block text-white">
              Get Started
            </span>
            <span className="block gradient-text-blue">
              In Minutes
            </span>
          </h2>
          <p className="text-lg sm:text-xl md:text-2xl text-gray-300 max-w-3xl mx-auto">
            Become a validator with a single command
          </p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6, delay: 0.2 }}
          className="max-w-4xl mx-auto mb-12"
        >
          <div className="grid grid-cols-3 gap-4 md:gap-8">
            {quickGuide.map((item, index) => (
              <motion.div
                key={item.step}
                initial={{ opacity: 0, scale: 0.8 }}
                whileInView={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.5, delay: 0.3 + index * 0.1 }}
                viewport={{ once: true }}
                className="text-center"
              >
                <div className="w-12 h-12 md:w-16 md:h-16 bg-gradient-to-r from-green-500 to-emerald-500 rounded-2xl flex items-center justify-center text-white font-black text-xl md:text-2xl mx-auto mb-3 shadow-lg">
                  {item.step}
                </div>
                <div className="text-white font-bold text-sm md:text-base mb-1">{item.title}</div>
                <div className="text-gray-400 text-xs md:text-sm">{item.time}</div>
              </motion.div>
            ))}
          </div>
        </motion.div>

        <div className="max-w-6xl mx-auto mb-12">
          <div className="flex justify-center gap-4 mb-8">
            <button
              onClick={() => setActiveTab('quickstart')}
              className={`px-6 py-3 rounded-xl font-bold transition-all duration-300 ${
                activeTab === 'quickstart'
                  ? 'bg-gradient-to-r from-green-600 to-emerald-600 text-white shadow-lg'
                  : 'bg-white/10 text-gray-400 hover:bg-white/20'
              }`}
            >
              Quick Start
            </button>
            <button
              onClick={() => setActiveTab('manual')}
              className={`px-6 py-3 rounded-xl font-bold transition-all duration-300 ${
                activeTab === 'manual'
                  ? 'bg-gradient-to-r from-green-600 to-emerald-600 text-white shadow-lg'
                  : 'bg-white/10 text-gray-400 hover:bg-white/20'
              }`}
            >
              Manual Install
            </button>
          </div>

          {activeTab === 'quickstart' && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5 }}
              className="grid md:grid-cols-2 gap-6 md:gap-8"
            >
              <motion.div
                initial={{ opacity: 0, x: -50 }}
                whileInView={{ opacity: 1, x: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6 }}
                className="glass-card p-6 md:p-8 rounded-3xl shadow-2xl"
              >
                <div className="flex items-center gap-3 mb-6">
                  <Terminal className="text-green-400 w-8 h-8" />
                  <h3 className="text-xl md:text-2xl font-bold text-white">Linux / macOS</h3>
                </div>
                <div className="bg-slate-900/90 p-4 rounded-xl mb-4 font-mono text-xs md:text-sm text-green-400 overflow-x-auto border border-green-500/30">
                  {linuxCommand}
                </div>
                <motion.button
                  onClick={() => copyToClipboard(linuxCommand, 'linux')}
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                  className="w-full bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 text-white py-3 md:py-4 rounded-xl font-bold transition-all shadow-lg flex items-center justify-center gap-2"
                >
                  {copiedLinux ? <Check className="w-5 h-5" /> : <Copy className="w-5 h-5" />}
                  <span>{copiedLinux ? 'Copied!' : 'Copy Command'}</span>
                </motion.button>
              </motion.div>

              <motion.div
                initial={{ opacity: 0, x: 50 }}
                whileInView={{ opacity: 1, x: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6 }}
                className="glass-card p-6 md:p-8 rounded-3xl shadow-2xl"
              >
                <div className="flex items-center gap-3 mb-6">
                  <Terminal className="text-blue-400 w-8 h-8" />
                  <h3 className="text-xl md:text-2xl font-bold text-white">Windows</h3>
                </div>
                <div className="bg-slate-900/90 p-4 rounded-xl mb-4 font-mono text-xs md:text-sm text-blue-400 overflow-x-auto border border-blue-500/30">
                  {windowsCommand}
                </div>
                <motion.button
                  onClick={() => copyToClipboard(windowsCommand, 'windows')}
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                  className="w-full bg-gradient-to-r from-blue-600 to-cyan-600 hover:from-blue-700 hover:to-cyan-700 text-white py-3 md:py-4 rounded-xl font-bold transition-all shadow-lg flex items-center justify-center gap-2"
                >
                  {copiedWindows ? <Check className="w-5 h-5" /> : <Copy className="w-5 h-5" />}
                  <span>{copiedWindows ? 'Copied!' : 'Copy Command'}</span>
                </motion.button>
              </motion.div>
            </motion.div>
          )}

          {activeTab === 'manual' && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5 }}
              className="glass-card p-6 md:p-10 rounded-3xl shadow-2xl"
            >
              <h3 className="text-2xl md:text-3xl font-bold text-white mb-8 flex items-center gap-3">
                <BookOpen className="w-8 h-8" />
                Manual Installation
              </h3>

              <div className="space-y-6">
                {manualSteps.map((step, index) => (
                  <motion.div
                    key={step.number}
                    initial={{ opacity: 0, x: -20 }}
                    whileInView={{ opacity: 1, x: 0 }}
                    viewport={{ once: true }}
                    transition={{ duration: 0.4, delay: index * 0.1 }}
                    className="relative"
                  >
                    <div className="flex items-start gap-4 mb-3">
                      <div className="w-10 h-10 md:w-12 md:h-12 bg-gradient-to-r from-green-600 to-emerald-600 rounded-xl flex items-center justify-center text-white font-bold text-lg flex-shrink-0 shadow-lg">
                        {step.number}
                      </div>
                      <div className="flex-1">
                        <h4 className="font-bold text-lg md:text-xl text-white mb-1">{step.title}</h4>
                        <p className="text-sm text-gray-400">{step.description}</p>
                      </div>
                    </div>
                    <div className="bg-slate-900/90 p-4 rounded-xl ml-14 md:ml-16 font-mono text-xs md:text-sm text-gray-300 overflow-x-auto border border-gray-700">
                      {step.command}
                    </div>
                  </motion.div>
                ))}
              </div>
            </motion.div>
          )}
        </div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6, delay: 0.4 }}
          className="text-center"
        >
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <GradientButton href="https://github.com/iyotee/SpiraChain" icon={Download} size="lg">
              Download Latest Release
            </GradientButton>
            <GradientButton href="https://github.com/iyotee/SpiraChain" variant="outline" icon={BookOpen} size="lg">
              View Documentation
            </GradientButton>
          </div>
        </motion.div>
      </div>
    </section>
  );
}
