<template>  
    <div class="flex h-screen">
        <AvoRedSidebar />
        <div class="bg-white flex-1">
            <AvoRedHeader />
            <div class="p-5">
                <router-view></router-view>
            </div>
        </div>
    </div>
</template>

<script async setup lang="ts">
import { ref, onMounted } from 'vue'
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../api'
import AvoRedSidebar from '../components/partials/AvoRedSidebar.vue';
import AvoRedHeader from '../components/partials/AvoRedHeader.vue';

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
