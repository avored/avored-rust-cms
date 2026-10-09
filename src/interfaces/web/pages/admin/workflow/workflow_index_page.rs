use leptos::prelude::*;

#[component]
pub fn WorkflowIndexPage() -> impl IntoView {
    view! {
        <div x-data="workflowIndexPage()" x-init="init()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-7xl">
                <div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <h1 class="text-3xl font-semibold tracking-tight text-slate-900">"Workflows"</h1>
                        <p class="mt-2 text-sm text-slate-500">"Manage triggered automation rules across entities."</p>
                    </div>
                    <a
                        href="/admin/workflows/create"
                        class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                    >
                        <i data-feather="plus" class="h-4 w-4"></i>
                        "Create workflow"
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
                            <h2 class="text-base font-semibold text-slate-900">"All workflows"</h2>
                            <p class="mt-1 text-sm text-slate-500"><span x-text="total"></span> " total"</p>
                        </div>
                        <div x-show="loading" class="flex items-center gap-2 text-sm text-slate-500">
                            <i data-feather="loader" class="h-4 w-4 animate-spin"></i>
                            "Loading workflows..."
                        </div>
                    </div>

                    <div class="overflow-x-auto">
                        <table class="min-w-full divide-y divide-slate-200">
                            <thead class="bg-slate-50">
                                <tr>
                                    <th scope="col" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wide text-slate-500 sm:px-6">"Name"</th>
                                    <th scope="col" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wide text-slate-500">"Trigger"</th>
                                    <th scope="col" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wide text-slate-500">"Status"</th>
                                    <th scope="col" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wide text-slate-500">"Actions"</th>
                                    <th scope="col" class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wide text-slate-500">"Updated"</th>
                                    <th scope="col" class="px-5 py-3 text-right text-xs font-semibold uppercase tracking-wide text-slate-500 sm:px-6">"Actions"</th>
                                </tr>
                            </thead>
                            <tbody x-show="!loading && workflows.length > 0" class="divide-y divide-slate-100 bg-white">
                                <template x-for="workflow in workflows" x-bind:key="workflow.id">
                                    <tr class="transition hover:bg-slate-50">
                                        <td class="whitespace-nowrap px-5 py-4 sm:px-6">
                                            <a x-bind:href="`/admin/workflows/${workflow.id}/edit`" class="font-medium text-slate-900 hover:text-primary-600" x-text="workflow.name"></a>
                                        </td>
                                        <td class="whitespace-nowrap px-5 py-4 text-sm text-slate-600">
                                            <div class="flex flex-col">
                                                <span class="font-medium text-slate-800" x-text="workflow.trigger.entity_type"></span>
                                                <span class="text-xs capitalize text-slate-500" x-text="workflow.trigger.event"></span>
                                            </div>
                                        </td>
                                        <td class="whitespace-nowrap px-5 py-4 text-sm">
                                            <button
                                                type="button"
                                                x-on:click="toggleWorkflow(workflow)"
                                                x-bind:class="workflow.is_active ? 'bg-emerald-100 text-emerald-800' : 'bg-slate-100 text-slate-700'"
                                                class="inline-flex items-center gap-2 rounded-full px-2.5 py-1 text-xs font-medium"
                                            >
                                                <span class="h-2 w-2 rounded-full" x-bind:class="workflow.is_active ? 'bg-emerald-500' : 'bg-slate-400'"></span>
                                                <span x-text="workflow.is_active ? 'Active' : 'Inactive'"></span>
                                            </button>
                                        </td>
                                        <td class="whitespace-nowrap px-5 py-4 text-sm text-slate-600" x-text="(workflow.actions || []).length"></td>
                                        <td class="whitespace-nowrap px-5 py-4 text-sm text-slate-500" x-text="formatDate(workflow.updated_at)"></td>
                                        <td class="whitespace-nowrap px-5 py-4 text-right sm:px-6">
                                            <div class="flex items-center justify-end gap-2">
                                                <a
                                                    x-bind:href="`/admin/workflows/${workflow.id}/edit`"
                                                    class="inline-flex items-center justify-center gap-2 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                                                >
                                                    <i data-feather="edit-3" class="h-4 w-4"></i>
                                                    "Edit"
                                                </a>
                                                <button
                                                    type="button"
                                                    x-on:click="confirmDelete(workflow)"
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

                    <div x-show="!loading && workflows.length === 0" class="px-5 py-12 text-center sm:px-6">
                        <i data-feather="inbox" class="mx-auto h-10 w-10 text-slate-300"></i>
                        <h3 class="mt-3 text-base font-medium text-slate-900">"No workflows yet"</h3>
                        <p class="mt-1 text-sm text-slate-500">"Create an automation to react to entity changes."</p>
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
                </div>
            </div>

            <template x-if="deleteModalOpen && workflowToDelete">
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 px-4">
                    <div class="w-full max-w-md rounded-xl border border-slate-200 bg-white p-6 shadow-xl">
                        <div class="flex items-start gap-3">
                            <div class="rounded-full bg-red-100 p-2 text-red-600">
                                <i data-feather="alert-triangle" class="h-5 w-5"></i>
                            </div>
                            <div class="flex-1">
                                <h3 class="text-lg font-semibold text-slate-900">"Delete workflow"</h3>
                                <p class="mt-2 text-sm text-slate-600">
                                    "Are you sure you want to delete "
                                    <span class="font-medium text-slate-900" x-text="workflowToDelete.name"></span>
                                    "? This action cannot be undone."
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
                                x-on:click="deleteWorkflow()"
                                x-bind:disabled="deleting"
                                class="rounded-lg bg-red-600 px-4 py-2 text-sm font-semibold text-white hover:bg-red-500 disabled:cursor-not-allowed disabled:opacity-60"
                            >
                                <span x-show="!deleting">"Delete workflow"</span>
                                <span x-show="deleting">"Deleting..."</span>
                            </button>
                        </div>
                    </div>
                </div>
            </template>
        </div>
    }
}
