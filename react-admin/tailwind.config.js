/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
        './node_modules/preline/preline.js',
    ],
    theme: {
        extend: {
            colors: {
                primary: require('tailwindcss/colors').purple
            }
        },
    },
    plugins: [
        require('preline/plugin'),
        require('@tailwindcss/forms')
    ],
}

