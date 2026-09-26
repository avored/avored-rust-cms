use leptos::prelude::*;

#[component]
pub fn EmailTemplateIndexPage() -> impl IntoView {
    view! {
        <div x-data="emailTemplateIndexPage()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-7xl">
                <div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <h1 class="text-3xl font-semibold tracking-tight text-slate-900">"Email Templates"</h1>
                    </div>
                    <a
                        href="/admin/email-templates/create"
                        class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                    >
                        <i data-feather="plus" class="h-4 w-4"></i>
                        "Create template"
                    </a>
                </div>

                <template x-if="errorMessage">
                    <div class="mb-6 flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" role="alert">
                        <i data-feather="alert-circle" class="mt-0.5 h-4 w-4 shrink-0"></i>
                        <span x-text="errorMessage"></span>
                    </div>
                </template>

                <div class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <div class="flex items-center justify-between border-b border-slate-200 px-5 py-4 sm:px-6">
                        <div>
                            <h2 class="text-base font-semibold text-slate-900">"All templates"</h2>
                            <p class="mt-1 text-sm text-slate-500"><span x-text="total"></span> " total"</p>
                        </div>
                        <div x-show="loading" class="flex items-center gap-2 text-sm text-slate-500">
                            <i data-feather="loader" class="h-4 w-4 animate-spin"></i>
                            "Loading templates..."
                        </div>
                    </div>

                    <div class="overflow-x-auto">
                        <table class="min-w-full divide-y divide-slate-200">
                            <thead class="bg-slate-50">
                                <tr>
                                    <th scope="col" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wide text-slate-500 sm:px-6">"Name"</th>
                                    <th scope="col" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wide text-slate-500">"Subject"</th>
                                    <th scope="col" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wide text-slate-500">"Created"</th>
                                    <th scope="col" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wide text-slate-500">"Updated"</th>
                                    <th scope="col" class="px-5 py-3 text-right text-xs font-semibold uppercase tracking-wide text-slate-500 sm:px-6">"Actions"</th>
                                </tr>
                            </thead>
                            <tbody x-show="!loading && templates.length > 0" class="divide-y divide-slate-100 bg-white">
                                <template x-for="template in templates" x-bind:key="template.id">
                                    <tr class="transition hover:bg-slate-50">
                                        <td class="whitespace-nowrap px-5 py-4 sm:px-6">
                                            <a x-bind:href="`/admin/email-templates/${template.id}/edit`" class="font-medium text-slate-900 hover:text-primary-600" x-text="template.name"></a>
                                        </td>
                                        <td class="whitespace-nowrap px-5 py-4 text-sm text-slate-600" x-text="template.subject"></td>
                                        <td class="whitespace-nowrap px-5 py-4 text-sm text-slate-500" x-text="formatDate(template.created_at)"></td>
                                        <td class="whitespace-nowrap px-5 py-4 text-sm text-slate-500" x-text="formatDate(template.updated_at)"></td>
                                        <td class="whitespace-nowrap px-5 py-4 text-right sm:px-6">
                                            <div class="flex items-center justify-end gap-2">
                                                <a
                                                    x-bind:href="`/admin/email-templates/${template.id}/edit`"
                                                    class="inline-flex items-center justify-center gap-2 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                                                >
                                                    <i data-feather="edit-3" class="h-4 w-4"></i>
                                                    "Edit"
                                                </a>
                                                <button
                                                    type="button"
                                                    x-on:click="confirmDelete(template)"
                                                    class="inline-flex items-center justify-center gap-2 rounded-lg border border-red-200 bg-red-50 px-3 py-2 text-sm font-medium text-red-700 transition hover:bg-red-100 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
                                                >
                                                    <i data-feather="trash-2" class="h-4 w-4"></i>
                                                    "Delete"
                                                </button>
                                            </div>
                                        </td>
                                    </tr>
                                </template>
                            </tbody>
                        </table>
                    </div>

                    <div x-show="!loading && total > 0" class="flex flex-col gap-3 border-t border-slate-200 px-5 py-4 sm:flex-row sm:items-center sm:justify-between sm:px-6">
                        <p class="text-sm text-slate-500">
                            "Showing "
                            <span class="font-medium text-slate-700" x-text="firstVisibleItem()"></span>
                            " to "
                            <span class="font-medium text-slate-700" x-text="lastVisibleItem()"></span>
                            " of "
                            <span class="font-medium text-slate-700" x-text="total"></span>
                        </p>
                        <div class="flex items-center gap-2">
                            <button
                                type="button"
                                x-on:click="previousPage()"
                                x-bind:disabled="page <= 1 || loading"
                                class="inline-flex items-center gap-2 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-50"
                            >
                                <i data-feather="chevron-left" class="h-4 w-4"></i>
                                "Previous"
                            </button>
                            <span class="px-2 text-sm text-slate-500">
                                "Page "
                                <span class="font-medium text-slate-700" x-text="page"></span>
                                " of "
                                <span class="font-medium text-slate-700" x-text="totalPages()"></span>
                            </span>
                            <button
                                type="button"
                                x-on:click="nextPage()"
                                x-bind:disabled="page >= totalPages() || loading"
                                class="inline-flex items-center gap-2 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-50"
                            >
                                "Next"
                                <i data-feather="chevron-right" class="h-4 w-4"></i>
                            </button>
                        </div>
                    </div>

                    <div x-show="!loading && templates.length === 0" class="px-6 py-16 text-center">
                        <i data-feather="layers" class="mx-auto h-8 w-8 text-slate-300"></i>
                        <h3 class="mt-3 text-sm font-semibold text-slate-900">"No email templates yet"</h3>
                        <p class="mt-1 text-sm text-slate-500">"Create your first template to start sending branded emails."</p>
                    </div>
                </div>

                <div x-show="deleteModalOpen" x-cloak class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 px-4" x-on:keydown.escape.window="cancelDelete()">
                    <div class="w-full max-w-md rounded-xl bg-white p-6 shadow-xl" x-on:click.outside="cancelDelete()" role="dialog" aria-modal="true" aria-labelledby="delete-email-template-title">
                        <h2 id="delete-email-template-title" class="text-lg font-semibold text-slate-900">"Delete email template?"</h2>
                        <p class="mt-2 text-sm leading-6 text-slate-500">"This will remove the template from the active list."</p>
                        <p class="mt-3 rounded-lg bg-slate-50 px-3 py-2 font-medium text-slate-700" x-text="templateToDelete?.name"></p>
                        <div class="mt-6 flex justify-end gap-3">
                            <button type="button" x-on:click="cancelDelete()" class="rounded-lg border border-slate-300 px-4 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-50">"Cancel"</button>
                            <button type="button" x-on:click="deleteTemplate()" x-bind:disabled="deleting" class="rounded-lg bg-red-600 px-4 py-2.5 text-sm font-semibold text-white hover:bg-red-700 disabled:cursor-not-allowed disabled:opacity-60">
                                <span x-show="!deleting">"Delete template"</span>
                                <span x-show="deleting">"Deleting..."</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
