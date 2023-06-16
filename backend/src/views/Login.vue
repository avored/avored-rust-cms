<template>
<section class="bg-white h-screen flex items-center max-w-lg mx-auto">
  <div class="flex items-center justify-center flex-col mx-auto shadow py-12 p-5 w-full">
      <div class="flex w-full justify-center">
        <a class="" href="/">
          <img src="../assets/logo.svg" class="w-16" />
        </a>
        <h1 class="mt-3 text-xl font-bold text-red-700">
            AvoRed Rust CMS
        </h1>
    </div>
    <div class="flex w-full">
        <form @submit.prevent="submit" action="#" class="mt-3 w-full">
          <div class="w-full">
            <label for="Email" class="w-full text-sm font-medium text-gray-700">
              Email
            </label>

            <input
              type="email"
              v-model="email"
              autofocus
              id="Email"
              name="email"
              class="mt-1 block w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm"
            />
          </div>

          <div class="mt-5">
            <label
              for="Password"
              class="block text-sm font-medium text-gray-700"
            >
              Password
            </label>

            <input
              type="password"
              id="Password"
              v-model="password"
              name="password"
              class="mt-1 w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm"
            />
          </div>

          <div class="mt-5 flex w-full items-center">
            <button
              type="submit"
              class="shrink-0 rounded-md border border-blue-600 bg-blue-600 px-8 py-3 text-sm font-medium text-white transition hover:bg-transparent hover:text-blue-600 focus:outline-none focus:ring active:text-blue-500"
            >
              Login
            </button>

            <p class="ml-auto text-sm text-gray-500">
              <a href="#" class="text-gray-700 underline">forgot your password?</a>.
            </p>
          </div>
        </form>
      </div>
  </div>
</section>
</template>


<script setup lang="ts">
import { AxiosResponse } from 'axios'
import { ref } from 'vue'
import type { Ref } from 'vue'
import avoredRustApi from '../api'
import { useRouter } from 'vue-router';


type AuthResponseType = {
  status: string,
  token: string
}

const router = useRouter()
const email: Ref<string> = ref('admin@admin.com')
const password: Ref<string> = ref('admin123')

const submit = async () => {
  
  const data = {email: email.value, password: password.value}
  
  const result: AxiosResponse<AuthResponseType> = await avoredRustApi.post(
      '/api/auth/login', 
      JSON.stringify(data),
      {
        headers: {
          'Content-Type': 'application/json'
        }
      });

  if (result.data.token) {
      localStorage.setItem('is_logged_in', 'true')
      localStorage.setItem('token', result.data.token)
      router.push({ path: '/' })

  }
}


</script>
