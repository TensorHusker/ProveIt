/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'assumption-blue': '#3b82f6',
        'rule-green': '#10b981',
        'goal-orange': '#f59e0b',
      },
    },
  },
  plugins: [],
}
