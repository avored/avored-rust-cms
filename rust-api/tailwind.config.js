/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')

module.exports = {
  content: ["./views/**/*.hbs"],
  theme: {
    extend: {
      colors: {
        primary: colors.purple
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms')
  ],
}

