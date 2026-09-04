use leptos::prelude::*;
use rust_i18n::t;

#[component]
pub fn EntityCreatePage() -> impl IntoView {
    view! {
        <div x-data="entityCreatePage()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-4xl">
                <div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <a
                            href="/admin/entity"
                            class="mb-3 inline-flex items-center gap-2 text-sm font-medium text-slate-500 transition hover:text-primary-600"
                        >
                            <i data-feather="arrow-left" class="h-4 w-4"></i>
                            "Back to entities"
                        </a>
                        <h1 class="text-3xl font-semibold tracking-tight text-slate-900">
                            {t!("create_entity")}
                        </h1>
                        <p class="mt-2 max-w-2xl text-sm leading-6 text-slate-500">
                            "Define a reusable content type and choose how its values should be stored."
                        </p>
                    </div>
                    <div class="hidden rounded-full border border-primary-100 bg-primary-50 px-3 py-1 text-xs font-semibold uppercase tracking-wide text-primary-700 sm:block">
                        "New entity"
                    </div>
                </div>

                <template x-if="errorMessage">
                    <div class="mb-6 flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" role="alert">
                        <i data-feather="alert-circle" class="mt-0.5 h-4 w-4 shrink-0"></i>
                        <span x-text="errorMessage"></span>
                    </div>
                </template>

                <form x-on:submit.prevent="handleSubmit" class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <div class="border-b border-slate-200 px-5 py-5 sm:px-8">
                        <h2 class="text-base font-semibold text-slate-900">"Entity details"</h2>
                        <p class="mt-1 text-sm text-slate-500">"Give this entity a clear name and stable identifier."</p>
                    </div>

                    <div class="grid gap-6 px-5 py-6 sm:px-8 md:grid-cols-2">
                        <div class="md:col-span-2">
                            <label for="entity-name" class="mb-2 block text-sm font-medium text-slate-700">
                                {t!("name")}
                            </label>
                            <input
                                id="entity-name"
                                name="name"
                                type="text"
                                x-model="name"
                                x-on:input="handleNameChange"
                                x-bind:class="fieldError('name') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                x-bind:aria-invalid="fieldError('name') ? 'true' : 'false'"
                                aria-describedby="entity-name-help entity-name-error"
                                autocomplete="off"
                                class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                placeholder="e.g. Blog post"
                            />
                            <p id="entity-name-help" class="mt-2 text-xs text-slate-500">"The display name shown throughout the admin."</p>
                            <template x-if="fieldError('name')">
                                <p id="entity-name-error" class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('name')"></p>
                            </template>
                        </div>

                        <div>
                            <label for="entity-identifier" class="mb-2 block text-sm font-medium text-slate-700">
                                {t!("identifier")}
                            </label>
                            <input
                                id="entity-identifier"
                                name="identifier"
                                type="text"
                                x-model="identifier"
                                x-on:input="handleIdentifierInput"
                                x-bind:class="fieldError('identifier') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                x-bind:aria-invalid="fieldError('identifier') ? 'true' : 'false'"
                                aria-describedby="entity-identifier-help entity-identifier-error"
                                autocomplete="off"
                                class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 font-mono text-sm text-slate-900 outline-none transition placeholder:font-sans placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                placeholder="blog_post"
                            />
                            <p id="entity-identifier-help" class="mt-2 text-xs text-slate-500">"Used in API requests and database queries."</p>
                            <template x-if="fieldError('identifier')">
                                <p id="entity-identifier-error" class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('identifier')"></p>
                            </template>
                        </div>

                    </div>

                    <div class="flex flex-col-reverse gap-3 border-t border-slate-200 bg-slate-50 px-5 py-4 sm:flex-row sm:justify-end sm:px-8">
                        <a
                            href="/admin/entity"
                            class="inline-flex items-center justify-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                        >
                            "Cancel"
                        </a>
                        <button
                            type="submit"
                            class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-60"
                            x-bind:disabled="submitting"
                        >
                            <i data-feather="save" class="h-4 w-4" x-show="!submitting"></i>
                            <i data-feather="loader" class="h-4 w-4 animate-spin" x-show="submitting"></i>
                            <span x-show="!submitting">"Create entity"</span>
                            <span x-show="submitting">"Creating..."</span>
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
