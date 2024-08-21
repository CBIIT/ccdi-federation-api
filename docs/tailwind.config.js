/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './.vitepress/**/*.js',
    './.vitepress/**/*.ts',
    './.vitepress/**/*.vue',
    './blog/**/*.js',
    './blog/**/*.ts',
    './blog/**/*.vue',
    './**/*.md',
    '!./node_modules'
  ],
  darkMode: 'selector',
  theme: {
    extend: {},
  },
  plugins: [],
}
