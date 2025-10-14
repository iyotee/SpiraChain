'use client';

import { motion } from 'framer-motion';
import { Infinity, Github, MessageCircle, Twitter, Mail, Send, FileText, Heart, ExternalLink } from 'lucide-react';
import { useState } from 'react';

export default function Footer() {
  const [email, setEmail] = useState('');
  const [subscribed, setSubscribed] = useState(false);

  const handleSubscribe = (e: React.FormEvent) => {
    e.preventDefault();
    setSubscribed(true);
    setTimeout(() => {
      setSubscribed(false);
      setEmail('');
    }, 3000);
  };

  const links = {
    resources: [
      { name: 'Whitepaper', href: 'https://github.com/iyotee/SpiraChain/blob/main/WHITEPAPER_VALIDATION.md' },
      { name: 'Roadmap', href: 'https://github.com/iyotee/SpiraChain/blob/main/ROADMAP.md' },
      { name: 'Documentation', href: 'https://github.com/iyotee/SpiraChain' },
      { name: 'GitHub', href: 'https://github.com/iyotee/SpiraChain' },
    ],
    developers: [
      { name: 'API Reference', href: '#' },
      { name: 'SDK Downloads', href: '#' },
      { name: 'Code Examples', href: '#' },
      { name: 'Developer Portal', href: '#' },
    ],
    community: [
      { name: 'Discord', href: '#', icon: MessageCircle },
      { name: 'Telegram', href: '#', icon: MessageCircle },
      { name: 'Twitter', href: '#', icon: Twitter },
      { name: 'GitHub', href: 'https://github.com/iyotee/SpiraChain', icon: Github },
    ],
    network: [
      { label: 'Bootstrap Node', value: '51.154.64.38:9000' },
      { label: 'RPC Port', value: '9933' },
      { label: 'DNS', value: 'bootstrap.spirachain.org' },
      { label: 'Chain ID', value: '7529' },
    ],
  };

  return (
    <footer className="relative overflow-hidden bg-gradient-to-b from-slate-900 to-slate-950 text-slate-400">
      <div className="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-purple-600 via-pink-500 to-blue-600" />

      <div className="absolute inset-0 opacity-10">
        <div className="absolute top-0 left-1/4 w-96 h-96 bg-purple-600/20 rounded-full blur-3xl" />
        <div className="absolute bottom-0 right-1/4 w-96 h-96 bg-blue-600/20 rounded-full blur-3xl" />
      </div>

      <div className="container mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
        <div className="py-16 md:py-20">
          <div className="grid lg:grid-cols-5 gap-12 mb-12">
            <div className="lg:col-span-2">
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.6 }}
              >
                <div className="flex items-center gap-3 mb-6">
                  <div className="w-12 h-12 bg-gradient-to-br from-purple-600 via-pink-500 to-blue-500 rounded-xl flex items-center justify-center shadow-lg">
                    <Infinity className="text-white w-7 h-7 animate-spiral-rotate" />
                  </div>
                  <span className="text-3xl font-black gradient-text">SpiraChain</span>
                </div>
                <p className="text-base leading-relaxed mb-8 text-slate-300">
                  The world&apos;s first post-quantum semantic blockchain. 
                  Powered by AI, secured by mathematics, accessible to everyone.
                </p>

                <div className="mb-4">
                  <h4 className="font-bold text-white mb-4 flex items-center gap-2">
                    <Mail className="w-5 h-5 text-purple-400" />
                    Stay Updated
                  </h4>
                  <form onSubmit={handleSubscribe} className="relative">
                    <input
                      type="email"
                      value={email}
                      onChange={(e) => setEmail(e.target.value)}
                      placeholder="Enter your email"
                      required
                      className="w-full px-4 py-3 bg-slate-800/50 border border-slate-700 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:border-purple-500 transition-colors"
                    />
                    <motion.button
                      type="submit"
                      whileHover={{ scale: 1.05 }}
                      whileTap={{ scale: 0.95 }}
                      className="absolute right-2 top-1/2 -translate-y-1/2 px-4 py-2 bg-gradient-to-r from-purple-600 to-pink-600 text-white font-bold rounded-lg hover:shadow-lg transition-all"
                    >
                      {subscribed ? (
                        <span className="flex items-center gap-1">
                          <Heart className="w-4 h-4" fill="currentColor" />
                          Thanks!
                        </span>
                      ) : (
                        <Send className="w-4 h-4" />
                      )}
                    </motion.button>
                  </form>
                </div>
              </motion.div>
            </div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6, delay: 0.1 }}
            >
              <h4 className="font-bold text-white mb-6">Resources</h4>
              <ul className="space-y-3">
                {links.resources.map((link) => (
                  <li key={link.name}>
                    <a
                      href={link.href}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="hover:text-white transition inline-flex items-center gap-2 group"
                    >
                      <FileText className="w-4 h-4 group-hover:text-purple-400 transition" />
                      <span>{link.name}</span>
                      <ExternalLink className="w-3 h-3 opacity-0 group-hover:opacity-100 transition" />
                    </a>
                  </li>
                ))}
              </ul>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6, delay: 0.2 }}
            >
              <h4 className="font-bold text-white mb-6">Developers</h4>
              <ul className="space-y-3">
                {links.developers.map((link) => (
                  <li key={link.name}>
                    <a
                      href={link.href}
                      className="hover:text-white transition inline-flex items-center gap-2 group"
                    >
                      <span>{link.name}</span>
                      <ExternalLink className="w-3 h-3 opacity-0 group-hover:opacity-100 transition" />
                    </a>
                  </li>
                ))}
              </ul>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6, delay: 0.3 }}
            >
              <h4 className="font-bold text-white mb-6">Community</h4>
              <ul className="space-y-3 mb-8">
                {links.community.map((link) => {
                  const Icon = link.icon;
                  return (
                    <li key={link.name}>
                      <a
                        href={link.href}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="hover:text-white transition inline-flex items-center gap-2 group"
                      >
                        <Icon className="w-4 h-4 group-hover:text-purple-400 transition" />
                        <span>{link.name}</span>
                      </a>
                    </li>
                  );
                })}
              </ul>

              <h4 className="font-bold text-white mb-4 text-sm">Network Info</h4>
              <ul className="space-y-2 text-xs">
                {links.network.map((item) => (
                  <li key={item.label} className="flex justify-between">
                    <span className="text-slate-500">{item.label}:</span>
                    <span className="font-mono text-purple-400">{item.value}</span>
                  </li>
                ))}
              </ul>
            </motion.div>
          </div>
        </div>

        <div className="border-t border-slate-800 py-8">
          <motion.div
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            viewport={{ once: true }}
            transition={{ duration: 0.6, delay: 0.4 }}
            className="flex flex-col md:flex-row justify-between items-center gap-4"
          >
            <p className="text-sm text-slate-500">
              &copy; 2025 SpiraChain. Licensed under GNU General Public License v3.0
            </p>
            <p className="text-sm flex items-center gap-2">
              <span className="text-slate-500">Built with</span>
              <Heart className="text-red-500 w-4 h-4" fill="currentColor" />
              <span className="text-slate-500">by the SpiraChain community</span>
            </p>
          </motion.div>
        </div>
      </div>
    </footer>
  );
}
