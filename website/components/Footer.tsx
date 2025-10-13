'use client';

import { motion } from 'framer-motion';
import { Infinity, Github, MessageCircle, Twitter, FileText, Heart } from 'lucide-react';

export default function Footer() {
  const links = {
    resources: [
      { name: 'Whitepaper', href: 'https://github.com/iyotee/SpiraChain/blob/main/WHITEPAPER_VALIDATION.md' },
      { name: 'Roadmap', href: 'https://github.com/iyotee/SpiraChain/blob/main/ROADMAP.md' },
      { name: 'Performance Analysis', href: 'https://github.com/iyotee/SpiraChain/blob/main/PERFORMANCE_ANALYSIS.md' },
      { name: 'GitHub', href: 'https://github.com/iyotee/SpiraChain' },
    ],
    community: [
      { name: 'Discord', href: '#', icon: MessageCircle },
      { name: 'Telegram', href: '#', icon: MessageCircle },
      { name: 'Twitter', href: '#', icon: Twitter },
      { name: 'GitHub', href: 'https://github.com/iyotee/SpiraChain', icon: Github },
    ],
    network: [
      { label: 'Bootstrap', value: '51.154.64.38:9000' },
      { label: 'RPC Port', value: '9933' },
      { label: 'DNS', value: 'bootstrap.spirachain.org' },
      { label: 'Chain ID', value: '7529' },
    ],
  };

  return (
    <footer className="py-16 bg-slate-900 text-slate-400">
      <div className="container mx-auto px-6">
        <div className="grid md:grid-cols-4 gap-12 mb-12">
          {/* Brand */}
          <div>
            <div className="flex items-center space-x-2 mb-4">
              <div className="w-10 h-10 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-xl flex items-center justify-center">
                <Infinity className="text-white" size={20} />
              </div>
              <span className="text-2xl font-black text-white">SpiraChain</span>
            </div>
            <p className="text-sm leading-relaxed">
              The world's first post-quantum semantic blockchain. 
              Powered by AI, secured by mathematics.
            </p>
          </div>

          {/* Resources */}
          <div>
            <h4 className="font-bold text-white mb-4">Resources</h4>
            <ul className="space-y-3">
              {links.resources.map((link) => (
                <li key={link.name}>
                  <a
                    href={link.href}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hover:text-white transition text-sm inline-flex items-center space-x-2 group"
                  >
                    <FileText size={16} className="group-hover:text-indigo-400 transition" />
                    <span>{link.name}</span>
                  </a>
                </li>
              ))}
            </ul>
          </div>

          {/* Community */}
          <div>
            <h4 className="font-bold text-white mb-4">Community</h4>
            <ul className="space-y-3">
              {links.community.map((link) => {
                const Icon = link.icon;
                return (
                  <li key={link.name}>
                    <a
                      href={link.href}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="hover:text-white transition text-sm inline-flex items-center space-x-2 group"
                    >
                      <Icon size={16} className="group-hover:text-indigo-400 transition" />
                      <span>{link.name}</span>
                    </a>
                  </li>
                );
              })}
            </ul>
          </div>

          {/* Network */}
          <div>
            <h4 className="font-bold text-white mb-4">Network</h4>
            <ul className="space-y-3 text-sm">
              {links.network.map((item) => (
                <li key={item.label} className="flex justify-between">
                  <span>{item.label}:</span>
                  <span className="font-mono text-indigo-400">{item.value}</span>
                </li>
              ))}
            </ul>
          </div>
        </div>

        {/* Bottom Bar */}
        <div className="border-t border-slate-800 pt-8">
          <div className="flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
            <p className="text-sm">
              &copy; 2025 SpiraChain. Licensed under GNU General Public License v3.0
            </p>
            <p className="text-sm flex items-center space-x-2">
              <span>Built with</span>
              <Heart className="text-red-500" size={16} fill="currentColor" />
              <span>by the SpiraChain community</span>
            </p>
          </div>
        </div>
      </div>
    </footer>
  );
}

