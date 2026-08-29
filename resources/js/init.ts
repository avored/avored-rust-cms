import Alpine from 'alpinejs'
import focus from '@alpinejs/focus'
import feather from 'feather-icons'

declare global {
  interface Window {
    Alpine: typeof Alpine
    feather: typeof feather
  }
}

window.feather = feather
window.Alpine = Alpine
window.Alpine.plugin(focus)

