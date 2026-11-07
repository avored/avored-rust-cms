use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn WorkflowEditPage() -> impl IntoView {
    let workflow_id = move || {
        let params = use_params_map();
        params.read().get("id").unwrap_or_default()
    };
    let data_init = move || format!("workflowFormPage('{}')", workflow_id());

    view! {
        <div x-data={data_init} x-init="init()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-5xl">
                <div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <a href="/admin/workflows" class="mb-3 inline-flex items-center gap-2 text-sm font-medium text-slate-500 transition hover:text-primary-600">
                            <i data-feather="arrow-left" class="h-4 w-4"></i>
                            "Back to workflows"
                        </a>
                        <h1 class="text-3xl font-semibold tracking-tight text-slate-900">"Edit workflow"</h1>
                        <p class="mt-2 max-w-2xl text-sm leading-6 text-slate-500">"Update the trigger and actions for this workflow definition."</p>
                    </div>
                </div>

                <template x-if="errorMessage">
                    <div class="mb-6 flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" role="alert">
                        <i data-feather="alert-circle" class="mt-0.5 h-4 w-4 shrink-0"></i>
                        <span x-text="errorMessage"></span>
                    </div>
                </template>

                <form x-on:submit.prevent="submit()" class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <div class="grid gap-6 px-5 py-6 sm:px-8">
                        <template x-if="loading">
                            <div class="flex items-center gap-2 text-sm text-slate-500">
                                <i data-feather="loader" class="h-4 w-4 animate-spin"></i>
                                "Loading workflow..."
                            </div>
                        </template>

                        <div x-show="!loading">
                            <div>
                                <label for="workflow-name" class="mb-2 block text-sm font-medium text-slate-700">"Name"</label>
                                <input
                                    id="workflow-name"
                                    name="name"
                                    type="text"
                                    x-model="name"
                                    x-bind:class="fieldError('name') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                    x-bind:aria-invalid="fieldError('name') ? 'true' : 'false'"
                                    autocomplete="off"
                                    class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                    placeholder="New lead notification"
                                />
                                <template x-if="fieldError('name')">
                                    <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('name')"></p>
                                </template>
                            </div>

                            <div class="mt-6 grid gap-6 md:grid-cols-2">
                                <div>
                                    <label for="workflow-entity-type" class="mb-2 block text-sm font-medium text-slate-700">"Entity type"</label>
                                    <input
                                        id="workflow-entity-type"
                                        name="trigger_entity_type"
                                        type="text"
                                        x-model="triggerEntityType"
                                        x-bind:class="fieldError('trigger.entity_type') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                        x-bind:aria-invalid="fieldError('trigger.entity_type') ? 'true' : 'false'"
                                        autocomplete="off"
                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                        placeholder="contact"
                                    />
                                    <template x-if="fieldError('trigger.entity_type')">
                                        <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('trigger.entity_type')"></p>
                                    </template>
                                </div>

                                <div>
                                    <label for="workflow-trigger-event" class="mb-2 block text-sm font-medium text-slate-700">"Trigger event"</label>
                                    <select
                                        id="workflow-trigger-event"
                                        name="trigger_event"
                                        x-model="triggerEvent"
                                        x-bind:class="fieldError('trigger.event') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                        class="w-full rounded-lg border border-slate-300 bg-white px-3.5 py-2.5 text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                    >
                                        <option value="created">"Created"</option>
                                        <option value="updated">"Updated"</option>
                                    </select>
                                    <template x-if="fieldError('trigger.event')">
                                        <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('trigger.event')"></p>
                                    </template>
                                </div>
                            </div>

                            <div class="mt-6 flex items-center justify-between rounded-lg border border-slate-200 bg-slate-50 px-3 py-3">
                                <div>
                                    <p class="text-sm font-medium text-slate-800">"Workflow enabled"</p>
                                    <p class="text-xs text-slate-500">"Turn this rule on or off without deleting it."</p>
                                </div>
                                <label class="relative inline-flex cursor-pointer items-center">
                                    <input type="checkbox" x-model="isActive" class="peer sr-only" />
                                    <span class="h-6 w-11 rounded-full bg-slate-200 transition peer-checked:bg-primary-600"></span>
                                    <span class="absolute left-1 top-1 h-4 w-4 rounded-full bg-white shadow transition peer-checked:translate-x-5"></span>
                                </label>
                            </div>

                            <div class="mt-6 space-y-5">
                                <div class="flex items-center justify-between">
                                    <h2 class="text-lg font-semibold text-slate-900">"Actions"</h2>
                                    <button
                                        type="button"
                                        x-on:click="addAction()"
                                        class="inline-flex items-center gap-2 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50"
                                    >
                                        <i data-feather="plus" class="h-4 w-4"></i>
                                        "Add action"
                                    </button>
                                </div>

                                <template x-for="(action, index) in actions" x-bind:key="index">
                                    <div class="rounded-xl border border-slate-200 bg-slate-50 p-4">
                                        <div class="mb-4 flex items-center justify-between gap-3">
                                            <p class="text-sm font-medium text-slate-700">"Action " <span x-text="index + 1"></span></p>
                                            <button
                                                type="button"
                                                x-show="actions.length > 1"
                                                x-on:click="removeAction(index)"
                                                class="inline-flex items-center gap-2 text-sm font-medium text-red-600 hover:text-red-700"
                                            >
                                                <i data-feather="trash-2" class="h-4 w-4"></i>
                                                "Remove"
                                            </button>
                                        </div>

                                        <div class="grid gap-4 md:grid-cols-2">
                                            <div class="md:col-span-2">
                                                <label class="mb-2 block text-sm font-medium text-slate-700">"Action type"</label>
                                                <select x-model="action.type" class="w-full rounded-lg border border-slate-300 bg-white px-3.5 py-2.5 text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100">
                                                    <option value="webhook">"Webhook"</option>
                                                    <option value="email">"Email"</option>
                                                </select>
                                            </div>

                                            <template x-if="action.type === 'webhook'">
                                                <div class="md:col-span-2">
                                                    <label x-bind:for="`workflow-action-url-${index}`" class="mb-2 block text-sm font-medium text-slate-700">"Webhook URL"</label>
                                                    <input
                                                        x-bind:id="`workflow-action-url-${index}`"
                                                        type="url"
                                                        x-model="action.url"
                                                        x-bind:class="fieldError('actions[' + index + '].url') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                                        placeholder="https://example.com/webhook"
                                                    />
                                                    <template x-if="fieldError('actions[' + index + '].url')">
                                                        <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('actions[' + index + '].url')"></p>
                                                    </template>
                                                </div>
                                            </template>

                                            <template x-if="action.type === 'email'">
                                                <div class="md:col-span-2">
                                                    <label x-bind:for="`workflow-action-template-${index}`" class="mb-2 block text-sm font-medium text-slate-700">"Template ID"</label>
                                                    <input
                                                        x-bind:id="`workflow-action-template-${index}`"
                                                        type="text"
                                                        x-model="action.template_id"
                                                        x-bind:class="fieldError('actions[' + index + '].template_id') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                                        placeholder="welcome-email"
                                                    />
                                                    <template x-if="fieldError('actions[' + index + '].template_id')">
                                                        <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('actions[' + index + '].template_id')"></p>
                                                    </template>
                                                </div>
                                            </template>

                                            <template x-if="action.type === 'email'">
                                                <div class="md:col-span-2">
                                                    <label x-bind:for="`workflow-action-recipient-${index}`" class="mb-2 block text-sm font-medium text-slate-700">"Recipient"</label>
                                                    <input
                                                        x-bind:id="`workflow-action-recipient-${index}`"
                                                        type="email"
                                                        x-model="action.recipient"
                                                        x-bind:class="fieldError('actions[' + index + '].recipient') ? 'border-red-400 ring-2 ring-red-100 focus:border-red-500' : ''"
                                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                                        placeholder="alerts@example.com"
                                                    />
                                                    <template x-if="fieldError('actions[' + index + '].recipient')">
                                                        <p class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('actions[' + index + '].recipient')"></p>
                                                    </template>
                                                </div>
                                            </template>
                                        </div>
                                    </div>
                                </template>
                            </div>
                        </div>
                    </div>

                    <div class="flex flex-col-reverse gap-3 border-t border-slate-200 bg-slate-50 px-5 py-4 sm:flex-row sm:justify-end sm:px-8">
                        <a href="/admin/workflows" class="inline-flex items-center justify-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2">
                            "Cancel"
                        </a>
                        <button
                            type="submit"
                            x-bind:disabled="submitting"
                            class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-60"
                        >
                            <i data-feather="loader" class="h-4 w-4 animate-spin" x-show="submitting"></i>
                            <span x-show="!submitting">"Save workflow"</span>
                            <span x-show="submitting">"Saving..."</span>
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
