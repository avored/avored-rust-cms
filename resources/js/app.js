import Alpine from 'alpinejs';
import focus from '@alpinejs/focus'
import feather from 'feather-icons' 
import EasyMDE from 'easymde'

Alpine.plugin(focus)
 

import MultiSelect from './multi-select'
import FieldBuilder from './field-builder'
import PageBuilder from './page-builder'

Alpine.data('multiselect', MultiSelect)
Alpine.data('field_builder', FieldBuilder)
Alpine.data('page_builder', PageBuilder)

window.EasyMDE = EasyMDE
window.Alpine = Alpine;
window.Feather = feather;
Alpine.start()

window.addEventListener("load", (event) => {
    Feather.replace()
});