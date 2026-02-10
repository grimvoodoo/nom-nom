/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./assets/**/*.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require("daisyui"),
  ],
  daisyui: {
    themes: ["dark", "light"],
    darkTheme: "dark",
  },
}
