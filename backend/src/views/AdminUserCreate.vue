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
        <button type="submit"
          class="shrink-0 rounded-md border border-blue-600 bg-blue-600 px-8 py-3 text-sm font-medium text-white transition hover:bg-transparent hover:text-blue-600 focus:outline-none focus:ring active:text-blue-500">
          Create Admin User
        </button>
      </div>
    </form>

  </div>
</template>
<script setup lang="ts">
import { computed, ref } from 'vue'
import { useVuelidate } from '@vuelidate/core'
import { required, email, sameAs } from '@vuelidate/validators'

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
console.log(validation.value)

const submit = () => {
  console.log(create_user_form.value)
}

interface CreateUserForm {
  email: String,
  password: String,
  confirm_password: String
}
</script>
