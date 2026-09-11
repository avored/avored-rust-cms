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
                            {t!("back_to_entities")}
                        </a>
                        <h1 class="text-3xl font-semibold tracking-tight text-slate-900">
                            {t!("create_entity")}
                        </h1>
                        <p class="mt-2 max-w-2xl text-sm leading-6 text-slate-500">
                            {t!("create_entity_description")}
                        </p>
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
                        <h2 class="text-base font-semibold text-slate-900">
                            {t!("entity_details")}
                        </h2>
                        <p class="mt-1 text-sm text-slate-500">
                            {t!("give_entity_clear_name")}
                        </p>
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
                                placeholder={t!("entity_name_placeholder")}
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
                                placeholder={t!("entity_identifier_placeholder")}
                            />
                            <template x-if="fieldError('identifier')">
                                <p id="entity-identifier-error" class="mt-2 text-xs font-medium text-red-600" x-text="fieldError('identifier')"></p>
                            </template>
                        </div>
                    </div>

                    <div class="border-t border-slate-200 bg-slate-50 px-5 py-6 sm:px-8">
                        <div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                            <div>
                                <h2 class="text-base font-semibold text-slate-900">"Attributes"</h2>
                                <p class="mt-1 text-sm text-slate-500">"Add the fields needed to describe this entity."</p>
                            </div>
                            <button
                                type="button"
                                x-on:click="openAttributeModal()"
                                class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                            >
                                <i data-feather="plus" class="h-4 w-4"></i>
                                "Add attribute"
                            </button>
                        </div>

                        <div class="mt-6 space-y-3">
                            <template x-if="attributes.length === 0">
                                <div class="rounded-xl border border-dashed border-slate-300 bg-white px-4 py-8 text-center">
                                    <i data-feather="layers" class="mx-auto h-8 w-8 text-slate-300"></i>
                                    <h3 class="mt-3 text-sm font-semibold text-slate-900">"No attributes yet"</h3>
                                    <p class="mt-1 text-sm text-slate-500">"Start with the first field for this entity."</p>
                                    <button
                                        type="button"
                                        x-on:click="openAttributeModal()"
                                        class="mt-4 inline-flex items-center justify-center rounded-lg border border-slate-300 bg-white px-3.5 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50"
                                    >
                                        "Add your first attribute"
                                    </button>
                                </div>
                            </template>

                            <template x-for="attribute in attributes" x-bind:key="attribute.id">
                                <div class="flex flex-col gap-3 rounded-xl border border-slate-200 bg-white p-4 sm:flex-row sm:items-center sm:justify-between">
                                    <div>
                                        <div class="flex items-center gap-2">
                                            <p class="font-medium text-slate-900" x-text="attribute.name"></p>
                                            <span class="rounded-full bg-primary-50 px-2 py-0.5 text-[11px] font-medium text-primary-700" x-text="attribute.field_type"></span>
                                        </div>
                                        <div class="mt-2 flex flex-wrap items-center gap-2 text-xs text-slate-500">
                                            <span class="rounded bg-slate-100 px-2 py-1 font-mono" x-text="attribute.identifier"></span>
                                            <span class="rounded bg-slate-100 px-2 py-1" x-text="attribute.data_type"></span>
                                        </div>
                                    </div>

                                    <div class="flex items-center gap-2">
                                        <button
                                            type="button"
                                            x-on:click="openAttributeModal(attribute)"
                                            class="inline-flex items-center justify-center gap-2 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50"
                                        >
                                            <i data-feather="edit-3" class="h-4 w-4"></i>
                                            "Edit"
                                        </button>
                                        <button
                                            type="button"
                                            x-on:click="removeAttribute(attribute.id)"
                                            class="inline-flex items-center justify-center rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50"
                                        >
                                            "Remove"
                                        </button>
                                    </div>
                                </div>
                            </template>
                        </div>
                    </div>

                    <div class="flex flex-col-reverse gap-3 border-t border-slate-200 bg-slate-50 px-5 py-4 sm:flex-row sm:justify-end sm:px-8">
                        <a
                            href="/admin/entity"
                            class="inline-flex items-center justify-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                        >
                            {t!("cancel")}
                        </a>
                        <button
                            type="submit"
                            class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-60"
                            x-bind:disabled="submitting"
                        >
                            <i data-feather="save" class="h-4 w-4" x-show="!submitting"></i>
                            <i data-feather="loader" class="h-4 w-4 animate-spin" x-show="submitting"></i>
                            <span x-show="!submitting">{t!("create_entity")}</span>
                            <span x-show="submitting">{t!("creating")}</span>
                        </button>
                    </div>
                </form>

                <div x-show="attributeModalOpen" x-cloak class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 px-4" x-on:keydown.escape.window="closeAttributeModal()">
                    <div class="w-full max-w-2xl rounded-xl bg-white p-6 shadow-xl" x-on:click.outside="closeAttributeModal()" role="dialog" aria-modal="true" aria-labelledby="attribute-modal-title">
                        <div class="flex items-start justify-between gap-4">
                            <div>
                                <h2 id="attribute-modal-title" class="text-lg font-semibold text-slate-900" x-text="editingAttributeId === null ? 'Add attribute' : 'Edit attribute'"></h2>
                                <p class="mt-1 text-sm text-slate-500">"Define the field details for this entity."</p>
                            </div>
                            <button
                                type="button"
                                x-on:click="closeAttributeModal()"
                                class="rounded-lg border border-slate-300 bg-white p-2 text-slate-500 transition hover:bg-slate-50 hover:text-slate-700"
                                aria-label="Close attribute modal"
                            >
                                <i data-feather="x" class="h-4 w-4"></i>
                            </button>
                        </div>

                        <form x-on:submit.prevent="submitAttribute" class="mt-6 space-y-5">
                            <div class="grid gap-5 md:grid-cols-2">
                                <div class="md:col-span-2">
                                    <label for="attribute-name" class="mb-2 block text-sm font-medium text-slate-700">"Name"</label>
                                    <input
                                        id="attribute-name"
                                        type="text"
                                        x-model="attributeDraft.name"
                                        x-on:input="handleAttributeNameChange"
                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                        placeholder="Attribute name"
                                    />
                                </div>

                                <div>
                                    <label for="attribute-identifier" class="mb-2 block text-sm font-medium text-slate-700">"Identifier"</label>
                                    <input
                                        id="attribute-identifier"
                                        type="text"
                                        x-model="attributeDraft.identifier"
                                        x-on:input="handleAttributeIdentifierInput"
                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 font-mono text-sm text-slate-900 outline-none transition placeholder:font-sans placeholder:text-slate-400 focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                        placeholder="attribute_identifier"
                                    />
                                </div>

                                <div>
                                    <label for="attribute-data-type" class="mb-2 block text-sm font-medium text-slate-700">"Data type"</label>
                                    <select
                                        id="attribute-data-type"
                                        x-model="attributeDraft.data_type"
                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                    >
                                        <option value="string">"string"</option>
                                        <option value="integer">"integer"</option>
                                        <option value="boolean">"boolean"</option>
                                        <option value="date">"date"</option>
                                        <option value="json">"json"</option>
                                    </select>
                                </div>

                                <div class="md:col-span-2">
                                    <label for="attribute-field-type" class="mb-2 block text-sm font-medium text-slate-700">"Field type"</label>
                                    <select
                                        id="attribute-field-type"
                                        x-model="attributeDraft.field_type"
                                        class="w-full rounded-lg border border-slate-300 px-3.5 py-2.5 text-sm text-slate-900 outline-none transition focus:border-primary-500 focus:ring-2 focus:ring-primary-100"
                                    >
                                        <option value="text">"text"</option>
                                        <option value="textarea">"textarea"</option>
                                        <option value="number">"number"</option>
                                        <option value="checkbox">"checkbox"</option>
                                        <option value="select">"select"</option>
                                        <option value="date">"date"</option>
                                    </select>
                                </div>
                            </div>

                            <div class="flex flex-col-reverse gap-3 sm:flex-row sm:justify-end">
                                <button
                                    type="button"
                                    x-on:click="closeAttributeModal()"
                                    class="inline-flex items-center justify-center rounded-lg border border-slate-300 bg-white px-4 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100"
                                >
                                    {t!("cancel")}
                                </button>
                                <button
                                    type="submit"
                                    class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary-600 px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                                >
                                    <i data-feather="plus" class="h-4 w-4" x-show="editingAttributeId === null"></i>
                                    <i data-feather="save" class="h-4 w-4" x-show="editingAttributeId !== null"></i>
                                    <span x-show="editingAttributeId === null">"Add attribute"</span>
                                    <span x-show="editingAttributeId !== null">"Save changes"</span>
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
