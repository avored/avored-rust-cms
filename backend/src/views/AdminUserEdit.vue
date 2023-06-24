<template>
  <div class="mx-auto max-w-screen-xl mt-3 mb-5">
    <div class="sm:flex sm:items-center sm:justify-between">
      <div class="text-center sm:text-left">
        <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">
          Edit Admin User
        </h1>
      </div>
    </div>
  </div>

  <div class="text-left border-t my-8 text-sm">
    <form @submit.prevent="submit" action="#" class="mt-3 w-full">
      <div class="w-full">
        <label for="Email" class="w-full text-sm font-medium text-gray-700">
          Email
        </label>

        <input type="email" 
          v-model="edit_user_form.email"
          autofocus 
          id="Email" 
          name="email" 
          @blur="validation.email.$touch()"
          class="mt-1 block w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm"
        />
          <div class="text-sm text-red-700" v-if="validation.email.$error && validation.email.required.$invalid">
            {{ validation.email.required.$message }}
          </div>
          <div class="text-sm text-red-700" v-if="validation.email.$error && validation.email.email.$invalid">
            {{ validation.email.email.$message }}
          </div>
      </div>

      <div class="mt-5 flex w-full items-center">
        <router-link class="mr-auto btn btn-default" :to="{name:'admin-users-list'}">
          Cancel
        </router-link>

        <button type="submit" :disabled="!validation.$dirty || validation.$error"
          class="btn btn-primary">
          Edit Admin User
        </button>
        
      </div>
    </form>

  </div>
</template>
<script setup lang="ts">
import useVuelidate from '@vuelidate/core'
import { computed, onMounted, ref } from 'vue'
import { required, email } from '@vuelidate/validators'
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../api'
import { useRouter, useRoute } from 'vue-router'


const router = useRouter()
const route = useRoute()
const edit_user_form = ref<AdminUserType>({email: '', id: ''})
const rules =  computed(() => ({
      email: {
        required,
        email,
        $autoDirty: true
      },
    }))

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

    edit_user_form.value = response.data

})


const validation =  useVuelidate(rules, edit_user_form)

const submit = async () => {

  const data = {email: edit_user_form.value.email}

  const token = localStorage.getItem('token')
  const result: AxiosResponse<AdminUserType> = await avoRedRustApi.put(
      '/api/admin-users', 
      JSON.stringify(data),
      {
        headers: {
          "Authorization": `Bearer ${token}`,
          'Content-Type': 'application/json'
        }
      });

  if (result.data.id) {
      router.push({ name: 'admin-users-list' })
  }
}
interface AdminUserType {
    id: String,
    email: String,
}

</script>
