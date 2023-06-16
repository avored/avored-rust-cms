<template>  
    <div class="h-screen w-full flex flex-col items-center justify-center">
        <div class="block">
            <img src="../assets/logo.svg" alt="AvoRed Rust CMS" class="w-24" />
        </div>
        <h2 class="text-red-700 text-2xl font-semibold">AvoRed Rust CMS</h2>
        <div class="max-w-lg mt-5  block">
            <p>Response for home route: {{ result }}</p>
        </div>
    </div>
</template>

<script async setup lang="ts">
import { ref, onMounted } from 'vue'
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../api'

const result = ref()

onMounted(async () => {
    const token = localStorage.getItem('token')
    const response : AxiosResponse<Array<PostType>> = await avoRedRustApi.get(
        '/', 
        {
            headers: {
                "Authorization": `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        });

    result.value = response.data
})



type PostType = {
    id: String,
    title: String,
    body: String,
    created_at: String,
    updated_at: StaticRangeInit
}
</script>
