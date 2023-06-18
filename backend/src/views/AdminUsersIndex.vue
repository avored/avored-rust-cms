<template>
    <div class="mx-auto max-w-screen-xl mt-3 mb-5">
        <div class="sm:flex sm:items-center sm:justify-between">
            <div class="text-center sm:text-left">
                <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">
                    List of Admin Users
                </h1>
            </div>

            <div class="mt-4 flex flex-col gap-4 sm:mt-0 sm:flex-row sm:items-center">
                <a href="#"
                    class="block rounded-lg bg-red-600 px-5 py-3 text-sm font-medium text-white transition hover:bg-red-700 focus:outline-none focus:ring"
                    type="button">
                    Create Admin User
                </a>
            </div>
        </div>
    </div>

    <div class="overflow-x-auto text-left border-t">
        <table class="min-w-full divide-y-2 divide-gray-200 text-sm">
            <thead class="font-semibold">
                <tr>
                    <th class="px-4 py-2 text-gray-900" v-for="col in columns" :key="`header-col-tr-${col.identifier}`" >
                        {{ col.text }}
                    </th>
                </tr>
            </thead>

            <tbody class="divide-y divide-gray-200">
                <tr  v-for="row in result" :key="`data-col-tr-${row.id}`" >
                    <td class="px-4 py-2 font-medium text-gray-900" v-for="col in columns" :key="`data-col-tr-${col.identifier}`">

                        <template v-if="col.identifier === '__action__'">
                            <span v-html="(col.value === null) ? null  : col.value(row)"></span>
                        </template>
                        <template v-else>
                            <template v-for="(val, colIdentifier) in row" :key="`template-col-${col.identifier}-value-${index}`">
                                <template v-if="String(colIdentifier) == col.identifier">
                                    <span>{{ val }}</span>
                                </template>
                            </template>
                        </template>
                        
                    </td>
                </tr>
            </tbody>
        </table>
    </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../api'

const result = ref()

const columns: Array<ColumnType> = [
    {
        identifier: "id",
        text: "ID",
        value: null
    },
    {
        identifier: "email",
        text: "Email address",
        value: null,
    },
    {
        identifier: "__action__",
        text: "Action",
        value: (row: AdminUserType) => {
            var html = "";

            html += `<a href="/admin-users/${row.id}">Edit</a>`

            return html;
        }
    }
]

onMounted(async () => {
    const token = localStorage.getItem('token')
    const response: AxiosResponse<Array<AdminUserType>> = await avoRedRustApi.get(
        '/api/admin-users',
        {
            headers: {
                "Authorization": `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        });

    result.value = response.data
})

interface ColumnType {
    identifier: String,
    text: String,
    value: Function | null,
}

interface AdminUserType {
    id: String,
    email: String,
    created_at: String,
    updated_at: StaticRangeInit
}

</script>
