/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      screens: {
        'print': {'raw': 'print'},
      },
    },
  },
  plugins: [],
}
