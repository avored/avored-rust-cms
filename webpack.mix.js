let mix = require('laravel-mix');


mix.js('resources/js/app.js', 'public/js/app.js').setPublicPath('public');

mix.postCss("resources/css/app.css", "public/css", [
    require("tailwindcss"),
  ]);
