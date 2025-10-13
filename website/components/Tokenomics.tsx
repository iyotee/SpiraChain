'use client';

import { motion } from 'framer-motion';
import { Coins, Flame, TrendingDown, Lock, Zap, Users } from 'lucide-react';

export default function Tokenomics() {
  const tokenInfo = [
    { label: 'Total Supply', value: '21,000,000 QBT', icon: Coins },
    { label: 'Block Time', value: '30 seconds', icon: Zap },
    { label: 'Block Reward', value: '10-20 QBT', icon: TrendingDown },
    { label: 'Halving Period', value: '~2 years', icon: TrendingDown },
    { label: 'Fee Burning', value: '30%', icon: Flame },
    { label: 'Validator Stake', value: '10,000 QBT', icon: Lock },
  ];

  const feeDistribution = [
    { label: 'Validator', percentage: 50, color: 'from-green-500 to-emerald-600' },
    { label: 'Burned Forever', percentage: 30, color: 'from-red-500 to-orange-600' },
    { label: 'Treasury', percentage: 20, color: 'from-indigo-500 to-purple-600' },
  ];

  return (
    <section id="tokenomics" className="py-20 bg-slate-50 dark:bg-slate-900/50">
      <div className="container mx-auto px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="text-center mb-16"
        >
          <h2 className="text-5xl md:text-6xl font-black mb-6">
            Tokenomics
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            Fair launch, no premine, deflationary by design
          </p>
        </motion.div>

        <div className="grid md:grid-cols-2 gap-12 max-w-7xl mx-auto">
          {/* Token Info Card */}
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="bg-gradient-to-br from-indigo-500 to-purple-600 p-8 rounded-3xl shadow-2xl text-white"
          >
            <h3 className="text-3xl font-bold mb-8 flex items-center space-x-3">
              <Coins size={32} />
              <span>Qubitum (QBT)</span>
            </h3>

            <div className="space-y-4">
              {tokenInfo.map((item, index) => {
                const Icon = item.icon;
                return (
                  <motion.div
                    key={item.label}
                    initial={{ opacity: 0, x: -20 }}
                    whileInView={{ opacity: 1, x: 0 }}
                    viewport={{ once: true }}
                    transition={{ duration: 0.4, delay: index * 0.1 }}
                    className="flex justify-between items-center border-b border-white/20 pb-3"
                  >
                    <span className="flex items-center space-x-2 text-white/90">
                      <Icon size={18} />
                      <span>{item.label}</span>
                    </span>
                    <span className="font-bold text-xl">{item.value}</span>
                  </motion.div>
                );
              })}
            </div>

            <div className="mt-8 p-6 bg-white/10 rounded-2xl backdrop-blur-sm">
              <div className="text-sm text-white/80 mb-2">Distribution</div>
              <div className="font-bold text-3xl mb-2">100% Fair Launch</div>
              <div className="text-sm text-white/90">No premine • No ICO • No team allocation</div>
            </div>
          </motion.div>

          {/* Fee Distribution */}
          <motion.div
            initial={{ opacity: 0, x: 50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6 }}
            className="bg-white dark:bg-slate-800 p-8 rounded-3xl shadow-2xl border border-slate-100 dark:border-slate-700"
          >
            <h3 className="text-3xl font-bold mb-8">Fee Distribution</h3>

            <div className="space-y-8">
              {feeDistribution.map((item, index) => (
                <motion.div
                  key={item.label}
                  initial={{ opacity: 0, scale: 0.8 }}
                  whileInView={{ opacity: 1, scale: 1 }}
                  viewport={{ once: true }}
                  transition={{ duration: 0.4, delay: index * 0.1 }}
                >
                  <div className="flex justify-between mb-3">
                    <span className="font-semibold text-slate-700 dark:text-slate-300">{item.label}</span>
                    <span className="font-bold text-xl">{item.percentage}%</span>
                  </div>
                  <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-4 overflow-hidden">
                    <motion.div
                      initial={{ width: 0 }}
                      whileInView={{ width: `${item.percentage}%` }}
                      viewport={{ once: true }}
                      transition={{ duration: 1, delay: index * 0.1 + 0.3, ease: 'easeOut' }}
                      className={`bg-gradient-to-r ${item.color} h-4 rounded-full`}
                    />
                  </div>
                </motion.div>
              ))}
            </div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6, delay: 0.5 }}
              className="mt-8 p-6 bg-gradient-to-br from-red-50 to-orange-50 dark:from-red-900/20 dark:to-orange-900/20 rounded-2xl border-2 border-red-200 dark:border-red-800"
            >
              <div className="flex items-start space-x-3">
                <Flame className="text-red-500 flex-shrink-0 mt-1" size={24} />
                <div>
                  <h4 className="font-bold text-lg mb-2">Deflationary Model</h4>
                  <p className="text-sm text-slate-700 dark:text-slate-300">
                    30% of every transaction fee is permanently burned, making QBT increasingly scarce over time. 
                    Combined with the 2-year halving schedule, this creates strong deflationary pressure.
                  </p>
                </div>
              </div>
            </motion.div>
          </motion.div>
        </div>

        {/* Comparison Table */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="max-w-5xl mx-auto"
        >
          <div className="bg-white dark:bg-slate-800 p-8 rounded-3xl shadow-2xl border border-slate-100 dark:border-slate-700 overflow-x-auto">
            <h3 className="text-2xl font-bold mb-6 text-center">SpiraChain vs Bitcoin</h3>
            <table className="w-full">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="text-left pb-4 text-slate-600 dark:text-slate-400">Metric</th>
                  <th className="text-center pb-4 text-slate-600 dark:text-slate-400">Bitcoin</th>
                  <th className="text-center pb-4 text-slate-600 dark:text-slate-400">SpiraChain</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-200 dark:divide-slate-700">
                <tr>
                  <td className="py-4 font-medium">Total Supply</td>
                  <td className="py-4 text-center">21M BTC</td>
                  <td className="py-4 text-center font-bold text-indigo-600">21M QBT</td>
                </tr>
                <tr>
                  <td className="py-4 font-medium">Block Time</td>
                  <td className="py-4 text-center">10 min</td>
                  <td className="py-4 text-center font-bold text-green-600">30 sec (20x faster)</td>
                </tr>
                <tr>
                  <td className="py-4 font-medium">Halving</td>
                  <td className="py-4 text-center">~4 years</td>
                  <td className="py-4 text-center font-bold text-indigo-600">~2 years</td>
                </tr>
                <tr>
                  <td className="py-4 font-medium">Fee Burning</td>
                  <td className="py-4 text-center">No</td>
                  <td className="py-4 text-center font-bold text-red-600">Yes (30%)</td>
                </tr>
                <tr>
                  <td className="py-4 font-medium">Energy Cost</td>
                  <td className="py-4 text-center">Very High</td>
                  <td className="py-4 text-center font-bold text-green-600">Very Low (99.9% less)</td>
                </tr>
              </tbody>
            </table>
          </div>
        </motion.div>
      </div>
    </section>
  );
}

