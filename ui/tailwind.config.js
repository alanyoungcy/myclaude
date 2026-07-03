/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        background: '#0f0f0f',
        surface: '#1a1a1a',
        surfaceHover: '#252525',
        border: '#2a2a2a',
        text: '#e5e5e5',
        textSecondary: '#a0a0a0',
        primary: '#8b5cf6',
        primaryHover: '#7c3aed',
      },
    },
  },
  plugins: [],
}
