'use client';

import { motion } from 'framer-motion';
import { LucideIcon } from 'lucide-react';

interface FeatureCardProps {
  icon: LucideIcon;
  title: string;
  description: string;
  gradient?: string;
  delay?: number;
}

export default function FeatureCard({
  icon: Icon,
  title,
  description,
  gradient = 'from-purple-500 to-pink-500',
  delay = 0,
}: FeatureCardProps) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 30, scale: 0.95 }}
      whileInView={{ opacity: 1, y: 0, scale: 1 }}
      transition={{ duration: 0.5, delay }}
      viewport={{ once: true }}
      whileHover={{ y: -8, scale: 1.02 }}
      className="group relative"
    >
      <div className="relative h-full p-8 glass-card rounded-3xl shadow-xl hover:shadow-2xl transition-all duration-300">
        <div className={`inline-flex p-4 rounded-2xl bg-gradient-to-r ${gradient} shadow-lg mb-6 group-hover:scale-110 transition-transform duration-300`}>
          <Icon className="w-8 h-8 text-white" />
        </div>
        
        <h3 className="text-2xl font-black text-slate-900 dark:text-white mb-4">
          {title}
        </h3>
        
        <p className="text-slate-600 dark:text-slate-400 leading-relaxed">
          {description}
        </p>

        <div className={`absolute inset-0 bg-gradient-to-r ${gradient} opacity-0 group-hover:opacity-5 rounded-3xl transition-opacity duration-300 pointer-events-none`} />
      </div>

      <div className={`absolute -inset-1 bg-gradient-to-r ${gradient} rounded-3xl blur-xl opacity-0 group-hover:opacity-30 transition-opacity duration-300 -z-10`} />
    </motion.div>
  );
}

