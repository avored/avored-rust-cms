import Alpine from 'alpinejs';
import focus from '@alpinejs/focus'
import feather from 'feather-icons' 

Alpine.plugin(focus)
 

import MultiSelect from './multi-select'
import FieldBuilder from './field-builder'

Alpine.data('multiselect', MultiSelect)
Alpine.data('field_builder', FieldBuilder)

window.Alpine = Alpine;
window.feather = feather;
Alpine.start()

window.addEventListener("load", (event) => {
    feather.replace()
});