/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "*.html",
    "./src/**/*.rs",
    "./*/src/**/*.rs",
    "./app/src/{components,layouts,pages}/*.rs",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
