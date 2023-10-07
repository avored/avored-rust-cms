import Alpine from 'alpinejs';
import focus from '@alpinejs/focus'
import feather from 'feather-icons' 
import EasyMDE from 'easymde'
import axios from 'axios'

Alpine.plugin(focus)
 

import MultiSelect from './multi-select'
import FieldBuilder from './field-builder'
import PageBuilder from './page-builder'
import AssetManager from './asset-manager'

Alpine.data('multiselect', MultiSelect)
Alpine.data('field_builder', FieldBuilder)
Alpine.data('page_builder', PageBuilder)
Alpine.data('asset_manager', AssetManager)

window.axios = axios
window.EasyMDE = EasyMDE
window.Alpine = Alpine;
window.Feather = feather;
Alpine.start()

window.addEventListener("load", (event) => {
    Feather.replace()
});