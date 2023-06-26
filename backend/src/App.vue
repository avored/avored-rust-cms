<template>
  <body class="antialiased">
        <component :is="(router.currentRoute.value.meta.requiresAuth) ? appLayout : guestLayout" />
  </body>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import GuestLayout from './layouts/GuestLayout.vue'
import AppLayout from './layouts/AppLayout.vue'

const router = useRouter()
const appLayout = AppLayout
const guestLayout = GuestLayout;
</script>
