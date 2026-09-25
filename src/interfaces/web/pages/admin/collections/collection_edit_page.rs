use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_query_map};

#[component]
pub fn CollectionEditPage() -> impl IntoView {
    let params = use_query_map();
    let entity_id = move || params.read().get("entity_id").unwrap_or_default();
    let record_id = move || {
        let params = use_params_map();
        params.read().get("id").unwrap_or_default()
    };

    let data_init = move || format!("collectionEditPage('{}','{}')", entity_id(), record_id());

    view! {
        <div x-data={data_init} x-init="init()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-5xl">
                <div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <a href="/admin/collections" class="mb-3 inline-flex items-center gap-2 text-sm font-medium text-slate-500 transition hover:text-primary-600">
                            <i data-feather="arrow-left" class="h-4 w-4"></i>
                            "Back to collections"
                        </a>
                        <h1 class="text-3xl font-semibold tracking-tight text-slate-900">
                            "Edit record"
                        </h1>
                        <p class="mt-2 max-w-2xl text-sm leading-6 text-slate-500">
                            "Update the selected record for this entity."
                        </p>
                    </div>
                </div>

                <template x-if="errorMessage">
                    <div class="mb-6 flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" role="alert">
                        <i data-feather="alert-circle" class="mt-0.5 h-4 w-4 shrink-0"></i>
                        <span x-text="errorMessage"></span>
                    </div>
                </template>

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
                                            x-bind:href="`/admin/collections/${recordId}?entity_id=${entity.id}`"
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
                            <div x-show="!selectedEntityId" class="py-16 text-center">
                                <i data-feather="layers" class="mx-auto h-10 w-10 text-slate-300"></i>
                                <h3 class="mt-3 text-base font-medium text-slate-900">"Select an Entity"</h3>
                                <p class="mt-1 text-sm text-slate-500">"Choose an entity to edit a collection record."</p>
                            </div>

                            <div x-show="selectedEntityId" class="space-y-6">
                                <div class="flex items-center justify-between gap-4">
                                    <div>
                                        <p class="text-xs font-medium uppercase tracking-wide text-slate-500">"Entity"</p>
                                        <h2 class="mt-1 text-xl font-semibold text-slate-900" x-text="currentEntity ? currentEntity.name : 'Loading...'"></h2>
                                    </div>
                                    <a
                                        x-bind:href="`/admin/collections?entity_id=${selectedEntityId}`"
                                        class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 shadow-sm hover:bg-slate-50"
                                    >
                                        "Cancel"
                                    </a>
                                </div>

                                <form x-on:submit.prevent="submit()" class="space-y-5">
                                    <template x-if="loading">
                                        <div class="flex items-center gap-2 text-sm text-slate-500">
                                            <i data-feather="loader" class="h-4 w-4 animate-spin"></i>
                                            "Loading record..."
                                        </div>
                                    </template>

                                    <div x-show="!loading && currentEntity" class="grid gap-5 md:grid-cols-2">
                                        <template x-for="attribute in (currentEntity ? currentEntity.attributes : [])" x-bind:key="attribute.id">
                                            <div class="space-y-2" x-bind:class="attribute.field_type === 'textarea' ? 'md:col-span-2' : ''">
                                                <label class="block text-sm font-medium text-slate-700" x-bind:for="`field-${attribute.identifier}`" x-text="attribute.name"></label>

                                                <template x-if="attribute.field_type === 'textarea'">
                                                    <textarea
                                                        x-bind:id="`field-${attribute.identifier}`"
                                                        x-model="formData[attribute.identifier]"
                                                        rows="4"
                                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                                    ></textarea>
                                                </template>

                                                <template x-if="attribute.field_type === 'checkbox'">
                                                    <label class="inline-flex items-center gap-2 rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-700">
                                                        <input type="checkbox" x-model="formData[attribute.identifier]" class="h-4 w-4 rounded border-slate-300 text-primary-600 focus:ring-primary-500" />
                                                        <span x-text="attribute.name"></span>
                                                    </label>
                                                </template>

                                                <template x-if="attribute.field_type === 'number' || attribute.data_type === 'integer' || attribute.data_type === 'float' || attribute.data_type === 'number'">
                                                    <input
                                                        x-bind:id="`field-${attribute.identifier}`"
                                                        type="number"
                                                        x-model.number="formData[attribute.identifier]"
                                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                                    />
                                                </template>

                                                <template x-if="attribute.field_type === 'date' || attribute.data_type === 'date'">
                                                    <input
                                                        x-bind:id="`field-${attribute.identifier}`"
                                                        type="date"
                                                        x-model="formData[attribute.identifier]"
                                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                                    />
                                                </template>

                                                <template x-if="attribute.field_type !== 'textarea' && attribute.field_type !== 'checkbox' && attribute.field_type !== 'number' && attribute.data_type !== 'integer' && attribute.data_type !== 'float' && attribute.data_type !== 'number' && attribute.field_type !== 'date' && attribute.data_type !== 'date'">
                                                    <input
                                                        x-bind:id="`field-${attribute.identifier}`"
                                                        type="text"
                                                        x-model="formData[attribute.identifier]"
                                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                                    />
                                                </template>
                                            </div>
                                        </template>
                                    </div>

                                    <div class="flex items-center justify-end gap-3 border-t border-slate-200 pt-5">
                                        <a
                                            x-bind:href="`/admin/collections?entity_id=${selectedEntityId}`"
                                            class="inline-flex items-center justify-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100"
                                        >
                                            "Cancel"
                                        </a>
                                        <button
                                            type="submit"
                                            x-bind:disabled="submitting || loading"
                                            class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 disabled:cursor-not-allowed disabled:opacity-60"
                                        >
                                            <i data-feather="loader" class="h-4 w-4 animate-spin" x-show="submitting"></i>
                                            <span x-text="submitting ? 'Saving...' : 'Save changes'"></span>
                                        </button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
