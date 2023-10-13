/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");

module.exports = {
  content: {
    files: ["*.html", "./app/src/**/*.rs"],
  },
  theme: {
    colors: colors,
    extend: {},
  },
  plugins: [],
};
