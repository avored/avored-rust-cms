use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ApiKeyEditPage() -> impl IntoView {
    let params = use_params_map();
    let key_id = move || params.read().get("id").unwrap_or_default();
    let data_init = move || format!("apiKeyEditPage('{}')", key_id());

    view! {
        <div x-data={data_init} x-init="init()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-4xl">
                <div class="mb-8">
                    <a
                        href="/admin/api-keys"
                        class="mb-3 inline-flex items-center gap-1.5 text-sm font-medium text-slate-500 hover:text-slate-700"
                    >
                        <i data-feather="arrow-left" class="h-4 w-4"></i>
                        "Back to API Keys"
                    </a>
                    <div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                        <div>
                            <h1 class="text-3xl font-semibold tracking-tight text-slate-900">"Edit API Key"</h1>
                            <p class="mt-2 text-sm text-slate-500">
                                "Prefix: "
                                <span class="font-mono font-medium text-slate-700" x-text="keyPrefix + '...'"></span>
                            </p>
                        </div>
                        <button
                            type="button"
                            x-on:click="confirmDelete()"
                            class="inline-flex items-center gap-2 rounded-lg border border-red-200 bg-red-50 px-4 py-2 text-sm font-semibold text-red-700 shadow-sm transition hover:bg-red-100"
                        >
                            <i data-feather="trash-2" class="h-4 w-4"></i>
                            "Delete key"
                        </button>
                    </div>
                </div>

                <template x-if="errorMessage">
                    <div class="mb-6 flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" role="alert">
                        <i data-feather="alert-circle" class="mt-0.5 h-4 w-4 shrink-0"></i>
                        <span x-text="errorMessage"></span>
                    </div>
                </template>

                <template x-if="successMessage">
                    <div class="mb-6 flex items-start gap-3 rounded-lg border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-800" role="alert">
                        <i data-feather="check-circle" class="mt-0.5 h-4 w-4 shrink-0 text-emerald-600"></i>
                        <span x-text="successMessage"></span>
                    </div>
                </template>

                <div class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <div x-show="loading" class="flex items-center justify-center p-12 text-sm text-slate-500">
                        <i data-feather="loader" class="mr-2 h-5 w-5 animate-spin"></i>
                        "Loading API key details..."
                    </div>

                    <form x-show="!loading" x-on:submit.prevent="submit()" class="p-6 sm:p-8">
                        <div class="space-y-6">
                            <div>
                                <label for="api-key-name" class="block text-sm font-medium text-slate-900">
                                    "Key Name"
                                    <span class="text-red-500">"*"</span>
                                </label>
                                <input
                                    id="api-key-name"
                                    type="text"
                                    x-model="name"
                                    required=true
                                    class="mt-2 block w-full rounded-lg border border-slate-300 px-3.5 py-2 text-sm text-slate-900 shadow-sm transition placeholder:text-slate-400 focus:border-primary-500 focus:outline-none focus:ring-2 focus:ring-primary-500/20"
                                />
                            </div>

                            <div class="border-t border-slate-200 pt-6">
                                <h3 class="text-sm font-semibold text-slate-900">"Permissions"</h3>
                                <p class="mt-1 text-xs text-slate-500">"Modify which endpoints and HTTP verbs this API key is permitted to invoke."</p>

                                <div class="mt-4 divide-y divide-slate-100 rounded-lg border border-slate-200 bg-slate-50/50">
                                    <label class="flex cursor-pointer items-center justify-between p-4 hover:bg-slate-100/50">
                                        <div class="flex items-center gap-3">
                                            <input
                                                type="checkbox"
                                                x-model="permissions['GET:/api/v1/entities']"
                                                class="h-4 w-4 rounded border-slate-300 text-primary-600 focus:ring-primary-500"
                                            />
                                            <div>
                                                <span class="inline-flex items-center rounded-md bg-blue-50 px-2 py-0.5 text-xs font-semibold text-blue-700">"GET"</span>
                                                <span class="ml-2 font-mono text-xs text-slate-800">"/api/v1/entities"</span>
                                            </div>
                                        </div>
                                        <span class="text-xs text-slate-500">"Paginate / list entity records"</span>
                                    </label>

                                    <label class="flex cursor-pointer items-center justify-between p-4 hover:bg-slate-100/50">
                                        <div class="flex items-center gap-3">
                                            <input
                                                type="checkbox"
                                                x-model="permissions['POST:/api/v1/entities']"
                                                class="h-4 w-4 rounded border-slate-300 text-primary-600 focus:ring-primary-500"
                                            />
                                            <div>
                                                <span class="inline-flex items-center rounded-md bg-emerald-50 px-2 py-0.5 text-xs font-semibold text-emerald-700">"POST"</span>
                                                <span class="ml-2 font-mono text-xs text-slate-800">"/api/v1/entities"</span>
                                            </div>
                                        </div>
                                        <span class="text-xs text-slate-500">"Create a new entity record"</span>
                                    </label>

                                    <label class="flex cursor-pointer items-center justify-between p-4 hover:bg-slate-100/50">
                                        <div class="flex items-center gap-3">
                                            <input
                                                type="checkbox"
                                                x-model="permissions['GET:/api/v1/entities/{id}']"
                                                class="h-4 w-4 rounded border-slate-300 text-primary-600 focus:ring-primary-500"
                                            />
                                            <div>
                                                <span class="inline-flex items-center rounded-md bg-blue-50 px-2 py-0.5 text-xs font-semibold text-blue-700">"GET"</span>
                                                <span class="ml-2 font-mono text-xs text-slate-800">"/api/v1/entities/{id}"</span>
                                            </div>
                                        </div>
                                        <span class="text-xs text-slate-500">"Fetch a record by ID"</span>
                                    </label>

                                    <label class="flex cursor-pointer items-center justify-between p-4 hover:bg-slate-100/50">
                                        <div class="flex items-center gap-3">
                                            <input
                                                type="checkbox"
                                                x-model="permissions['PATCH:/api/v1/entities/{id}']"
                                                class="h-4 w-4 rounded border-slate-300 text-primary-600 focus:ring-primary-500"
                                            />
                                            <div>
                                                <span class="inline-flex items-center rounded-md bg-amber-50 px-2 py-0.5 text-xs font-semibold text-amber-700">"PATCH"</span>
                                                <span class="ml-2 font-mono text-xs text-slate-800">"/api/v1/entities/{id}"</span>
                                            </div>
                                        </div>
                                        <span class="text-xs text-slate-500">"Update a record by ID"</span>
                                    </label>

                                    <label class="flex cursor-pointer items-center justify-between p-4 hover:bg-slate-100/50">
                                        <div class="flex items-center gap-3">
                                            <input
                                                type="checkbox"
                                                x-model="permissions['DELETE:/api/v1/entities/{id}']"
                                                class="h-4 w-4 rounded border-slate-300 text-primary-600 focus:ring-primary-500"
                                            />
                                            <div>
                                                <span class="inline-flex items-center rounded-md bg-rose-50 px-2 py-0.5 text-xs font-semibold text-rose-700">"DELETE"</span>
                                                <span class="ml-2 font-mono text-xs text-slate-800">"/api/v1/entities/{id}"</span>
                                            </div>
                                        </div>
                                        <span class="text-xs text-slate-500">"Delete a record by ID"</span>
                                    </label>
                                </div>
                            </div>
                        </div>

                        <div class="mt-8 flex items-center justify-end gap-3 border-t border-slate-200 pt-6">
                            <a
                                href="/admin/api-keys"
                                class="rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-50"
                            >
                                "Cancel"
                            </a>
                            <button
                                type="submit"
                                x-bind:disabled="submitting"
                                class="inline-flex items-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-500 disabled:cursor-not-allowed disabled:opacity-60"
                            >
                                <span x-show="!submitting">"Save changes"</span>
                                <span x-show="submitting">"Saving..."</span>
                            </button>
                        </div>
                    </form>
                </div>
            </div>

            // Delete confirmation modal
            <template x-if="deleteModalOpen">
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 px-4">
                    <div class="w-full max-w-md rounded-xl border border-slate-200 bg-white p-6 shadow-xl">
                        <div class="flex items-start gap-3">
                            <div class="rounded-full bg-red-100 p-2 text-red-600">
                                <i data-feather="alert-triangle" class="h-5 w-5"></i>
                            </div>
                            <div class="flex-1">
                                <h3 class="text-lg font-semibold text-slate-900">"Delete API key"</h3>
                                <p class="mt-2 text-sm text-slate-600">
                                    "Are you sure you want to delete this API key? This action is immediate and cannot be undone."
                                </p>
                            </div>
                        </div>
                        <div class="mt-6 flex justify-end gap-3">
                            <button
                                type="button"
                                x-on:click="cancelDelete()"
                                class="rounded-lg border border-slate-300 bg-white px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50"
                            >
                                "Cancel"
                            </button>
                            <button
                                type="button"
                                x-on:click="deleteKey()"
                                x-bind:disabled="deleting"
                                class="rounded-lg bg-red-600 px-4 py-2 text-sm font-semibold text-white hover:bg-red-500 disabled:cursor-not-allowed disabled:opacity-60"
                            >
                                <span x-show="!deleting">"Delete API key"</span>
                                <span x-show="deleting">"Deleting..."</span>
                            </button>
                        </div>
                    </div>
                </div>
            </template>
        </div>
    }
}
