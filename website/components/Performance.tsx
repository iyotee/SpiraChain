'use client';

import { motion } from 'framer-motion';
import { Cpu, Smartphone, Server, Zap, DollarSign, Gauge, Lock } from 'lucide-react';

export default function Performance() {
  const hardwareConfigs = [
    {
      name: 'Raspberry Pi 5',
      icon: Smartphone,
      cpu: 'ARM Cortex-A76',
      spirals: '10,000',
      maxComplexity: '200',
      power: '5W',
      cost: '$80',
      roi: '~150%',
      color: 'from-pink-500 to-rose-600',
      badge: null,
      delay: 0.1,
    },
    {
      name: 'Standard CPU',
      icon: Cpu,
      cpu: 'Intel i5 / AMD Ryzen 5',
      spirals: '50,000',
      maxComplexity: '300',
      power: '65W',
      cost: '$500',
      roi: '~200%',
      color: 'from-indigo-500 to-purple-600',
      badge: 'RECOMMENDED',
      delay: 0.2,
    },
    {
      name: 'GPU Accelerated',
      icon: Server,
      cpu: 'NVIDIA RTX 4090',
      spirals: '500,000',
      maxComplexity: '500+',
      power: '450W',
      cost: '$1,600',
      roi: '~250%',
      color: 'from-purple-500 to-pink-600',
      badge: null,
      delay: 0.3,
    },
  ];

  return (
    <section id="performance" className="py-20 bg-white dark:bg-slate-950">
      <div className="container mx-auto px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="text-center mb-16"
        >
          <h2 className="text-5xl md:text-6xl font-black mb-6">
            Performance
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            Designed for <span className="font-bold text-indigo-600">CPU</span>, <span className="font-bold text-purple-600">GPU</span>, and <span className="font-bold text-pink-600">AI</span>. Optimized for efficiency.
          </p>
        </motion.div>

        <div className="grid md:grid-cols-3 gap-8 max-w-7xl mx-auto mb-16">
          {hardwareConfigs.map((config) => {
            const Icon = config.icon;
            return (
              <motion.div
                key={config.name}
                initial={{ opacity: 0, y: 50 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6, delay: config.delay }}
                className={`relative bg-white dark:bg-slate-800 p-8 rounded-3xl shadow-2xl border-2 ${
                  config.badge ? 'border-indigo-500' : 'border-slate-100 dark:border-slate-700'
                } hover:scale-105 transition-transform duration-300`}
              >
                {config.badge && (
                  <div className="absolute -top-4 left-1/2 transform -translate-x-1/2">
                    <div className="bg-indigo-500 text-white px-4 py-1 rounded-full text-xs font-bold">
                      {config.badge}
                    </div>
                  </div>
                )}

                <div className="text-center mb-6">
                  <div className={`w-16 h-16 bg-gradient-to-br ${config.color} rounded-2xl flex items-center justify-center mx-auto mb-4`}>
                    <Icon className="text-white" size={32} />
                  </div>
                  <h3 className="text-2xl font-bold mb-1">{config.name}</h3>
                  <p className="text-sm text-slate-600 dark:text-slate-400">{config.cpu}</p>
                </div>

                <div className="space-y-4">
                  <div className="flex justify-between items-center">
                    <span className="text-slate-600 dark:text-slate-400">Spirals/sec</span>
                    <span className="font-bold text-lg">{config.spirals}</span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-slate-600 dark:text-slate-400">Max Complexity</span>
                    <span className="font-bold text-lg">{config.maxComplexity}</span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-slate-600 dark:text-slate-400">Power</span>
                    <span className={`font-bold text-lg ${config.power === '5W' ? 'text-green-500' : config.power === '65W' ? 'text-yellow-500' : 'text-red-500'}`}>
                      {config.power}
                    </span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-slate-600 dark:text-slate-400">Cost</span>
                    <span className="font-bold text-lg">{config.cost}</span>
                  </div>
                  <div className="flex justify-between items-center border-t border-slate-200 dark:border-slate-700 pt-4">
                    <span className="text-slate-600 dark:text-slate-400">ROI/year</span>
                    <span className="font-bold text-lg text-green-500">{config.roi}</span>
                  </div>
                </div>
              </motion.div>
            );
          })}
        </div>

        {/* Performance Note */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="max-w-4xl mx-auto"
        >
          <div className="bg-gradient-to-r from-indigo-50 to-purple-50 dark:from-indigo-900/20 dark:to-purple-900/20 p-8 rounded-3xl border-2 border-indigo-200 dark:border-indigo-800">
            <div className="flex items-start space-x-4">
              <div className="w-12 h-12 bg-indigo-500 rounded-xl flex items-center justify-center flex-shrink-0">
                <Zap className="text-white" size={24} />
              </div>
              <div>
                <h4 className="font-bold text-xl mb-2">Difficulty Cap = Raspberry Pi Forever</h4>
                <p className="text-slate-700 dark:text-slate-300 leading-relaxed">
                  SpiraChain caps difficulty at <span className="font-bold text-indigo-600">250</span> to ensure Raspberry Pi validators remain viable forever. 
                  Even in 2149, you&apos;ll be able to validate with a $80 device. No arms race, no centralization.
                </p>
                <div className="mt-4 flex items-center space-x-6 text-sm">
                  <div className="flex items-center space-x-2">
                    <Gauge className="text-green-500" size={16} />
                    <span className="font-semibold">Current: 50</span>
                  </div>
                  <div className="flex items-center space-x-2">
                    <Gauge className="text-yellow-500" size={16} />
                    <span className="font-semibold">Target: 100-200</span>
                  </div>
                  <div className="flex items-center space-x-2">
                    <Lock className="text-red-500" size={16} />
                    <span className="font-semibold">Max: 250</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </motion.div>
      </div>
    </section>
  );
}

