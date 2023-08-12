import Alpine from 'alpinejs';
import focus from '@alpinejs/focus'
import feather from 'feather-icons' 

Alpine.plugin(focus)
 

import MultiSelect from './multi-select'
import PageBuilder from './page-builder'

Alpine.data('multiselect', MultiSelect)
Alpine.data('page_builder', PageBuilder)

window.Alpine = Alpine;


Alpine.start();

feather.replace()
