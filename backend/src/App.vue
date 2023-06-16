<template>
  <body class="antialiased">
      <router-view></router-view>
  </body>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute()

if (route.meta.requiresAuth) {
  console.log('dashboard layout')

} else {
  console.log('guest layout')
}
</script>
