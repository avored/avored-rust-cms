<template>
  <div class="mx-auto max-w-screen-xl mt-3 mb-5">
    <div class="sm:flex sm:items-center sm:justify-between">
      <div class="text-center sm:text-left">
        <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">
          Show Role
        </h1>
      </div>
    </div>
  </div>

  <div class="text-left border-t my-8 text-sm">
    <form @submit.prevent="submit" action="#" class="mt-3 w-full">
      
      <div class="flex items-center">
        <label for="name" class="text-gray-700">
          Name:
        </label>
        <div class="ml-5">
          {{ show_form.name }}
        </div>
      </div>
      <div class="flex items-center">
        <label for="description" class="text-gray-700">
          Description:
        </label>
        <div class="ml-5">
          {{ show_form.description }}
        </div>
      </div>

      <div class="mt-5 flex w-full items-center">
        <router-link class="mr-auto btn btn-default" :to="{name:'role-list'}">
          Cancel
        </router-link>

        <button type="submit" class="btn btn-primary">
          Delete Role
        </button>
        
      </div>
    </form>

  </div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../../api'
import { useRoute, useRouter } from 'vue-router'


const router = useRouter()
const route = useRoute()
const show_form = ref<RoleType>({name: '', description: '',id: ''})

  onMounted(async () => {

    const role_id = route.params.role_id
    const token = localStorage.getItem('token')
    const response: AxiosResponse<RoleType> = await avoRedRustApi.get(
        '/api/role/' + role_id,
        {
            headers: {
                "Authorization": `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        })
        console.log(response)
    show_form.value = response.data

})

const submit = async () => {

  const token = localStorage.getItem('token')
  const result: AxiosResponse<RoleType> = await avoRedRustApi.delete(
      '/api/role/' + show_form.value.id, 
      {
        headers: {
          "Authorization": `Bearer ${token}`,
          'Content-Type': 'application/json'
        }
      });

  if (result.status === 204) {
      router.push({ name: 'role-list' })
  }
}
interface RoleType {
    id: String,
    name: String,
    description: String
}

</script>
