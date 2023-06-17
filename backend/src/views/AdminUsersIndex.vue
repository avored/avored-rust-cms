<template>
    Admin users
    {{ result }}
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../api'

const result = ref()

onMounted(async () => {
    const token = localStorage.getItem('token')
    const response : AxiosResponse<Array<AdminUserType>> = await avoRedRustApi.get(
        '/api/admin-users', 
        {
            headers: {
                "Authorization": `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        });

    result.value = response.data
})


type AdminUserType = {
    id: String,
    email: String,
    created_at: String,
    updated_at: StaticRangeInit
}

</script>
