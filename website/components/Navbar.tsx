'use client';

import { motion, useScroll, useTransform } from 'framer-motion';
import { Sun, Moon, Menu, X, Github, Infinity } from 'lucide-react';
import { useState, useEffect } from 'react';
import { useTheme } from './ThemeProvider';
import GradientButton from './GradientButton';

export default function Navbar() {
  const { effectiveTheme, toggleTheme } = useTheme();
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);
  const { scrollY } = useScroll();
  const navOpacity = useTransform(scrollY, [0, 100], [0.8, 1]);
  const navBlur = useTransform(scrollY, [0, 100], [10, 24]);

  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 50);
    };
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const navItems = [
    { name: 'Features', href: '#features' },
    { name: 'Technology', href: '#technology' },
    { name: 'Tokenomics', href: '#tokenomics' },
    { name: 'Roadmap', href: '#roadmap' },
    { name: 'Get Started', href: '#get-started' },
  ];

  const closeMobileMenu = () => setMobileMenuOpen(false);

  return (
    <>
      <motion.nav
        style={{ 
          opacity: navOpacity,
        }}
        className={`fixed w-full top-0 z-50 transition-all duration-300 ${
          scrolled 
            ? 'bg-white/90 dark:bg-slate-900/90 backdrop-blur-2xl border-b border-slate-200/50 dark:border-slate-800/50 shadow-lg' 
            : 'bg-white/70 dark:bg-slate-900/70 backdrop-blur-xl border-b border-transparent'
        }`}
      >
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16 md:h-20">
            <motion.a
              href="#"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="flex items-center space-x-3 cursor-pointer group"
            >
              <div className="relative w-10 h-10 md:w-12 md:h-12 bg-gradient-to-br from-purple-600 via-pink-500 to-blue-500 rounded-xl flex items-center justify-center shadow-lg group-hover:shadow-xl transition-shadow">
                <Infinity className="text-white w-6 h-6 md:w-7 md:h-7 animate-spiral-rotate" />
              </div>
              <span className="text-xl md:text-2xl font-black gradient-text">SpiraChain</span>
            </motion.a>

            <div className="hidden lg:flex items-center space-x-8">
              {navItems.map((item) => (
                <motion.a
                  key={item.name}
                  href={item.href}
                  className="relative text-slate-700 dark:text-slate-300 hover:text-purple-600 dark:hover:text-purple-400 transition-colors font-semibold text-sm group"
                  whileHover={{ y: -2 }}
                >
                  {item.name}
                  <span className="absolute -bottom-1 left-0 w-0 h-0.5 bg-gradient-to-r from-purple-600 to-pink-500 group-hover:w-full transition-all duration-300" />
                </motion.a>
              ))}
              
              <motion.a
                href="https://github.com/iyotee/SpiraChain"
                target="_blank"
                rel="noopener noreferrer"
                className="text-slate-700 dark:text-slate-300 hover:text-purple-600 dark:hover:text-purple-400 transition-colors"
                whileHover={{ scale: 1.1, rotate: 5 }}
                whileTap={{ scale: 0.9 }}
              >
                <Github className="w-5 h-5" />
              </motion.a>

              <motion.button
                onClick={toggleTheme}
                className="p-2.5 rounded-xl bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
                whileHover={{ scale: 1.1, rotate: 180 }}
                whileTap={{ scale: 0.9 }}
                aria-label="Toggle theme"
              >
                {effectiveTheme === 'dark' ? 
                  <Sun className="w-5 h-5 text-yellow-500" /> : 
                  <Moon className="w-5 h-5 text-slate-700" />
                }
              </motion.button>

              <GradientButton href="#get-started" size="sm">
                Launch App
              </GradientButton>
            </div>

            <div className="lg:hidden flex items-center space-x-3">
              <motion.button
                onClick={toggleTheme}
                className="p-2 rounded-xl bg-slate-100 dark:bg-slate-800 transition-colors"
                whileTap={{ scale: 0.9 }}
              >
                {effectiveTheme === 'dark' ? 
                  <Sun className="w-5 h-5 text-yellow-500" /> : 
                  <Moon className="w-5 h-5 text-slate-700" />
                }
              </motion.button>
              
              <motion.button
                onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
                className="p-2 rounded-xl bg-slate-100 dark:bg-slate-800 transition-colors"
                whileTap={{ scale: 0.9 }}
              >
                {mobileMenuOpen ? 
                  <X className="w-6 h-6 text-slate-700 dark:text-slate-300" /> : 
                  <Menu className="w-6 h-6 text-slate-700 dark:text-slate-300" />
                }
              </motion.button>
            </div>
          </div>
        </div>
      </motion.nav>

      {mobileMenuOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 bg-black/50 backdrop-blur-sm z-40 lg:hidden"
            onClick={closeMobileMenu}
          />
          
          <motion.div
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ type: 'spring', damping: 25, stiffness: 200 }}
            className="fixed right-0 top-0 bottom-0 w-80 bg-white dark:bg-slate-900 shadow-2xl z-50 lg:hidden overflow-y-auto"
          >
            <div className="p-6">
              <div className="flex items-center justify-between mb-8">
                <div className="flex items-center space-x-3">
                  <div className="w-10 h-10 bg-gradient-to-br from-purple-600 via-pink-500 to-blue-500 rounded-xl flex items-center justify-center">
                    <Infinity className="text-white w-6 h-6" />
                  </div>
                  <span className="text-xl font-black gradient-text">SpiraChain</span>
                </div>
                <button
                  onClick={closeMobileMenu}
                  className="p-2 rounded-xl hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
                  aria-label="Close menu"
                >
                  <X className="w-6 h-6 text-slate-700 dark:text-slate-300" />
                </button>
              </div>

              <nav className="space-y-2">
                {navItems.map((item, index) => (
                  <motion.a
                    key={item.name}
                    href={item.href}
                    initial={{ opacity: 0, x: 20 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: index * 0.1 }}
                    className="block px-4 py-3 rounded-xl text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 font-semibold transition-colors"
                    onClick={closeMobileMenu}
                  >
                    {item.name}
                  </motion.a>
                ))}
              </nav>

              <div className="mt-8 pt-8 border-t border-slate-200 dark:border-slate-800 space-y-4">
                <motion.a
                  href="https://github.com/iyotee/SpiraChain"
                  target="_blank"
                  rel="noopener noreferrer"
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: 0.5 }}
                  className="flex items-center space-x-3 px-4 py-3 rounded-xl hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 font-semibold transition-colors"
                  onClick={closeMobileMenu}
                >
                  <Github className="w-5 h-5" />
                  <span>GitHub</span>
                </motion.a>

                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: 0.6 }}
                >
                  <GradientButton href="#get-started" size="md" className="w-full">
                    Launch App
                  </GradientButton>
                </motion.div>
              </div>
            </div>
          </motion.div>
        </>
      )}
    </>
  );
}
