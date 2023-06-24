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

  <div class="text-left border-t my-8 text-sm">
    <form @submit.prevent="submit" action="#" class="mt-3 w-full">
      <div class="w-full">
        <label for="Email" class="w-full text-sm font-medium text-gray-700">
          Email
        </label>

        <input type="email" 
          v-model="create_user_form.email"
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

      <div class="mt-5">
        <label for="Password" class="block text-sm font-medium text-gray-700">
          Password
        </label>

        <input type="password" id="Password" v-model="create_user_form.password" name="password"
          @blur="validation.password.$touch()"
          class="mt-1 w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm" />
          <div class="text-sm text-red-700" v-if="validation.password.$error && validation.password.required.$invalid">
            {{ validation.password.required.$message }}
          </div>
      </div>
      <div class="mt-5">
        <label for="Password" class="block text-sm font-medium text-gray-700">
          Password Confirmation
        </label>

        <input type="password" id="Password" v-model="create_user_form.confirm_password" name="confirm_password"
          @blur="validation.confirm_password.$touch()"
          class="mt-1 w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm" />
          <div class="text-sm text-red-700" v-if="validation.confirm_password.$error &&  validation.confirm_password.required.$invalid">
            {{ validation.confirm_password.required.$message }}
          </div>
          <div class="text-sm text-red-700" v-if="validation.confirm_password.$error &&  validation.confirm_password.sameAs.$invalid">
            {{ validation.confirm_password.sameAs.$message }}
          </div>
        </div>

      <div class="mt-5 flex w-full items-center">
        <router-link class="mr-auto btn btn-default" :to="{name:'admin-users-list'}">Cancel</router-link>
        <button type="submit" :disabled="!validation.$dirty || validation.$error"
          class="btn btn-primary">
          Create Admin User
        </button>
        
      </div>
    </form>

  </div>
</template>
<script setup lang="ts">
import useVuelidate from '@vuelidate/core'
import { computed, ref } from 'vue'
import { required, email, sameAs } from '@vuelidate/validators'
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../api'
import { useRouter } from 'vue-router'


const router = useRouter()
const create_user_form = ref<CreateUserForm>({email: '', password: '', confirm_password: ''})
const rules =  computed(() => ({
      email: {
        required,
        email
      },
      password: {
        required,
      },
      confirm_password: {
        required,
        sameAs: sameAs(computed(() => create_user_form.value.password) ),
      },
    }))


const validation =  useVuelidate(rules, create_user_form)

const submit = async () => {
  const data = {email: create_user_form.value.email, password: create_user_form.value.password}

  const token = localStorage.getItem('token')
  const result: AxiosResponse<AdminUserType> = await avoRedRustApi.post(
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
    created_at: String,
    updated_at: StaticRangeInit
}

interface CreateUserForm {
  email: String,
  password: String,
  confirm_password: String
}
</script>
