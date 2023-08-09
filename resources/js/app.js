import Alpine from 'alpinejs';
import focus from '@alpinejs/focus'
import feather from 'feather-icons' 

Alpine.plugin(focus)
 

import MultiSelect from './multi-select'

Alpine.data('multiselect', MultiSelect)

window.Alpine = Alpine;


Alpine.start();

feather.replace()
