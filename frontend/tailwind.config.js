/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{rs,html}',
  ],
  theme: {
    extend: {
      boxShadow: {
        'inner-solid': 'inset 0 0 0 1px transparent'
      }
    },
  },
  plugins: [],
}