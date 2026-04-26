import type { Config } from 'tailwindcss'

/**
 * LEAD.HUNTER design tokens — single source of truth.
 * Never use hex values in templates; always reference these tokens.
 */
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
        bg:           '#0a0e12',
        'bg-2':       '#0f1419',
        surface:      '#151b22',
        'surface-2':  '#1a2129',
        border:       '#1e2a35',
        text:         '#c9d1d9',
        'text-dim':   '#6e7681',
        accent:       '#00ff9c',
        'accent-2':   '#ff3864',
        'accent-3':   '#00d4ff',
        yellow:       '#ffb800',
        orange:       '#ff8c42',
        purple:       '#a855f7',
        pink:         '#ff4d9d',
      },
      fontFamily: {
        mono: ['JetBrains Mono', 'monospace'],
        sans: ['Rajdhani', 'sans-serif'],
      },
    },
  },
  plugins: [],
} satisfies Config
