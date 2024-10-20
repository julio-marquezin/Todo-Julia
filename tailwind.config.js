/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
    "./src/pages/**/*.rs",
    "./src/components/**/*.rs",
  ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
