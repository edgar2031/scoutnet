import type { Config } from 'tailwindcss'

export default {
  darkMode: 'class',
  content: [
    './pages/**/*.{vue,ts}',
    './components/**/*.{vue,ts}',
    './composables/**/*.ts',
    './layouts/**/*.vue',
    './app.vue',
  ],
  theme: {
    extend: {
      colors: {
        // Base — cold dark navy, Cardinal-style
        bg:           '#080b14',
        'bg-2':       '#0c1019',
        // Card surfaces — blue-grey tint
        surface:      '#0f1320',
        'surface-2':  '#151929',
        'surface-3':  '#1c2035',
        // Borders — cool grey-blue
        border:       '#1e2640',
        'border-2':   '#2a3352',
        'border-subtle': 'rgba(140,160,200,0.08)',
        // Typography
        text:         '#e8ecf5',
        'text-dim':   '#6b7394',
        'text-muted': '#3d4563',
        // Primary accent — warm red-orange (Cardinal HIGH badge)
        accent:       '#f97316',
        'accent-dim': '#ea6a0a',
        // Status / semantic
        'accent-2':   '#ef4444',
        'accent-3':   '#facc15',
        yellow:       '#facc15',
        orange:       '#f97316',
        purple:       '#a78bfa',
        pink:         '#f472b6',
      },

      fontFamily: {
        sans: ['Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'ui-monospace', 'monospace'],
      },

      fontSize: {
        '2xs': ['10px', { lineHeight: '14px', letterSpacing: '0.04em' }],
      },

      borderRadius: {
        card:   '6px',
        panel:  '10px',
        pill:   '9999px',
      },

      boxShadow: {
        card:          '0 1px 3px rgba(0,0,0,0.5), 0 4px 20px rgba(0,0,0,0.4)',
        'card-md':     '0 2px 8px rgba(0,0,0,0.5), 0 8px 40px rgba(0,0,0,0.5)',
        'card-hover':  '0 8px 40px rgba(0,0,0,0.6), 0 0 0 1px rgba(140,160,200,0.08)',
        // Accent glow
        'glow-sm':     '0 0 12px rgba(249,115,22,0.25)',
        'glow':        '0 0 24px rgba(249,115,22,0.20), 0 0 48px rgba(249,115,22,0.08)',
        'glow-red':    '0 0 20px rgba(239,68,68,0.25)',
        'glow-cyan':   '0 0 20px rgba(250,204,21,0.18)',
        'inset-accent':'inset 0 0 0 1px rgba(249,115,22,0.12)',
        'focus':       '0 0 0 3px rgba(249,115,22,0.12)',
      },

      backgroundImage: {
        'gradient-accent':   'linear-gradient(135deg, #f97316 0%, #ef4444 100%)',
        'gradient-accent-v': 'linear-gradient(180deg, #f97316 0%, #ef4444 100%)',
        'gradient-surface':  'linear-gradient(180deg, #0f1320 0%, #080b14 100%)',
        'glow-orange': 'radial-gradient(ellipse 60% 50% at 20% 0%, rgba(249,115,22,0.06) 0%, transparent 70%)',
        'glow-red':    'radial-gradient(ellipse 50% 40% at 80% 100%, rgba(239,68,68,0.04) 0%, transparent 70%)',
      },

      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4,0,0.6,1) infinite',
        'float':      'float 6s ease-in-out infinite',
        'glow-pulse': 'glow-pulse 2s ease-in-out infinite',
        'slide-up':   'slide-up 0.2s ease-out',
        'fade-in':    'fade-in 0.15s ease-out',
      },

      keyframes: {
        'float': {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%':      { transform: 'translateY(-6px)' },
        },
        'glow-pulse': {
          '0%, 100%': { boxShadow: '0 0 8px rgba(249,115,22,0.2)' },
          '50%':      { boxShadow: '0 0 24px rgba(249,115,22,0.5), 0 0 48px rgba(249,115,22,0.2)' },
        },
        'slide-up': {
          from: { opacity: '0', transform: 'translateY(6px)' },
          to:   { opacity: '1', transform: 'translateY(0)' },
        },
        'fade-in': {
          from: { opacity: '0' },
          to:   { opacity: '1' },
        },
      },

      spacing: {
        '18': '4.5rem',
        '22': '5.5rem',
      },
    },
  },
  plugins: [],
} satisfies Config
