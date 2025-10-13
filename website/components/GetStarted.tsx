'use client';

import { motion } from 'framer-motion';
import { Terminal, Copy, Check } from 'lucide-react';
import { useState } from 'react';

export default function GetStarted() {
  const [copiedLinux, setCopiedLinux] = useState(false);
  const [copiedWindows, setCopiedWindows] = useState(false);

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
      command: 'git clone https://github.com/iyotee/SpiraChain.git',
    },
    {
      number: 2,
      title: 'Build',
      command: 'cargo build --release',
    },
    {
      number: 3,
      title: 'Create Wallet',
      command: './target/release/spira wallet new --output wallet.json',
    },
    {
      number: 4,
      title: 'Start Validator',
      command: './target/release/spira node --validator --wallet wallet.json',
    },
  ];

  return (
    <section id="get-started" className="py-20 bg-slate-50 dark:bg-slate-900/50">
      <div className="container mx-auto px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="text-center mb-16"
        >
          <h2 className="text-5xl md:text-6xl font-black mb-6">
            Get Started
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            Become a validator in one line
          </p>
        </motion.div>

        {/* One-Line Install */}
        <div className="max-w-5xl mx-auto grid md:grid-cols-2 gap-8 mb-16">
          {/* Linux/macOS */}
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="bg-slate-900 p-8 rounded-3xl shadow-2xl"
          >
            <div className="flex items-center space-x-3 mb-6">
              <Terminal className="text-green-400" size={32} />
              <h3 className="text-2xl font-bold text-white">Linux / macOS</h3>
            </div>
            <div className="bg-slate-800 p-4 rounded-xl mb-4 font-mono text-sm text-green-400 overflow-x-auto">
              {linuxCommand}
            </div>
            <button
              onClick={() => copyToClipboard(linuxCommand, 'linux')}
              className="w-full bg-indigo-500 hover:bg-indigo-600 text-white py-3 rounded-xl font-semibold transition-all hover:scale-105 flex items-center justify-center space-x-2"
            >
              {copiedLinux ? <Check size={20} /> : <Copy size={20} />}
              <span>{copiedLinux ? 'Copied!' : 'Copy Command'}</span>
            </button>
          </motion.div>

          {/* Windows */}
          <motion.div
            initial={{ opacity: 0, x: 50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="bg-slate-900 p-8 rounded-3xl shadow-2xl"
          >
            <div className="flex items-center space-x-3 mb-6">
              <Terminal className="text-blue-400" size={32} />
              <h3 className="text-2xl font-bold text-white">Windows</h3>
            </div>
            <div className="bg-slate-800 p-4 rounded-xl mb-4 font-mono text-sm text-blue-400 overflow-x-auto">
              {windowsCommand}
            </div>
            <button
              onClick={() => copyToClipboard(windowsCommand, 'windows')}
              className="w-full bg-indigo-500 hover:bg-indigo-600 text-white py-3 rounded-xl font-semibold transition-all hover:scale-105 flex items-center justify-center space-x-2"
            >
              {copiedWindows ? <Check size={20} /> : <Copy size={20} />}
              <span>{copiedWindows ? 'Copied!' : 'Copy Command'}</span>
            </button>
          </motion.div>
        </div>

        {/* Manual Installation */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="max-w-5xl mx-auto"
        >
          <div className="bg-white dark:bg-slate-800 p-8 rounded-3xl shadow-2xl border border-slate-100 dark:border-slate-700">
            <h3 className="text-3xl font-bold mb-8">Manual Installation</h3>

            <div className="space-y-6">
              {manualSteps.map((step, index) => (
                <motion.div
                  key={step.number}
                  initial={{ opacity: 0, x: -20 }}
                  whileInView={{ opacity: 1, x: 0 }}
                  viewport={{ once: true }}
                  transition={{ duration: 0.4, delay: index * 0.1 }}
                >
                  <div className="flex items-center mb-3">
                    <div className="w-10 h-10 bg-indigo-500 rounded-full flex items-center justify-center text-white font-bold mr-4">
                      {step.number}
                    </div>
                    <h4 className="font-bold text-lg">{step.title}</h4>
                  </div>
                  <div className="bg-slate-100 dark:bg-slate-900 p-4 rounded-xl ml-14 font-mono text-sm overflow-x-auto">
                    {step.command}
                  </div>
                </motion.div>
              ))}
            </div>
          </div>
        </motion.div>
      </div>
    </section>
  );
}

