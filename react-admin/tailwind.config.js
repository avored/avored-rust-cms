/** @type {import('tailwindcss').Config} */



module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
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

