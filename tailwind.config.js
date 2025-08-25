/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        // VoidCat RDC Brand Colors
        voidcat: {
          50: '#f0f4ff',
          100: '#e0e9ff',
          200: '#c7d8ff',
          300: '#a3bfff',
          400: '#7a9bff',
          500: '#5173ff',
          600: '#3e4df7',
          700: '#3338e3',
          800: '#2c2fb8',
          900: '#2a2c91',
          950: '#1c1c57',
        },
        forbidden: {
          50: '#fef7f0',
          100: '#fdeadb',
          200: '#fad2b7',
          300: '#f6b189',
          400: '#f08659',
          500: '#ec6434',
          600: '#dd4a24',
          700: '#b73520',
          800: '#922c22',
          900: '#76271f',
          950: '#40110e',
        }
      },
      fontFamily: {
        'mono': ['Fira Code', 'Monaco', 'Cascadia Code', 'Roboto Mono', 'monospace'],
        'sans': ['Inter', 'system-ui', 'sans-serif'],
      },
      animation: {
        'fade-in': 'fadeIn 0.2s ease-in-out',
        'slide-up': 'slideUp 0.3s ease-out',
        'pulse-glow': 'pulseGlow 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        pulseGlow: {
          '0%, 100%': {
            boxShadow: '0 0 5px rgba(81, 115, 255, 0.5)',
            transform: 'scale(1)'
          },
          '50%': {
            boxShadow: '0 0 20px rgba(81, 115, 255, 0.8)',
            transform: 'scale(1.05)'
          },
        }
      },
      boxShadow: {
        'forbidden': '0 4px 14px 0 rgba(236, 100, 52, 0.25)',
        'voidcat': '0 4px 14px 0 rgba(81, 115, 255, 0.25)',
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
  ],
  darkMode: 'class',
}
