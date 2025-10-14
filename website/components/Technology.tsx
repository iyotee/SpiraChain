'use client';

import { motion } from 'framer-motion';
import { Code, Shield, Brain, Infinity, Zap, Lock, Cpu, Network } from 'lucide-react';

export default function Technology() {
  const techSpecs = [
    {
      icon: Shield,
      title: 'Post-Quantum Cryptography',
      specs: [
        { label: 'Signature Scheme', value: 'XMSS (eXtended Merkle Signature Scheme)' },
        { label: 'Encryption', value: 'McEliece with Goppa codes' },
        { label: 'Key Exchange', value: 'Kyber-1024 (NIST approved)' },
        { label: 'Security Level', value: '256-bit quantum resistance' },
      ],
      gradient: 'from-red-500 to-pink-500',
      code: `// Post-quantum signature verification
fn verify_xmss_signature(
    message: &[u8],
    signature: &XMSSSignature,
    public_key: &XMSSPublicKey
) -> bool {
    xmss::verify(message, signature, public_key)
}`
    },
    {
      icon: Brain,
      title: 'AI Semantic Layer',
      specs: [
        { label: 'Model Architecture', value: 'Transformer-based NLP' },
        { label: 'Analysis', value: 'Real-time transaction scoring' },
        { label: 'Fraud Detection', value: 'Pattern recognition & anomaly detection' },
        { label: 'Quality Rewards', value: 'Up to 100 QBT per transaction' },
      ],
      gradient: 'from-blue-500 to-cyan-500',
      code: `// AI semantic analysis
async fn analyze_transaction(tx: &Transaction) -> Score {
    let embedding = ai::encode_semantics(&tx.data);
    let quality = ai::score_quality(embedding);
    let fraud_risk = ai::detect_fraud(embedding);
    Score { quality, fraud_risk }
}`
    },
    {
      icon: Infinity,
      title: 'Proof-of-Spiral Consensus',
      specs: [
        { label: 'Algorithm', value: 'Fibonacci spiral generation' },
        { label: 'Energy Usage', value: '0.15 TWh/year (99.9% less than Bitcoin)' },
        { label: 'Block Time', value: '30 seconds average' },
        { label: 'Finality', value: 'Instant (single confirmation)' },
      ],
      gradient: 'from-purple-500 to-indigo-500',
      code: `// Spiral proof generation
fn generate_spiral_proof(
    seed: &Hash,
    difficulty: u64
) -> SpiralProof {
    let spiral = fibonacci_spiral(seed);
    validate_complexity(spiral, difficulty)
}`
    },
  ];

  const networkStats = [
    { icon: Zap, label: 'Throughput', value: '10,000+ TPS', gradient: 'from-yellow-500 to-orange-500' },
    { icon: Lock, label: 'Security', value: '256-bit quantum', gradient: 'from-green-500 to-emerald-500' },
    { icon: Cpu, label: 'Min Hardware', value: 'Raspberry Pi 4', gradient: 'from-blue-500 to-cyan-500' },
    { icon: Network, label: 'Consensus', value: 'Byzantine Fault Tolerant', gradient: 'from-purple-500 to-pink-500' },
  ];

  return (
    <section id="technology" className="py-24 md:py-32 relative overflow-hidden section-gradient-dark">
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
            className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-indigo-600/20 via-purple-600/20 to-pink-600/20 border border-indigo-500/30 rounded-full backdrop-blur-sm mb-8"
          >
            <Code className="w-5 h-5 text-indigo-400" />
            <span className="text-sm font-bold text-indigo-300">Technology Deep Dive</span>
          </motion.div>

          <h2 className="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-black mb-6">
            <span className="block text-white">
              Built on
            </span>
            <span className="block gradient-text-hero">
              Cutting-Edge Innovation
            </span>
          </h2>
          
          <p className="text-lg sm:text-xl md:text-2xl text-gray-300 max-w-3xl mx-auto leading-relaxed">
            Advanced cryptography, artificial intelligence, and sustainable consensus
          </p>
        </motion.div>

        <div className="grid lg:grid-cols-2 gap-8 md:gap-12 mb-16 md:mb-20">
          {networkStats.map((stat, index) => (
            <motion.div
              key={stat.label}
              initial={{ opacity: 0, scale: 0.9 }}
              whileInView={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
              viewport={{ once: true }}
              whileHover={{ scale: 1.03, y: -5 }}
              className="p-6 md:p-8 glass-card rounded-2xl shadow-xl hover:shadow-2xl transition-all duration-300 flex items-center gap-4 md:gap-6 group"
            >
              <div className={`p-4 rounded-2xl bg-gradient-to-r ${stat.gradient} shadow-lg group-hover:scale-110 transition-transform duration-300`}>
                <stat.icon className="w-8 h-8 text-white" />
              </div>
              <div>
                <div className={`text-2xl md:text-3xl font-black bg-gradient-to-r ${stat.gradient} bg-clip-text text-transparent mb-1`}>
                  {stat.value}
                </div>
                <div className="text-sm font-semibold text-gray-300">
                  {stat.label}
                </div>
              </div>
            </motion.div>
          ))}
        </div>

        <div className="space-y-8 md:space-y-12">
          {techSpecs.map((tech, index) => (
            <motion.div
              key={tech.title}
              initial={{ opacity: 0, y: 50 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.8, delay: index * 0.2 }}
              viewport={{ once: true }}
              className="glass-card rounded-3xl p-6 md:p-10 shadow-2xl"
            >
              <div className="grid lg:grid-cols-2 gap-8 md:gap-12">
                <div>
                  <div className="flex items-center gap-4 mb-6">
                    <div className={`p-4 rounded-2xl bg-gradient-to-r ${tech.gradient} shadow-lg`}>
                      <tech.icon className="w-8 h-8 text-white" />
                    </div>
                    <h3 className="text-2xl md:text-3xl font-black text-white">
                      {tech.title}
                    </h3>
                  </div>

                  <div className="space-y-4">
                    {tech.specs.map((spec, specIndex) => (
                      <motion.div
                        key={spec.label}
                        initial={{ opacity: 0, x: -20 }}
                        whileInView={{ opacity: 1, x: 0 }}
                        transition={{ duration: 0.5, delay: index * 0.2 + specIndex * 0.1 }}
                        viewport={{ once: true }}
                        className="p-4 bg-white/5 rounded-xl border border-white/10 hover:bg-white/10 transition-colors"
                      >
                        <div className="text-sm font-semibold text-gray-400 mb-1">
                          {spec.label}
                        </div>
                        <div className="text-base font-bold text-white">
                          {spec.value}
                        </div>
                      </motion.div>
                    ))}
                  </div>
                </div>

                <div>
                  <div className="text-sm font-semibold text-gray-400 mb-3">
                    Implementation Example
                  </div>
                  <motion.div
                    initial={{ opacity: 0, scale: 0.95 }}
                    whileInView={{ opacity: 1, scale: 1 }}
                    transition={{ duration: 0.5, delay: index * 0.2 + 0.3 }}
                    viewport={{ once: true }}
                    className="relative"
                  >
                    <div className="absolute inset-0 bg-gradient-to-r from-slate-900 to-slate-800 rounded-2xl blur-sm" />
                    <div className="relative bg-slate-900/90 backdrop-blur-sm rounded-2xl p-4 md:p-6 border border-slate-700 overflow-x-auto">
                      <pre className="text-xs md:text-sm text-gray-300 font-mono leading-relaxed">
                        <code>{tech.code}</code>
                      </pre>
                    </div>
                    <div className={`absolute -inset-1 bg-gradient-to-r ${tech.gradient} rounded-2xl blur opacity-20 -z-10`} />
                  </motion.div>
                </div>
              </div>
            </motion.div>
          ))}
        </div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.6 }}
          viewport={{ once: true }}
          className="mt-16 text-center"
        >
          <div className="inline-block p-1 bg-gradient-to-r from-indigo-600 to-purple-600 rounded-2xl">
            <motion.a
              href="#get-started"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="block px-8 md:px-12 py-4 md:py-6 bg-slate-900 text-white font-black text-lg md:text-xl rounded-xl hover:bg-slate-800 transition-colors duration-300"
            >
              Experience the Technology
            </motion.a>
          </div>
        </motion.div>
      </div>
    </section>
  );
}

