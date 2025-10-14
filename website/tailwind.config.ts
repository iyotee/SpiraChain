import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        background: "var(--background)",
        foreground: "var(--foreground)",
        primary: {
          50: '#faf5ff',
          100: '#f3e8ff',
          200: '#e9d5ff',
          300: '#d8b4fe',
          400: '#c084fc',
          500: '#a855f7',
          600: '#9333ea',
          700: '#7e22ce',
          800: '#6b21a8',
          900: '#581c87',
        },
      },
      animation: {
        'spiral-rotate': 'spiral-rotate 20s linear infinite',
        'floating': 'floating 3s ease-in-out infinite',
        'glow': 'glow 2s ease-in-out infinite',
        'pulse-glow': 'pulse-glow 3s ease-in-out infinite',
        'gradient-flow': 'gradient-flow 4s ease infinite',
        'particle-float': 'particle-float 6s ease-in-out infinite',
        'slide-up': 'slide-up 0.6s ease-out',
        'slide-down': 'slide-down 0.6s ease-out',
        'fade-in': 'fade-in 0.8s ease-out',
        'scale-in': 'scale-in 0.5s ease-out',
        'shimmer': 'shimmer 2s linear infinite',
      },
      keyframes: {
        'spiral-rotate': {
          'from': { transform: 'rotate(0deg)' },
          'to': { transform: 'rotate(360deg)' },
        },
        'floating': {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-20px)' },
        },
        'glow': {
          '0%, 100%': { 
            boxShadow: '0 0 20px rgba(168, 85, 247, 0.4), 0 0 40px rgba(168, 85, 247, 0.2)',
            filter: 'brightness(1)',
          },
          '50%': { 
            boxShadow: '0 0 30px rgba(168, 85, 247, 0.6), 0 0 60px rgba(168, 85, 247, 0.4)',
            filter: 'brightness(1.2)',
          },
        },
        'pulse-glow': {
          '0%, 100%': { 
            opacity: '1',
            boxShadow: '0 0 30px rgba(168, 85, 247, 0.5)',
          },
          '50%': { 
            opacity: '0.8',
            boxShadow: '0 0 50px rgba(168, 85, 247, 0.8)',
          },
        },
        'gradient-flow': {
          '0%, 100%': { backgroundPosition: '0% 50%' },
          '50%': { backgroundPosition: '100% 50%' },
        },
        'particle-float': {
          '0%, 100%': { 
            transform: 'translateY(0) translateX(0) rotate(0deg)',
            opacity: '0.2',
          },
          '33%': { 
            transform: 'translateY(-20px) translateX(10px) rotate(120deg)',
            opacity: '0.6',
          },
          '66%': { 
            transform: 'translateY(-10px) translateX(-10px) rotate(240deg)',
            opacity: '0.4',
          },
        },
        'slide-up': {
          'from': { 
            transform: 'translateY(30px)',
            opacity: '0',
          },
          'to': { 
            transform: 'translateY(0)',
            opacity: '1',
          },
        },
        'slide-down': {
          'from': { 
            transform: 'translateY(-30px)',
            opacity: '0',
          },
          'to': { 
            transform: 'translateY(0)',
            opacity: '1',
          },
        },
        'fade-in': {
          'from': { opacity: '0' },
          'to': { opacity: '1' },
        },
        'scale-in': {
          'from': { 
            transform: 'scale(0.9)',
            opacity: '0',
          },
          'to': { 
            transform: 'scale(1)',
            opacity: '1',
          },
        },
        'shimmer': {
          '0%': { transform: 'translateX(-100%)' },
          '100%': { transform: 'translateX(100%)' },
        },
      },
      backgroundSize: {
        '200': '200% 200%',
        '300': '300% 300%',
      },
    },
  },
  plugins: [],
};
export default config;
