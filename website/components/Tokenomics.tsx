'use client';

import { motion } from 'framer-motion';
import { Coins, TrendingUp, Users, Lock, Award, Zap, Target, Calculator } from 'lucide-react';
import { useState } from 'react';
import StatsCounter from './StatsCounter';

export default function Tokenomics() {
  const [stakeAmount, setStakeAmount] = useState(1000);
  const annualReward = (stakeAmount * 0.15).toFixed(2);
  const monthlyReward = (parseFloat(annualReward) / 12).toFixed(2);

  const tokenomicsData = [
    {
      category: 'Community Rewards',
      percentage: 40,
      amount: '8.4M QBT',
      description: 'Distributed to validators and community contributors',
      color: 'from-blue-500 to-cyan-500',
      icon: Users
    },
    {
      category: 'Development Fund',
      percentage: 25,
      amount: '5.25M QBT',
      description: 'Long-term development and ecosystem growth',
      color: 'from-purple-500 to-pink-500',
      icon: TrendingUp
    },
    {
      category: 'Team & Advisors',
      percentage: 20,
      amount: '4.2M QBT',
      description: 'Core team and strategic advisors',
      color: 'from-green-500 to-emerald-500',
      icon: Award
    },
    {
      category: 'Liquidity Pool',
      percentage: 15,
      amount: '3.15M QBT',
      description: 'Initial liquidity and exchange listings',
      color: 'from-orange-500 to-yellow-500',
      icon: Coins
    }
  ];

  const rewardsStructure = [
    {
      type: 'Block Production',
      reward: '10-50 QBT',
      frequency: 'Every 30 seconds',
      icon: Zap,
      color: 'from-yellow-400 to-orange-500',
    },
    {
      type: 'AI Quality Bonus',
      reward: 'Up to 100 QBT',
      frequency: 'Per transaction',
      icon: Target,
      color: 'from-blue-400 to-cyan-500',
    },
    {
      type: 'Network Security',
      reward: '25 QBT',
      frequency: 'Per consensus round',
      icon: Lock,
      color: 'from-green-400 to-emerald-500',
    },
  ];

  const validatorRequirements = [
    {
      icon: Coins,
      title: 'Minimum Stake',
      value: '1,000 QBT',
      description: 'Tokens required to become a validator',
      color: 'from-blue-500 to-cyan-500'
    },
    {
      icon: Zap,
      title: 'Hardware',
      value: '$80 RPi',
      description: 'Raspberry Pi or equivalent hardware',
      color: 'from-green-500 to-emerald-500'
    },
    {
      icon: Lock,
      title: 'Uptime',
      value: '99%+',
      description: 'Minimum uptime requirement',
      color: 'from-purple-500 to-pink-500'
    },
    {
      icon: Target,
      title: 'Quality Score',
      value: '8.5+',
      description: 'Minimum AI analysis quality score',
      color: 'from-orange-500 to-yellow-500'
    }
  ];

  return (
    <section id="tokenomics" className="py-24 md:py-32 relative overflow-hidden section-gradient-light">
      <div className="absolute inset-0 opacity-30">
        <div className="absolute top-20 left-20 w-96 h-96 bg-emerald-500/10 rounded-full blur-3xl" />
        <div className="absolute bottom-20 right-20 w-96 h-96 bg-teal-500/10 rounded-full blur-3xl" />
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
            className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-emerald-600/10 via-teal-600/10 to-cyan-600/10 border border-emerald-500/20 rounded-full backdrop-blur-sm mb-8"
          >
            <Coins className="w-5 h-5 text-emerald-600 dark:text-emerald-400" />
            <span className="text-sm font-bold text-emerald-600 dark:text-emerald-400">Economics</span>
          </motion.div>

          <h2 className="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black mb-6">
            <span className="block text-slate-900 dark:text-white">
              Tokenomics
            </span>
            <span className="block bg-gradient-to-r from-emerald-600 via-teal-600 to-cyan-600 bg-clip-text text-transparent">
              Designed for Growth
            </span>
          </h2>
          
          <p className="text-lg sm:text-xl md:text-2xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto leading-relaxed">
            Sustainable economics that reward quality over quantity
          </p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.2 }}
          viewport={{ once: true }}
          className="text-center mb-16 md:mb-20"
        >
          <div className="inline-block p-8 md:p-12 glass-card rounded-3xl shadow-2xl border-2 border-emerald-500/20">
            <div className="text-6xl sm:text-7xl md:text-8xl lg:text-9xl font-black bg-gradient-to-r from-emerald-600 to-cyan-600 bg-clip-text text-transparent mb-4">
              21M
            </div>
            <div className="text-xl md:text-2xl font-bold text-slate-900 dark:text-white mb-2">
              Total Supply
            </div>
            <div className="text-base md:text-lg text-slate-600 dark:text-slate-400">
              QBT (Quantum Blockchain Token)
            </div>
          </div>
        </motion.div>

        <div className="grid lg:grid-cols-2 gap-8 md:gap-12 mb-16 md:mb-20">
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            whileInView={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.8, delay: 0.4 }}
            viewport={{ once: true }}
            className="glass-card rounded-3xl p-6 md:p-8 shadow-2xl"
          >
            <h3 className="text-2xl md:text-3xl font-bold text-slate-900 dark:text-white mb-8 text-center">
              Token Distribution
            </h3>
            
            <div className="space-y-6">
              {tokenomicsData.map((item, index) => (
                <motion.div
                  key={item.category}
                  initial={{ opacity: 0, x: -30 }}
                  whileInView={{ opacity: 1, x: 0 }}
                  transition={{ duration: 0.6, delay: 0.5 + index * 0.1 }}
                  viewport={{ once: true }}
                  className="relative"
                >
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-3">
                      <div className={`p-2 rounded-lg bg-gradient-to-r ${item.color}`}>
                        <item.icon className="w-5 h-5 text-white" />
                      </div>
                      <span className="font-semibold text-slate-900 dark:text-white">{item.category}</span>
                    </div>
                    <div className="text-right">
                      <div className="font-bold text-slate-900 dark:text-white">{item.percentage}%</div>
                      <div className="text-xs text-slate-600 dark:text-slate-400">{item.amount}</div>
                    </div>
                  </div>
                  
                  <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-3 overflow-hidden">
                    <motion.div
                      initial={{ width: 0 }}
                      whileInView={{ width: `${item.percentage}%` }}
                      transition={{ duration: 1, delay: 0.7 + index * 0.1 }}
                      viewport={{ once: true }}
                      className={`h-3 rounded-full bg-gradient-to-r ${item.color}`}
                    />
                  </div>
                  
                  <p className="text-xs sm:text-sm text-slate-600 dark:text-slate-400 mt-2">
                    {item.description}
                  </p>
                </motion.div>
              ))}
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, x: 50 }}
            whileInView={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.8, delay: 0.4 }}
            viewport={{ once: true }}
            className="space-y-6"
          >
            <div className="glass-card rounded-3xl p-6 md:p-8 shadow-2xl">
              <div className="flex items-center gap-3 mb-6">
                <Calculator className="w-6 h-6 text-emerald-600 dark:text-emerald-400" />
                <h3 className="text-2xl font-bold text-slate-900 dark:text-white">
                  Staking Calculator
                </h3>
              </div>
              
              <div className="space-y-6">
                <div>
                  <label htmlFor="stake-amount-slider" className="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">
                    Stake Amount (QBT)
                  </label>
                  <input
                    id="stake-amount-slider"
                    type="range"
                    min="1000"
                    max="100000"
                    step="1000"
                    value={stakeAmount}
                    onChange={(e) => setStakeAmount(parseInt(e.target.value))}
                    className="w-full h-3 bg-slate-200 dark:bg-slate-700 rounded-lg appearance-none cursor-pointer"
                    aria-label="Stake amount"
                  />
                  <div className="text-center mt-2">
                    <span className="text-3xl font-black bg-gradient-to-r from-emerald-600 to-cyan-600 bg-clip-text text-transparent">
                      {stakeAmount.toLocaleString()}
                    </span>
                    <span className="text-slate-600 dark:text-slate-400 ml-2">QBT</span>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div className="p-4 bg-slate-50 dark:bg-slate-800/50 rounded-xl">
                    <div className="text-xs text-slate-600 dark:text-slate-400 mb-1">Monthly Rewards</div>
                    <div className="text-2xl font-black bg-gradient-to-r from-emerald-600 to-teal-600 bg-clip-text text-transparent">
                      {monthlyReward}
                    </div>
                    <div className="text-xs text-slate-600 dark:text-slate-400">QBT</div>
                  </div>
                  <div className="p-4 bg-slate-50 dark:bg-slate-800/50 rounded-xl">
                    <div className="text-xs text-slate-600 dark:text-slate-400 mb-1">Annual Rewards</div>
                    <div className="text-2xl font-black bg-gradient-to-r from-cyan-600 to-blue-600 bg-clip-text text-transparent">
                      {annualReward}
                    </div>
                    <div className="text-xs text-slate-600 dark:text-slate-400">QBT</div>
                  </div>
                </div>

                <div className="text-xs text-center text-slate-600 dark:text-slate-400">
                  15% Annual Percentage Yield (APY)
                </div>
              </div>
            </div>

            <div className="glass-card rounded-3xl p-6 md:p-8 shadow-2xl">
              <h4 className="text-xl font-bold text-slate-900 dark:text-white mb-4">
                Reward Structure
              </h4>
              
              <div className="space-y-4">
                {rewardsStructure.map((reward, index) => (
                  <motion.div
                    key={reward.type}
                    initial={{ opacity: 0, y: 20 }}
                    whileInView={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.5, delay: 0.5 + index * 0.1 }}
                    viewport={{ once: true }}
                    className="flex items-start gap-3 p-3 bg-slate-50 dark:bg-slate-800/50 rounded-xl hover:scale-102 transition-transform"
                  >
                    <div className={`p-2 rounded-lg bg-gradient-to-r ${reward.color} shadow-lg flex-shrink-0`}>
                      <reward.icon className="w-5 h-5 text-white" />
                    </div>
                    
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center justify-between mb-1">
                        <h5 className="text-sm font-bold text-slate-900 dark:text-white">
                          {reward.type}
                        </h5>
                        <div className="text-right">
                          <div className="text-sm font-bold text-slate-900 dark:text-white">
                            {reward.reward}
                          </div>
                        </div>
                      </div>
                      <div className="text-xs text-slate-600 dark:text-slate-400">
                        {reward.frequency}
                      </div>
                    </div>
                  </motion.div>
                ))}
              </div>
            </div>
          </motion.div>
        </div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.6 }}
          viewport={{ once: true }}
          className="mb-16"
        >
          <h3 className="text-3xl sm:text-4xl md:text-5xl font-black text-center text-slate-900 dark:text-white mb-12">
            Become a Validator
          </h3>

          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6 md:gap-8">
            {validatorRequirements.map((req, index) => (
              <motion.div
                key={req.title}
                initial={{ opacity: 0, scale: 0.8 }}
                whileInView={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.6, delay: 0.7 + index * 0.1 }}
                viewport={{ once: true }}
                whileHover={{ scale: 1.05, y: -5 }}
                className="group text-center"
              >
                <div className="p-6 md:p-8 glass-card rounded-2xl shadow-xl hover:shadow-2xl transition-all duration-300">
                  <div className={`inline-flex p-4 rounded-2xl bg-gradient-to-r ${req.color} shadow-lg mb-6 group-hover:scale-110 transition-transform duration-300`}>
                    <req.icon className="w-8 h-8 text-white" />
                  </div>
                  
                  <h4 className="text-xl font-bold text-slate-900 dark:text-white mb-3">
                    {req.title}
                  </h4>
                  
                  <div className={`text-2xl md:text-3xl font-black bg-gradient-to-r ${req.color} bg-clip-text text-transparent mb-4`}>
                    {req.value}
                  </div>
                  
                  <p className="text-sm text-slate-600 dark:text-slate-400">
                    {req.description}
                  </p>
                </div>
              </motion.div>
            ))}
          </div>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.8 }}
          viewport={{ once: true }}
          className="text-center"
        >
          <div className="inline-block p-1 bg-gradient-to-r from-emerald-600 to-cyan-600 rounded-2xl">
            <motion.a
              href="#get-started"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="block px-8 md:px-12 py-4 md:py-6 bg-white dark:bg-slate-900 text-slate-900 dark:text-white font-black text-lg md:text-xl rounded-xl hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors duration-300"
            >
              Start Earning QBT Today
            </motion.a>
          </div>
        </motion.div>
      </div>
    </section>
  );
}
