use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use rust_i18n::t;

#[component]
pub fn EntityEditPage() -> impl IntoView {
    let params = use_params_map();
    let entity_id = move || params.read().get("id").unwrap_or_default();

    view! {
        <div x-data=format!("entityEditPage('{}')", entity_id()) class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-4xl">
                <div class="mb-8">
                    <a href="/admin/entity" class="mb-3 inline-flex items-center gap-2 text-sm font-medium text-slate-500 transition hover:text-primary-600">
                        <i data-feather="arrow-left" class="h-4 w-4"></i>
                        "Back to entities"
                    </a>
                    <h1 class="text-3xl font-semibold tracking-tight text-slate-900">{t!("edit_entity")}</h1>
                    <p class="mt-2 text-sm leading-6 text-slate-500">"Update the name or identifier for this entity."</p>
                </div>

                <template x-if="errorMessage">
                    <div class="mb-6 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" role="alert" x-text="errorMessage"></div>
                </template>

                <div x-show="loading" class="rounded-xl border border-slate-200 bg-white px-6 py-16 text-center text-sm text-slate-500">
                    "Loading entity..."
                </div>

                <form x-show="!loading" x-on:submit.prevent="handleSubmit" class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <div class="border-b border-slate-200 px-5 py-5 sm:px-8">
                        <h2 class="text-base font-semibold text-slate-900">"Entity details"</h2>
                        <p class="mt-1 text-sm text-slate-500">"Keep the identifier stable if other systems already use it."</p>
                    </div>

                    <div class="grid gap-6 px-5 py-6 sm:px-8">
                        <div>
                            <label for="edit-entity-name" class="mb-2 block text-sm font-medium text-slate-700">{t!("name")}</label>
                            <input
                                id="edit-entity-name"
                                name="name"
                                type="text"
                                x-model="name"
                                x-bind:class="fieldError('name') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                            />
                            <template x-if="fieldError('name')">
                                <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('name')"></p>
                            </template>
                        </div>

                        <div>
                            <label for="edit-entity-identifier" class="mb-2 block text-sm font-medium text-slate-700">{t!("identifier")}</label>
                            <input
                                id="edit-entity-identifier"
                                name="identifier"
                                type="text"
                                x-model="identifier"
                                x-bind:class="fieldError('identifier') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 font-mono text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                            />
                            <template x-if="fieldError('identifier')">
                                <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('identifier')"></p>
                            </template>
                        </div>
                    </div>

                    <div class="flex flex-col-reverse gap-3 border-t border-slate-200 bg-slate-50 px-5 py-4 sm:flex-row sm:justify-end sm:px-8">
                        <a href="/admin/entity" class="inline-flex items-center justify-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-100">"Cancel"</a>
                        <button type="submit" x-bind:disabled="submitting" class="inline-flex items-center justify-center rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white hover:bg-primary-700 disabled:cursor-not-allowed disabled:opacity-60">
                            <span x-show="!submitting">"Save changes"</span>
                            <span x-show="submitting">"Saving..."</span>
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
