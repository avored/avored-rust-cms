<template>
    <div class="mx-auto max-w-screen-xl mt-3 mb-5">
        <div class="sm:flex sm:items-center sm:justify-between">
            <div class="text-center sm:text-left">
                <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">
                    List of Admin Users
                </h1>
            </div>

            <div class="mt-4 flex flex-col gap-4 sm:mt-0 sm:flex-row sm:items-center">
                <router-link :to="{name: 'admin-user-create'}"
                    class="block rounded-lg bg-red-600 px-5 py-3 text-sm font-medium text-white transition hover:bg-red-700 focus:outline-none focus:ring"
                    type="button">
                    Create Admin User
                </router-link>
            </div>
        </div>
    </div>

    <div class="overflow-x-auto text-left border-t">
        <table class="min-w-full divide-y-2 divide-gray-200 text-sm">
            <thead class="font-semibold">
                <tr>
                    <th class="px-4 py-2 text-gray-900" v-for="col in columns" :key="`header-col-tr-${col.identifier}`">
                        {{ col.text }}
                    </th>
                </tr>
            </thead>

            <tbody class="divide-y divide-gray-200">
                <tr v-for="row in rows" :key="`data-col-tr-${row.id}`">
                    <td class="px-4 py-2 font-medium text-gray-900" v-for="col in columns"
                        :key="`data-col-tr-${col.identifier}`">

                        <template v-if="col.identifier === '__action__'">
                            <span v-html="(col.value === null) ? null : col.value(row)"></span>
                        </template>
                        <template v-else>
                            <template v-for="(val, colIdentifier) in row"
                                :key="`template-col-${col.identifier}-value-${index}`">
                                <template v-if="String(colIdentifier) == col.identifier">
                                    <span>{{ val }}</span>
                                </template>
                            </template>
                        </template>

                    </td>
                </tr>
            </tbody>
        </table>

        <div class="mt-6 sm:flex sm:items-center sm:justify-between ">
            <div class="text-sm text-gray-500 dark:text-gray-400">
                Showing 
                <span class="font-medium text-gray-700 dark:text-gray-100">
                    {{ getPagerFromNumber() }} 
                        to 
                    {{ getPagerToNumber() }}
                </span>
            </div>

            <div class="flex items-center mt-4 gap-x-4 sm:mt-0">
                <button type="button" :disabled="isPrevPageButtonEnabled()" @click="getPrevPage"
                    class="flex items-center justify-center w-1/2 px-5 py-2 text-sm disabled:bg-gray-300 text-gray-700 capitalize transition-colors duration-200 bg-white border rounded-md sm:w-auto gap-x-2 hover:bg-gray-100 dark:bg-gray-900 dark:text-gray-200 dark:border-gray-700 dark:hover:bg-gray-800">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                        stroke="currentColor" class="w-5 h-5 rtl:-scale-x-100">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 15.75L3 12m0 0l3.75-3.75M3 12h18" />
                    </svg>

                    <span>
                        previous
                    </span>
                </button>

                <button :disabled="isNextPageButtonEnabled()" @click="getNextPage"
                    class="flex items-center justify-center w-1/2 px-5 py-2 disabled:bg-gray-300 text-sm text-gray-700 capitalize transition-colors duration-200 bg-white border rounded-md sm:w-auto gap-x-2 hover:bg-gray-100 dark:bg-gray-900 dark:text-gray-200 dark:border-gray-700 dark:hover:bg-gray-800">
                    <span>
                        Next
                    </span>

                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                        stroke="currentColor" class="w-5 h-5 rtl:-scale-x-100">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 8.25L21 12m0 0l-3.75 3.75M21 12H3" />
                    </svg>
                </button>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as _ from "lodash"
import { AxiosResponse } from 'axios'
import avoRedRustApi from '../api'


const per_page = ref(5)
const rows = ref<Array<AdminUserType>>()
const paginate = ref<Paginate>()

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

            html += `<a href="#/admin-user-edit/${row.id}">Edit</a>`
            html += ` | <a href="#/admin-user-show/${row.id}">Show</a>`

            return html;
        }
    }
]
const getPagerFromNumber = () => {
    const current_page: number = _.get(paginate.value, 'current_page', 0);
    if (current_page === 0) {
        return 1;
    }

    return (current_page * per_page.value) + 1; 
}
const isPrevPageButtonEnabled = () => {
    return (paginate.value?.current_page ?? 0) === 0
}
const isNextPageButtonEnabled = () => {
    return !(((paginate.value?.current_page ?? 0) + 1) < (paginate.value?.no_of_pages ?? 0))
}
const getPagerToNumber = () => {
    const current_page: number = _.get(paginate.value, 'current_page', 0);
    var from: number = 0;
    if (current_page === 0) {
         return per_page.value
    }
    var from = (current_page * per_page.value)
    return from + per_page.value
}
const getNextPage = async () => {
    let current_page: number = (paginate.value?.current_page ?? 0) + 1
    const response: AxiosResponse<AdminUsersListResponse> = await getAdminUserList(current_page, per_page.value)

    rows.value = response.data.results
    paginate.value = {
        current_page: response.data.current_page,
        no_of_pages: response.data.no_of_pages
    };
}

const getPrevPage = async () => {
    let current_page: number = (paginate.value?.current_page ?? 0) - 1
    const response: AxiosResponse<AdminUsersListResponse> = await getAdminUserList(current_page, per_page.value)

    rows.value = response.data.results
    paginate.value = {
        current_page: response.data.current_page,
        no_of_pages: response.data.no_of_pages
    };
}

onMounted(async () => {
    
    const response: AxiosResponse<AdminUsersListResponse> = await getAdminUserList(0, per_page.value)

    rows.value = response.data.results
    paginate.value = {
        current_page: response.data.current_page,
        no_of_pages: response.data.no_of_pages
    };
})


const getAdminUserList = async (current_page: number, per_page: number)  => {
    const token = localStorage.getItem('token')
    return await avoRedRustApi.get(
        '/api/admin-users',
        {
            params: {
                current_page: current_page,
                per_page: per_page
            },
            headers: {
                "Authorization": `Bearer ${token}`,
                'Content-Type': 'application/json'
            }
        })
}

interface AdminUsersListResponse {
    current_page: number,
    no_of_pages: number,
    results: Array<AdminUserType>
}

interface Paginate {
    current_page: number,
    no_of_pages: number,
}
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
