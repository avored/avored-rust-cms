import axios, { InternalAxiosRequestConfig } from 'axios'
import Alpine from 'alpinejs'
import focus from '@alpinejs/focus'

declare global {
  interface Window {
    Alpine: typeof Alpine
    axios: typeof axios
  }
}

window.axios = axios
window.axios.defaults.headers.common['X-Requested-With'] = 'XMLHttpRequest';

window.axios.interceptors.request.use(function (config: InternalAxiosRequestConfig) {
  const token = localStorage.getItem("avored_admin_token");
  const token_without_quote = token?.replaceAll("\"", '');
  if (token) {
    config.headers.Authorization = `Bearer ${token_without_quote}`;
  }
  return config;
});

window.Alpine = Alpine
window.Alpine.plugin(focus)
