<template>
    <div class="mx-auto max-w-screen-xl mt-3 mb-5">
        <div class="sm:flex sm:items-center sm:justify-between">
            <div class="text-center sm:text-left">
                <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">
                    Create Admin User
                </h1>
            </div>
        </div>
    </div>

    <div class="overflow-x-auto text-left border-t my-8 text-sm">
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
              Create Admin User
            </button>
          </div>
        </form>
       
    </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'

const email = ref('')
const password = ref('')
const submit = () => {
    console.log(email, password)
}
// const showPassword = ref<boolean>(false)
</script>
