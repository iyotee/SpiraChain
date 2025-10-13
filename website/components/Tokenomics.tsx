'use client';

import { motion } from 'framer-motion';
import { Coins, TrendingUp, Users, Lock, Award, Zap, Target } from 'lucide-react';

export default function Tokenomics() {
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
      description: 'Base reward for producing valid blocks'
    },
    {
      type: 'AI Quality Bonus',
      reward: 'Up to 100 QBT',
      frequency: 'Per transaction',
      icon: Target,
      color: 'from-blue-400 to-cyan-500',
      description: 'Bonus for high-quality AI analysis'
    },
    {
      type: 'Network Security',
      reward: '25 QBT',
      frequency: 'Per consensus round',
      icon: Lock,
      color: 'from-green-400 to-emerald-500',
      description: 'Reward for maintaining network security'
    },
    {
      type: 'Community Contribution',
      reward: 'Variable',
      frequency: 'Monthly',
      icon: Users,
      color: 'from-purple-400 to-pink-500',
      description: 'Rewards for community development'
    }
  ];

  const validatorRequirements = [
    {
      icon: Coins,
      title: 'Minimum Stake',
      value: '1,000 QBT',
      description: 'Minimum tokens required to become a validator',
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
          <h2 className="text-6xl md:text-8xl font-black mb-8">
            <span className="bg-gradient-to-r from-slate-900 to-slate-700 dark:from-white dark:to-gray-300 bg-clip-text text-transparent">
              Tokenomics
            </span>
            <br />
            <span className="bg-gradient-to-r from-purple-600 via-pink-600 to-blue-600 bg-clip-text text-transparent">
              Designed for Growth
            </span>
          </h2>
          
          <p className="text-2xl md:text-3xl text-slate-600 dark:text-slate-400 max-w-4xl mx-auto leading-relaxed">
            Sustainable economics that reward quality over quantity
          </p>
        </motion.div>

        {/* Token Supply Overview */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.2 }}
          viewport={{ once: true }}
          className="text-center mb-20"
        >
          <div className="inline-block p-8 bg-white/80 dark:bg-slate-800/80 backdrop-blur-sm border-2 border-purple-500/20 rounded-3xl shadow-2xl">
            <div className="text-8xl md:text-9xl font-black bg-gradient-to-r from-purple-600 to-blue-600 bg-clip-text text-transparent mb-4">
              21M
            </div>
            <div className="text-2xl font-bold text-slate-900 dark:text-white mb-2">
              Total Supply
            </div>
            <div className="text-lg text-slate-600 dark:text-slate-400">
              QBT (Quantum Blockchain Token)
            </div>
          </div>
        </motion.div>

        {/* Token Distribution */}
        <div className="grid lg:grid-cols-2 gap-12 mb-24">
          {/* Distribution Chart */}
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            whileInView={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.8, delay: 0.4 }}
            viewport={{ once: true }}
            className="bg-white/80 dark:bg-slate-800/80 backdrop-blur-sm border border-slate-200/50 dark:border-slate-700/50 rounded-3xl p-8 shadow-2xl"
          >
            <h3 className="text-3xl font-bold text-slate-900 dark:text-white mb-8 text-center">
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
                    <div className="flex items-center space-x-3">
                      <div className={`p-2 rounded-lg bg-gradient-to-r ${item.color}`}>
                        <item.icon className="w-5 h-5 text-white" />
                      </div>
                      <span className="font-semibold text-slate-900 dark:text-white">{item.category}</span>
                    </div>
                    <div className="text-right">
                      <div className="font-bold text-slate-900 dark:text-white">{item.percentage}%</div>
                      <div className="text-sm text-slate-600 dark:text-slate-400">{item.amount}</div>
                    </div>
                  </div>
                  
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-4">
                    <motion.div
                      initial={{ width: 0 }}
                      whileInView={{ width: `${item.percentage}%` }}
                      transition={{ duration: 1, delay: 0.7 + index * 0.1 }}
                      viewport={{ once: true }}
                      className={`h-4 rounded-full bg-gradient-to-r ${item.color}`}
                    />
                  </div>
                  
                  <p className="text-sm text-slate-600 dark:text-slate-400 mt-2">
                    {item.description}
                  </p>
                </motion.div>
              ))}
            </div>
          </motion.div>

          {/* Rewards Structure */}
          <motion.div
            initial={{ opacity: 0, x: 50 }}
            whileInView={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.8, delay: 0.4 }}
            viewport={{ once: true }}
            className="space-y-6"
          >
            <h3 className="text-3xl font-bold text-slate-900 dark:text-white mb-8 text-center">
              Reward Structure
            </h3>
            
            {rewardsStructure.map((reward, index) => (
              <motion.div
                key={reward.type}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6, delay: 0.5 + index * 0.1 }}
                viewport={{ once: true }}
                whileHover={{ scale: 1.02, y: -2 }}
                className="p-6 bg-white/80 dark:bg-slate-800/80 backdrop-blur-sm border border-slate-200/50 dark:border-slate-700/50 rounded-2xl shadow-xl hover:shadow-2xl transition-all duration-300"
              >
                <div className="flex items-start space-x-4">
                  <div className={`p-3 rounded-xl bg-gradient-to-r ${reward.color} shadow-lg`}>
                    <reward.icon className="w-6 h-6 text-white" />
                  </div>
                  
                  <div className="flex-1">
                    <div className="flex items-center justify-between mb-2">
                      <h4 className="text-xl font-bold text-slate-900 dark:text-white">
                        {reward.type}
                      </h4>
                      <div className="text-right">
                        <div className="font-bold text-slate-900 dark:text-white">
                          {reward.reward}
                        </div>
                        <div className="text-sm text-slate-600 dark:text-slate-400">
                          {reward.frequency}
                        </div>
                      </div>
                    </div>
                    
                    <p className="text-slate-600 dark:text-slate-400">
                      {reward.description}
                    </p>
                  </div>
                </div>
              </motion.div>
            ))}
          </motion.div>
        </div>

        {/* Validator Requirements */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.6 }}
          viewport={{ once: true }}
          className="mb-20"
        >
          <h3 className="text-4xl md:text-5xl font-black text-center text-slate-900 dark:text-white mb-12">
            Become a Validator
          </h3>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
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
                <div className="p-8 bg-white/80 dark:bg-slate-800/80 backdrop-blur-sm border border-slate-200/50 dark:border-slate-700/50 rounded-2xl shadow-xl hover:shadow-2xl transition-all duration-300">
                  <div className={`inline-flex p-4 rounded-2xl bg-gradient-to-r ${req.color} shadow-lg mb-6`}>
                    <req.icon className="w-8 h-8 text-white" />
                  </div>
                  
                  <h4 className="text-2xl font-bold text-slate-900 dark:text-white mb-3">
                    {req.title}
                  </h4>
                  
                  <div className={`text-3xl font-black bg-gradient-to-r ${req.color} bg-clip-text text-transparent mb-4`}>
                    {req.value}
                  </div>
                  
                  <p className="text-slate-600 dark:text-slate-400">
                    {req.description}
                  </p>
                </div>
              </motion.div>
            ))}
          </div>
        </motion.div>

        {/* Call to Action */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.8 }}
          viewport={{ once: true }}
          className="text-center"
        >
          <div className="inline-block p-1 bg-gradient-to-r from-purple-600 to-blue-600 rounded-2xl">
            <motion.a
              href="#get-started"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="block px-12 py-6 bg-white dark:bg-slate-900 text-slate-900 dark:text-white font-black text-xl rounded-xl hover:bg-gray-50 dark:hover:bg-slate-800 transition-colors duration-300"
            >
              Start Earning QBT Today
            </motion.a>
          </div>
        </motion.div>
      </div>
    </section>
  );
}