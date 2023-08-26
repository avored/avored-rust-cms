import Alpine from 'alpinejs';
import focus from '@alpinejs/focus'
import feather from 'feather-icons' 

Alpine.plugin(focus)
 

import MultiSelect from './multi-select'
import FieldBuilder from './field-builder'

Alpine.data('multiselect', MultiSelect)
Alpine.data('field_builder', FieldBuilder)

window.Alpine = Alpine;

Alpine.start();


window.document.addEventListener("DOMContentLoaded", function() {
    feather.replace()
});

