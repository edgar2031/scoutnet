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
        // Backgrounds — Cardinal spec
        bg:           '#0a0a0f',
        'bg-2':       '#0f0f1a',
        surface:      '#0f0f1a',
        'surface-2':  '#12121e',
        'surface-3':  '#16162a',
        // Borders
        border:       '#1e1e30',
        'border-2':   '#2d2d4a',
        'border-subtle': 'rgba(255,255,255,0.04)',
        // Typography
        text:         '#e2e8f0',
        'text-dim':   '#94a3b8',
        'text-muted': '#64748b',
        'text-faint': '#334155',
        // Purple accent — Cardinal main
        accent:       '#8b5cf6',
        'accent-dim': '#7c3aed',
        'accent-3':   '#9d5cf6',
        // Status / semantic
        'accent-2':   '#ef4444',
        yellow:       '#f97316',
        orange:       '#f97316',
        purple:       '#8b5cf6',
        pink:         '#f472b6',
        cyan:         '#22d3ee',
        // Priority tags
        'tag-high':   '#ef4444',
        'tag-mid':    '#f97316',
        'tag-low':    '#3b82f6',
        // Sources
        'src-tg':     '#2ca5e0',
        'src-thread': '#ffffff',
        'src-kanal':  '#f97316',
        'src-bot':    '#22c55e',
        'src-name':   '#6366f1',
      },

      fontFamily: {
        sans: ['Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
        mono: ['Share Tech Mono', 'JetBrains Mono', 'ui-monospace', 'monospace'],
      },

      fontSize: {
        '2xs': ['9px', { lineHeight: '1.4', letterSpacing: '0.2em' }],
        'xs':  ['11px', { lineHeight: '1.4', letterSpacing: '0.1em' }],
      },

      letterSpacing: {
        'ui':   '0.1em',
        'wide': '0.15em',
        'wider':'0.2em',
      },

      borderRadius: {
        card:   '6px',
        node:   '8px',
        panel:  '6px',
        pill:   '9999px',
        tag:    '3px',
      },

      boxShadow: {
        card:          '0 1px 3px rgba(0,0,0,0.5)',
        'card-hover':  '0 4px 16px rgba(0,0,0,0.6)',
        'glow-sm':     '0 0 8px rgba(139,92,246,0.3)',
        'glow':        '0 0 20px rgba(139,92,246,0.4), 0 0 40px rgba(139,92,246,0.2)',
        'glow-lg':     '0 0 20px rgba(139,92,246,0.4), 0 0 40px rgba(139,92,246,0.2), 0 0 80px rgba(139,92,246,0.1)',
        'glow-red':    '0 0 12px rgba(239,68,68,0.15)',
        'glow-orange': '0 0 12px rgba(249,115,22,0.15)',
        'glow-blue':   '0 0 12px rgba(59,130,246,0.15)',
        'focus':       '0 0 0 2px rgba(139,92,246,0.2)',
      },

      backgroundImage: {
        'gradient-accent':   'linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%)',
        'gradient-surface':  'linear-gradient(180deg, #0f0f1a 0%, #0a0a0f 100%)',
      },

      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4,0,0.6,1) infinite',
        'glow-pulse': 'glow-pulse 3s ease-in-out infinite',
        'slide-up':   'slide-up 0.2s ease-out',
        'fade-in':    'fade-in 0.15s ease-out',
      },

      keyframes: {
        'glow-pulse': {
          '0%, 100%': { boxShadow: '0 0 20px rgba(139,92,246,0.3)' },
          '50%':      { boxShadow: '0 0 40px rgba(139,92,246,0.6), 0 0 80px rgba(139,92,246,0.3)' },
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
