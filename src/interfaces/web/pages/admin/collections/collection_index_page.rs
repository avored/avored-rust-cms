use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use rust_i18n::t;

#[component]
pub fn CollectionIndexPage() -> impl IntoView {
    let params = use_query_map();
    let entity_id = move || params.read().get("entity_id").unwrap_or_default();

    let data_init = move || format!("collectionIndexPage('{}')", entity_id());

    view! {
        <div
            x-data={data_init}
            class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8"
        >

            <div class="mx-auto">
                <div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <h1 class="text-3xl font-semibold tracking-tight text-slate-900">
                            {t!("collections")}
                        </h1>
                    </div>
                </div>

                <div class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <div class="flex border-b border-slate-200">
                        <div class="w-64 border-r border-slate-200 bg-slate-50/50 p-4">
                            <h2 class="mb-3 px-3 text-xs font-semibold uppercase tracking-wider text-slate-500">
                                "Entities"
                            </h2>
                            <ul class="space-y-1">
                                <template x-for="entity in entitiesOptions" x-bind:key="entity.id">
                                    <li>
                                        <a
                                            x-bind:href="`/admin/collections?entity_id=${entity.id}`"
                                            x-text="entity.name"
                                            x-bind:class="selectedEntityId === entity.id
                                                ? 'bg-primary-700 text-white font-semibold shadow-xs'
                                                : 'hover:bg-slate-100 hover:text-slate-900'"
                                            class="block rounded-lg px-3 py-2 text-sm transition"
                                        ></a>
                                    </li>
                                </template>
                            </ul>
                        </div>

                        <div class="flex-1 p-6">
                            <template x-if="errorMessage">
                                <div class="mb-4 flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
                                    <i data-feather="alert-circle" class="mt-0.5 h-4 w-4 shrink-0"></i>
                                    <span x-text="errorMessage"></span>
                                </div>
                            </template>

                            <div x-show="!selectedEntityId" class="py-16 text-center">
                                <i data-feather="layers" class="mx-auto h-10 w-10 text-slate-300"></i>
                                <h3 class="mt-3 text-base font-medium text-slate-900">"Select an Entity"</h3>
                                <p class="mt-1 text-sm text-slate-500">"Choose an entity from the list to view and manage its collections."</p>
                            </div>

                            <div x-show="selectedEntityId">
                                <div class="mb-4 flex items-center justify-between">
                                    <div>
                                        <h2 
                                            class="text-lg font-semibold text-slate-900" 
                                            x-text="currentEntity ? currentEntity.name : ''"
                                        ></h2>
                                        <p class="text-xs text-slate-500" x-show="currentEntity">
                                            "Table: "
                                            <span class="font-mono" x-text="currentEntity ? currentEntity.identifier : ''"></span>
                                            " · "
                                            <span x-text="total"></span>
                                            " records"
                                        </p>
                                    </div>
                                    <div class="ml-auto">
                                        <a
                                            x-bind:href="`/admin/collections/create?entity_id=${selectedEntityId}`"
                                            class="rounded-md bg-primary-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-primary-500 focus-visible:outline focus-visible:outline-offset-2 focus-visible:outline-primary-600"
                                        >
                                            "Create"
                                        </a>
                                    </div>
                                    <div x-show="loading" class="flex items-center gap-2 text-sm text-slate-500">
                                        <i data-feather="loader" class="h-4 w-4 animate-spin"></i>
                                        "Loading records..."
                                    </div>
                                </div>

                                <div class="overflow-x-auto rounded-lg border border-slate-200">
                                    <table class="min-w-full divide-y divide-slate-200 text-left text-sm">
                                        <thead class="bg-slate-50 font-medium text-slate-600">
                                            <tr>
                                                <th scope="col" class="px-4 py-3">"ID"</th>
                                                <template x-for="attr in (currentEntity ? currentEntity.attributes : [])" x-bind:key="attr.id">
                                                    <th scope="col" class="px-4 py-3" x-text="attr.name"></th>
                                                </template>
                                                <th scope="col" class="px-4 py-3 text-right">"Actions"</th>
                                            </tr>
                                        </thead>
                                        <tbody x-show="!loading && collections.length > 0" class="divide-y divide-slate-100 bg-white">
                                            <template x-for="(item, index) in collections" x-bind:key="getAttributeValue(item, 'id') || index">
                                                <tr class="transition hover:bg-slate-50">
                                                    <td class="whitespace-nowrap px-4 py-3 font-mono text-xs text-slate-600">
                                                        <span x-text="getAttributeValue(item, 'id')"></span>
                                                    </td>
                                                    <template x-for="attr in (currentEntity ? currentEntity.attributes : [])" x-bind:key="attr.id">
                                                        <td class="whitespace-nowrap px-4 py-3 text-slate-800" x-text="getAttributeValue(item, attr.identifier)"></td>
                                                    </template>
                                                    <td class="whitespace-nowrap px-4 py-3 text-right">
                                                        <div class="flex items-center justify-end gap-2">
                                                            <a
                                                                x-bind:href="`/admin/collections/${getAttributeValue(item, 'id')}/edit?entity_id=${selectedEntityId}`"
                                                                class="inline-flex items-center rounded-md border border-slate-300 bg-white px-2.5 py-1.5 text-xs font-medium text-slate-700 shadow-sm transition hover:bg-slate-50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary-600"
                                                            >
                                                                "Edit"
                                                            </a>
                                                            <button
                                                                type="button"
                                                                x-on:click="confirmDelete(item)"
                                                                class="inline-flex items-center rounded-md border border-red-200 bg-red-50 px-2.5 py-1.5 text-xs font-medium text-red-700 shadow-sm transition hover:bg-red-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600"
                                                            >
                                                                "Delete"
                                                            </button>
                                                        </div>
                                                    </td>
                                                </tr>
                                            </template>
                                        </tbody>
                                    </table>

                                    <div x-show="!loading && collections.length === 0" class="py-12 text-center text-slate-500">
                                        <p class="text-sm">"No records found in this collection."</p>
                                    </div>
                                </div>

                                <div x-show="deleteModalOpen" x-cloak class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 px-4" x-on:keydown.escape.window="cancelDelete()">
                                    <div class="w-full max-w-md rounded-xl bg-white p-6 shadow-xl" x-on:click.outside="cancelDelete()" role="dialog" aria-modal="true" aria-labelledby="delete-collection-title">
                                        <h2 id="delete-collection-title" class="text-lg font-semibold text-slate-900">"Delete record?"</h2>
                                        <p class="mt-2 text-sm leading-6 text-slate-500">"This will remove the selected collection record from the table."</p>
                                        <p class="mt-3 rounded-lg bg-slate-50 px-3 py-2 font-mono text-xs text-slate-700" x-text="recordToDelete ? getAttributeValue(recordToDelete, 'id') : ''"></p>
                                        <div class="mt-6 flex justify-end gap-3">
                                            <button type="button" x-on:click="cancelDelete()" class="rounded-lg border border-slate-300 px-4 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-50">"Cancel"</button>
                                            <button type="button" x-on:click="deleteRecord()" x-bind:disabled="deleting" class="rounded-lg bg-red-600 px-4 py-2.5 text-sm font-semibold text-white hover:bg-red-700 disabled:cursor-not-allowed disabled:opacity-60">
                                                <span x-show="!deleting">"Delete record"</span>
                                                <span x-show="deleting">"Deleting..."</span>
                                            </button>
                                        </div>
                                    </div>
                                </div>

                                <div x-show="total > 0" class="mt-4 flex items-center justify-between border-t border-slate-200 pt-4">
                                    <div class="text-xs text-slate-500">
                                        "Showing "
                                        <span class="font-medium text-slate-700" x-text="firstVisibleItem()"></span>
                                        " to "
                                        <span class="font-medium text-slate-700" x-text="lastVisibleItem()"></span>
                                        " of "
                                        <span class="font-medium text-slate-700" x-text="total"></span>
                                        " records"
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <button
                                            type="button"
                                            x-on:click="previousPage()"
                                            x-bind:disabled="page <= 1 || loading"
                                            class="inline-flex items-center gap-1 rounded-md border border-slate-300 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-xs hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-50"
                                        >
                                            "Previous"
                                        </button>
                                        <span class="px-2 text-xs text-slate-500">
                                            "Page "
                                            <span class="font-medium text-slate-700" x-text="page"></span>
                                            " of "
                                            <span class="font-medium text-slate-700" x-text="totalPages()"></span>
                                        </span>
                                        <button
                                            type="button"
                                            x-on:click="nextPage()"
                                            x-bind:disabled="page >= totalPages() || loading"
                                            class="inline-flex items-center gap-1 rounded-md border border-slate-300 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-xs hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-50"
                                        >
                                            "Next"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
