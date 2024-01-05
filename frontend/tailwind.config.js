/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{rs,html}',
  ],
  theme: {
    extend: {
      borderRadius: {
        'half': '50%',
      },
      boxShadow: {
        'inner-solid': 'inset 0 0 0 1px transparent',
        'inner-l-solid': 'inset 4px 0 0 0 transparent',
        'inner-b-solid': 'inset 0 -4px 0 0 transparent',
      },
      cursor: {
        'inherit': 'inherit'
      },
      minHeight: {
        '48px': '48px'
      }
    },
  },
  plugins: [],
}