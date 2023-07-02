<template>
  <div class="mx-auto max-w-screen-xl mt-3 mb-5">
    <div class="sm:flex sm:items-center sm:justify-between">
      <div class="text-center sm:text-left">
        <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">
          Show Admin User
        </h1>
      </div>
    </div>
  </div>

  <div class="text-left border-t my-8 text-sm">
    <form @submit.prevent="submit" action="#" class="mt-3 w-full">
      <div class="flex items-center">
        <label for="Email" class="text-gray-700">
          Email:
        </label>
        <div class="ml-5">
          {{ show_user_form.email }}

        </div>

      </div>

      <div class="mt-5 flex w-full items-center">
        <router-link class="mr-auto btn btn-default" :to="{name:'admin-users-list'}">
          Cancel
        </router-link>

        <button type="submit" class="btn btn-primary">
          Delete Admin User
        </button>
        
      </div>
    </form>

  </div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../api'
import { useRoute, useRouter } from 'vue-router'


const router = useRouter()
const route = useRoute()
const show_user_form = ref<AdminUserType>({email: '', id: ''})

  onMounted(async () => {

    const admin_user_id = route.params.admin_user_id
    const token = localStorage.getItem('token')
    const response: AxiosResponse<AdminUserType> = await avoRedRustApi.get(
        '/api/admin-users/' + admin_user_id,
        {
            headers: {
                "Authorization": `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        })
        console.log(response)
    show_user_form.value = response.data

})

const submit = async () => {

  const token = localStorage.getItem('token')
  const result: AxiosResponse<AdminUserType> = await avoRedRustApi.delete(
      '/api/admin-users/' + show_user_form.value.id, 
      {
        headers: {
          "Authorization": `Bearer ${token}`,
          'Content-Type': 'application/json'
        }
      });

  if (result.status === 204) {
      router.push({ name: 'admin-users-list' })
  }
}
interface AdminUserType {
    id: String,
    email: String,
}

</script>
