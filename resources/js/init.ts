import Alpine from 'alpinejs'
import focus from '@alpinejs/focus'

declare global {
  interface Window {
    Alpine: typeof Alpine
  }
}

window.Alpine = Alpine
window.Alpine.plugin(focus)
