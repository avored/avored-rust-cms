/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: require('tailwindcss/colors').purple
      }
    },
  },
  plugins: [require('@tailwindcss/forms')],
}

