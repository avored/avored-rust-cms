import Alpine from 'alpinejs';
import focus from '@alpinejs/focus'
 
Alpine.plugin(focus)
 

import MultiSelect from './multi-select'

Alpine.data('multiselect', MultiSelect)

window.Alpine = Alpine;


Alpine.start();
