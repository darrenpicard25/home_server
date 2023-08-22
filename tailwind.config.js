/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./app/src/**/*.rs"],
  },
  theme: {
    colors: {
      color1: "#8ecae6",
      color2: "#219ebc",
      color3: "#023047",
      color4: "#ffb703",
      color5: "#fb8500",
    },
    extend: {},
  },
  plugins: [],
};
