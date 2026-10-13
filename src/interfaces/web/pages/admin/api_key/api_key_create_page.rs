use leptos::prelude::*;

#[component]
pub fn ApiKeyCreatePage() -> impl IntoView {
    view! {
        <div x-data="apiKeyCreatePage()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-4xl">
                <div class="mb-8">
                    <a
                        href="/admin/api-keys"
                        class="mb-3 inline-flex items-center gap-1.5 text-sm font-medium text-slate-500 hover:text-slate-700"
                    >
                        <i data-feather="arrow-left" class="h-4 w-4"></i>
                        "Back to API Keys"
                    </a>
                    <h1 class="text-3xl font-semibold tracking-tight text-slate-900">"Create API Key"</h1>
                    <p class="mt-2 text-sm text-slate-500">"Generate a new API key with tailored permissions for external integrations."</p>
                </div>

                <template x-if="errorMessage">
                    <div class="mb-6 flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" role="alert">
                        <i data-feather="alert-circle" class="mt-0.5 h-4 w-4 shrink-0"></i>
                        <span x-text="errorMessage"></span>
                    </div>
                </template>

                // One-time secret reveal panel (shown once after successful creation)
                <div x-show="plainSecret" class="mb-8 overflow-hidden rounded-xl border border-amber-300 bg-amber-50 p-6 shadow-sm">
                    <div class="flex items-start gap-3">
                        <div class="rounded-full bg-amber-200 p-2 text-amber-800">
                            <i data-feather="alert-triangle" class="h-5 w-5"></i>
                        </div>
                        <div class="flex-1">
                            <h2 class="text-base font-semibold text-amber-900">"Save your API key secret"</h2>
                            <p class="mt-1 text-sm text-amber-800">
                                "Please copy and securely store this secret now. For security purposes, it will "
                                <strong class="font-bold">"never be shown again."</strong>
                            </p>

                            <div class="mt-4 flex items-center gap-2">
                                <input
                                    type="text"
                                    readonly=true
                                    x-bind:value="plainSecret"
                                    class="w-full rounded-lg border border-amber-300 bg-white px-3.5 py-2.5 font-mono text-sm text-slate-900 shadow-sm focus:outline-none focus:ring-2 focus:ring-amber-500"
                                />
                                <button
                                    type="button"
                                    x-on:click="copySecret()"
                                    class="inline-flex shrink-0 items-center gap-2 rounded-lg bg-amber-800 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-amber-900"
                                >
                                    <i data-feather="copy" class="h-4 w-4"></i>
                                    <span x-text="copied ? 'Copied!' : 'Copy'"></span>
                                </button>
                            </div>

                            <div class="mt-4">
                                <a
                                    href="/admin/api-keys"
                                    class="inline-flex items-center gap-1.5 text-sm font-semibold text-amber-900 hover:text-amber-950"
                                >
                                    "Done, go back to API keys"
                                    <i data-feather="arrow-right" class="h-4 w-4"></i>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>

                // Creation Form (hidden once secret is revealed)
                <div x-show="!plainSecret" class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <form x-on:submit.prevent="submit()" class="p-6 sm:p-8">
                        <div class="space-y-6">
                            <div>
                                <label for="api-key-name" class="block text-sm font-medium text-slate-900">
                                    "Key Name"
                                    <span class="text-red-500">"*"</span>
                                </label>
                                <p class="mt-1 text-xs text-slate-500">"A friendly descriptive name for identifying this key (e.g. 'Stripe Webhook Worker', 'Mobile App Client')."</p>
                                <input
                                    id="api-key-name"
                                    type="text"
                                    x-model="name"
                                    required=true
                                    placeholder="e.g. Production Mobile App"
                                    class="mt-2 block w-full rounded-lg border border-slate-300 px-3.5 py-2 text-sm text-slate-900 shadow-sm transition placeholder:text-slate-400 focus:border-primary-500 focus:outline-none focus:ring-2 focus:ring-primary-500/20"
                                />
                            </div>

                            <div class="border-t border-slate-200 pt-6">
                                <h3 class="text-sm font-semibold text-slate-900">"Permissions"</h3>
                                <p class="mt-1 text-xs text-slate-500">"Select which v1 API endpoints and HTTP methods this key can access."</p>

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
                                <span x-show="!submitting">"Generate API key"</span>
                                <span x-show="submitting">"Generating..."</span>
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
