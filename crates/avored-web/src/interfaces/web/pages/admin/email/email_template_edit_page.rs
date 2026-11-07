use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn EmailTemplateEditPage() -> impl IntoView {
    let params = use_params_map();
    let template_id = move || params.read().get("id").unwrap_or_default();

    let data_init = move || format!("emailTemplateEditPage('{}')", template_id());

    view! {
        <div x-data={data_init} x-init="init()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-4xl">
                <div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <a
                            href="/admin/email-templates"
                            class="mb-3 inline-flex items-center gap-2 text-sm font-medium text-slate-500 transition hover:text-primary-600"
                        >
                            <i data-feather="arrow-left" class="h-4 w-4"></i>
                            "Back to templates"
                        </a>
                        <h1 class="text-3xl font-semibold tracking-tight text-slate-900">"Edit email template"</h1>
                        <p class="mt-2 max-w-2xl text-sm leading-6 text-slate-500">"Update the template details and save your changes."</p>
                    </div>
                </div>

                <template x-if="errorMessage">
                    <div class="mb-6 flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" role="alert">
                        <i data-feather="alert-circle" class="mt-0.5 h-4 w-4 shrink-0"></i>
                        <span x-text="errorMessage"></span>
                    </div>
                </template>

                <div x-show="loading" class="rounded-xl border border-slate-200 bg-white px-6 py-16 text-center text-sm text-slate-500">
                    "Loading template..."
                </div>

                <form x-show="!loading" x-on:submit.prevent="handleSubmit" class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <div class="grid gap-6 px-5 py-6 sm:px-8">
                        <div>
                            <label for="template-edit-name" class="mb-2 block text-sm font-medium text-slate-700">"Name"</label>
                            <input
                                id="template-edit-name"
                                name="name"
                                type="text"
                                x-model="name"
                                x-bind:class="fieldError('name') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                x-bind:aria-invalid="fieldError('name') ? 'true' : 'false'"
                                autocomplete="off"
                                class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                            />
                            <template x-if="fieldError('name')">
                                <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('name')"></p>
                            </template>
                        </div>

                        <div>
                            <label for="template-edit-subject" class="mb-2 block text-sm font-medium text-slate-700">"Subject"</label>
                            <input
                                id="template-edit-subject"
                                name="subject"
                                type="text"
                                x-model="subject"
                                x-bind:class="fieldError('subject') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                x-bind:aria-invalid="fieldError('subject') ? 'true' : 'false'"
                                autocomplete="off"
                                class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                            />
                            <template x-if="fieldError('subject')">
                                <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('subject')"></p>
                            </template>
                        </div>

                        <div>
                            <label for="template-edit-body-html" class="mb-2 block text-sm font-medium text-slate-700">"HTML body"</label>
                            <textarea
                                id="template-edit-body-html"
                                name="body_html"
                                x-model="body_html"
                                rows="8"
                                class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                            ></textarea>
                        </div>

                        <div>
                            <label for="template-edit-body-plain" class="mb-2 block text-sm font-medium text-slate-700">"Plain text body"</label>
                            <textarea
                                id="template-edit-body-plain"
                                name="body_plain"
                                x-model="body_plain"
                                rows="6"
                                class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                            ></textarea>
                        </div>
                    </div>

                    <div class="flex flex-col-reverse gap-3 border-t border-slate-200 bg-slate-50 px-5 py-4 sm:flex-row sm:justify-end sm:px-8">
                        <a
                            href="/admin/email-templates"
                            class="inline-flex items-center justify-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                        >
                            "Cancel"
                        </a>
                        <button
                            type="submit"
                            x-bind:disabled="submitting || loading"
                            class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-60"
                        >
                            <i data-feather="loader" class="h-4 w-4 animate-spin" x-show="submitting"></i>
                            <span x-show="!submitting">"Save changes"</span>
                            <span x-show="submitting">"Saving..."</span>
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
