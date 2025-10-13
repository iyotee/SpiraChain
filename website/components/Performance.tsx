'use client';

import { motion } from 'framer-motion';
import { Cpu, Zap, TrendingUp, Activity, BarChart3, Target, Award } from 'lucide-react';

export default function Performance() {
  const performanceMetrics = [
    {
      icon: Zap,
      title: 'Block Time',
      value: '30s',
      description: 'Lightning fast block production',
      color: 'from-yellow-400 to-orange-500',
      improvement: '+300% faster than Bitcoin'
    },
    {
      icon: Activity,
      title: 'Energy Efficiency',
      value: '99.9%',
      description: 'Less energy consumption',
      color: 'from-green-400 to-emerald-500',
      improvement: 'vs traditional mining'
    },
    {
      icon: Cpu,
      title: 'Hardware Cost',
      value: '$80',
      description: 'Raspberry Pi validator',
      color: 'from-blue-400 to-cyan-500',
      improvement: 'vs $10,000+ mining rigs'
    },
    {
      icon: TrendingUp,
      title: 'Throughput',
      value: '10,000+',
      description: 'Transactions per second',
      color: 'from-purple-400 to-pink-500',
      improvement: 'Scalable architecture'
    }
  ];

  const benchmarks = [
    {
      name: 'Bitcoin',
      energy: '150 TWh/year',
      cost: '$10,000+',
      speed: '10 min',
      color: 'bg-orange-500',
      percentage: 100
    },
    {
      name: 'Ethereum',
      energy: '78 TWh/year',
      cost: '$2,000+',
      speed: '15 sec',
      color: 'bg-blue-500',
      percentage: 75
    },
    {
      name: 'SpiraChain',
      energy: '0.15 TWh/year',
      cost: '$80',
      speed: '30 sec',
      color: 'bg-gradient-to-r from-purple-500 to-pink-500',
      percentage: 5
    }
  ];

  const features = [
    {
      icon: Target,
      title: 'Precision Mining',
      description: 'Mathematical spirals instead of brute force hashing',
      color: 'from-indigo-500 to-purple-500'
    },
    {
      icon: BarChart3,
      title: 'Real-time Analytics',
      description: 'AI-powered transaction analysis and optimization',
      color: 'from-pink-500 to-rose-500'
    },
    {
      icon: Award,
      title: 'Quality Rewards',
      description: 'Rewards based on transaction quality, not quantity',
      color: 'from-green-500 to-teal-500'
    }
  ];

  return (
    <section className="py-32 relative overflow-hidden bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
      {/* Animated Background */}
      <div className="absolute inset-0">
        <div className="absolute top-20 left-20 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl animate-pulse" />
        <div className="absolute bottom-20 right-20 w-96 h-96 bg-blue-500/10 rounded-full blur-3xl animate-pulse delay-1000" />
        <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-gradient-to-r from-purple-600/5 to-blue-600/5 rounded-full blur-3xl animate-spin" style={{animationDuration: '30s'}} />
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
            <span className="bg-gradient-to-r from-white to-gray-300 bg-clip-text text-transparent">
              Performance
            </span>
            <br />
            <span className="bg-gradient-to-r from-purple-400 via-pink-500 to-blue-400 bg-clip-text text-transparent">
              That Matters
            </span>
          </h2>
          
          <p className="text-2xl md:text-3xl text-gray-300 max-w-4xl mx-auto leading-relaxed">
            Designed for CPU, GPU, and AI. Optimized for efficiency and sustainability.
          </p>
        </motion.div>

        {/* Performance Metrics */}
        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8 mb-24">
          {performanceMetrics.map((metric, index) => (
            <motion.div
              key={metric.title}
              initial={{ opacity: 0, y: 50, scale: 0.8 }}
              whileInView={{ opacity: 1, y: 0, scale: 1 }}
              transition={{ duration: 0.6, delay: index * 0.1 }}
              viewport={{ once: true }}
              whileHover={{ scale: 1.05, y: -10 }}
              className="group"
            >
              <div className="relative p-8 bg-white/10 backdrop-blur-sm border border-white/20 rounded-3xl shadow-2xl hover:shadow-3xl transition-all duration-300 text-center">
                {/* Icon */}
                <div className={`inline-flex p-4 rounded-2xl bg-gradient-to-r ${metric.color} shadow-lg mb-6`}>
                  <metric.icon className="w-8 h-8 text-white" />
                </div>

                {/* Value */}
                <div className={`text-5xl md:text-6xl font-black bg-gradient-to-r ${metric.color} bg-clip-text text-transparent mb-4`}>
                  {metric.value}
                </div>

                {/* Title */}
                <h3 className="text-2xl font-bold text-white mb-3">
                  {metric.title}
                </h3>

                {/* Description */}
                <p className="text-gray-300 mb-4">
                  {metric.description}
                </p>

                {/* Improvement */}
                <div className="text-sm text-gray-400 font-medium">
                  {metric.improvement}
                </div>

                {/* Background Effect */}
                <div className={`absolute inset-0 bg-gradient-to-r ${metric.color} opacity-0 group-hover:opacity-10 rounded-3xl transition-opacity duration-300 -z-10`} />
              </div>
            </motion.div>
          ))}
        </div>

        {/* Benchmark Comparison */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.4 }}
          viewport={{ once: true }}
          className="mb-24"
        >
          <h3 className="text-4xl md:text-5xl font-black text-center text-white mb-12">
            Benchmark Comparison
          </h3>

          <div className="bg-white/5 backdrop-blur-sm border border-white/10 rounded-3xl p-8 shadow-2xl">
            <div className="grid md:grid-cols-3 gap-8">
              {benchmarks.map((benchmark, index) => (
                <motion.div
                  key={benchmark.name}
                  initial={{ opacity: 0, x: -30 }}
                  whileInView={{ opacity: 1, x: 0 }}
                  transition={{ duration: 0.6, delay: 0.5 + index * 0.1 }}
                  viewport={{ once: true }}
                  className={`relative p-6 rounded-2xl ${benchmark.name === 'SpiraChain' ? 'bg-white/10 border-2 border-purple-500/50' : 'bg-white/5 border border-white/10'}`}
                >
                  {/* Highlight Badge for SpiraChain */}
                  {benchmark.name === 'SpiraChain' && (
                    <div className="absolute -top-3 left-1/2 transform -translate-x-1/2">
                      <div className="bg-gradient-to-r from-purple-600 to-pink-600 text-white px-4 py-1 rounded-full text-sm font-bold">
                        WINNER
                      </div>
                    </div>
                  )}

                  <div className="text-center">
                    <div className={`w-16 h-16 mx-auto rounded-2xl ${benchmark.color} flex items-center justify-center mb-4`}>
                      <span className="text-white font-bold text-lg">{benchmark.name[0]}</span>
                    </div>

                    <h4 className="text-2xl font-bold text-white mb-6">
                      {benchmark.name}
                    </h4>

                    <div className="space-y-4">
                      <div>
                        <div className="text-sm text-gray-400 mb-1">Energy Consumption</div>
                        <div className="text-lg font-semibold text-white">{benchmark.energy}</div>
                      </div>

                      <div>
                        <div className="text-sm text-gray-400 mb-1">Hardware Cost</div>
                        <div className="text-lg font-semibold text-white">{benchmark.cost}</div>
                      </div>

                      <div>
                        <div className="text-sm text-gray-400 mb-1">Block Time</div>
                        <div className="text-lg font-semibold text-white">{benchmark.speed}</div>
                      </div>

                      {/* Efficiency Bar */}
                      <div className="mt-4">
                        <div className="flex justify-between text-sm text-gray-400 mb-2">
                          <span>Efficiency</span>
                          <span>{benchmark.percentage}%</span>
                        </div>
                        <div className="w-full bg-gray-700 rounded-full h-3">
                          <motion.div
                            initial={{ width: 0 }}
                            whileInView={{ width: `${benchmark.percentage}%` }}
                            transition={{ duration: 1, delay: 0.8 + index * 0.2 }}
                            viewport={{ once: true }}
                            className={`h-3 rounded-full ${benchmark.color}`}
                          />
                        </div>
                      </div>
                    </div>
                  </div>
                </motion.div>
              ))}
            </div>
          </div>
        </motion.div>

        {/* Key Features */}
        <div className="grid md:grid-cols-3 gap-8">
          {features.map((feature, index) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, y: 30 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: 0.6 + index * 0.1 }}
              viewport={{ once: true }}
              whileHover={{ scale: 1.05, y: -5 }}
              className="group"
            >
              <div className="p-8 bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl shadow-xl hover:shadow-2xl transition-all duration-300 text-center">
                <div className={`inline-flex p-4 rounded-2xl bg-gradient-to-r ${feature.color} shadow-lg mb-6`}>
                  <feature.icon className="w-8 h-8 text-white" />
                </div>
                
                <h4 className="text-2xl font-bold text-white mb-4">
                  {feature.title}
                </h4>
                
                <p className="text-gray-300 leading-relaxed">
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
          transition={{ duration: 0.8, delay: 0.8 }}
          viewport={{ once: true }}
          className="text-center mt-20"
        >
          <div className="inline-block p-1 bg-gradient-to-r from-purple-600 to-blue-600 rounded-2xl">
            <motion.a
              href="#get-started"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="block px-12 py-6 bg-white text-slate-900 font-black text-xl rounded-xl hover:bg-gray-50 transition-colors duration-300"
            >
              Experience the Performance
            </motion.a>
          </div>
        </motion.div>
      </div>
    </section>
  );
}