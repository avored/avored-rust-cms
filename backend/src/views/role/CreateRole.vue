<template>
  <div class="mx-auto max-w-screen-xl mt-3 mb-5">
    <div class="sm:flex sm:items-center sm:justify-between">
      <div class="text-center sm:text-left">
        <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">
          Create Role
        </h1>
      </div>
    </div>
  </div>

  <div class="text-left border-t my-8 text-sm">
    <form @submit.prevent="submit" action="#" class="mt-3 w-full">
      <div class="w-full">
        <label for="name" class="w-full text-sm font-medium text-gray-700">
          Name
        </label>

        <input type="text" 
          v-model="create_role_form.name"
          autofocus 
          id="name" 
          @blur="validation.name.$touch()"
          class="mt-1 block w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm"
        />
          <div class="text-sm text-red-700" v-if="validation.name.$error && validation.name.required.$invalid">
            {{ validation.name.required.$message }}
          </div>
      </div>

      <div class="mt-5">
        <label for="description" class="block text-sm font-medium text-gray-700">
          Description
        </label>

        <input type="text" id="Password" v-model="create_role_form.description"
          class="mt-1 w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm" />
          
      </div>
     
      <div class="mt-5 flex w-full items-center">
        <router-link class="mr-auto btn btn-default" :to="{name:'role-list'}">Cancel</router-link>
        <button type="submit" :disabled="!validation.$dirty || validation.$error"
          class="btn btn-primary">
          Create Role
        </button>
        
      </div>
    </form>

  </div>
</template>
<script setup lang="ts">
import useVuelidate from '@vuelidate/core'
import { computed, ref } from 'vue'
import { required } from '@vuelidate/validators'
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../../api'
import { useRouter } from 'vue-router'


const router = useRouter()
const create_role_form = ref<CreateRoleForm>({name: '', description: ''})
const rules =  computed(() => ({
      name: {
        required
      }
    }))


const validation =  useVuelidate(rules, create_role_form)

const submit = async () => {
  const data = {name: create_role_form.value.name, description: create_role_form.value.description}

  const token = localStorage.getItem('token')
  const result: AxiosResponse<RoleType> = await avoRedRustApi.post(
      '/api/role', 
      JSON.stringify(data),
      {
        headers: {
          "Authorization": `Bearer ${token}`,
          'Content-Type': 'application/json'
        }
      });
  
  console.log(result.data )
  if (result.data.id) {
      // router.push({ name: 'role-list' })
  }
}
interface RoleType {
    id: String,
    name: String,
    password: String,
    created_at: Date,
    updated_at: Date,
    created_by: String,
    updated_by: String
}
interface CreateRoleForm {
  name: String,
  description: String,
}
</script>
