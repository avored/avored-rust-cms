<template>
  <body class="antialiased">
    <component :is="layout" />
  </body>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import GuestLayout from './layouts/GuestLayout.vue'
import AppLayout from './layouts/AppLayout.vue'

const route = useRoute()
var layout = AppLayout;

console.log(route.meta.requiresAuth === false)
if (route.meta.requiresAuth === false) {
  layout = GuestLayout
}
</script>
