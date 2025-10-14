'use client';

import { motion, useInView, useMotionValue, useSpring } from 'framer-motion';
import { useEffect, useRef } from 'react';

interface StatsCounterProps {
  end: number;
  duration?: number;
  suffix?: string;
  prefix?: string;
  decimals?: number;
  gradient?: string;
  label: string;
  description?: string;
  delay?: number;
}

export default function StatsCounter({
  end,
  duration = 2,
  suffix = '',
  prefix = '',
  decimals = 0,
  gradient = 'from-purple-500 to-pink-500',
  label,
  description,
  delay = 0,
}: StatsCounterProps) {
  const ref = useRef<HTMLDivElement>(null);
  const motionValue = useMotionValue(0);
  const springValue = useSpring(motionValue, { duration: duration * 1000 });
  const isInView = useInView(ref, { once: true, margin: '-100px' });

  useEffect(() => {
    if (isInView) {
      const timeout = setTimeout(() => {
        motionValue.set(end);
      }, delay * 1000);
      return () => clearTimeout(timeout);
    }
  }, [isInView, end, motionValue, delay]);

  useEffect(() => {
    const unsubscribe = springValue.on('change', (latest) => {
      if (ref.current) {
        const value = latest.toFixed(decimals);
        ref.current.textContent = `${prefix}${value}${suffix}`;
      }
    });
    return unsubscribe;
  }, [springValue, decimals, prefix, suffix]);

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.8 }}
      whileInView={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.5, delay }}
      viewport={{ once: true }}
      whileHover={{ scale: 1.05, y: -5 }}
      className="group text-center"
    >
      <div className="relative p-8 glass-card rounded-3xl shadow-xl hover:shadow-2xl transition-all duration-300">
        <div
          ref={ref}
          className={`text-5xl md:text-6xl lg:text-7xl font-black bg-gradient-to-r ${gradient} bg-clip-text text-transparent mb-3 group-hover:scale-110 transition-transform duration-300`}
        >
          {prefix}0{suffix}
        </div>
        
        <div className="text-xl font-bold text-slate-900 dark:text-white mb-2">
          {label}
        </div>
        
        {description && (
          <div className="text-sm text-slate-600 dark:text-slate-400">
            {description}
          </div>
        )}

        <div className={`absolute inset-0 bg-gradient-to-r ${gradient} opacity-0 group-hover:opacity-5 rounded-3xl transition-opacity duration-300 pointer-events-none`} />
      </div>
    </motion.div>
  );
}

