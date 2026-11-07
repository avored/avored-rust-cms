use leptos::prelude::*;

#[component]
pub fn SettingPage() -> impl IntoView {
    view! {
        <div x-data="settingsPage()" x-init="init()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8 pb-24">
            <div class="mx-auto max-w-5xl">
                <div class="mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
                    <div>
                        <h1 class="text-3xl font-bold tracking-tight text-slate-900">"Settings"</h1>
                        <p class="mt-1 text-sm text-slate-500">
                            "Manage global CMS preferences, metadata, email delivery, and integrations."
                        </p>
                    </div>

                    <div class="flex items-center gap-3">
                        <button
                            type="button"
                            x-on:click="discardChanges()"
                            x-bind:disabled="!isDirty || saving"
                            class="rounded-lg border border-slate-300 bg-white px-4 py-2 text-sm font-medium text-slate-700 shadow-sm transition hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-primary-500 disabled:opacity-40"
                        >
                            "Discard"
                        </button>
                        <button
                            type="button"
                            x-on:click="saveSettings()"
                            x-bind:disabled="saving"
                            class="inline-flex items-center gap-2 rounded-lg bg-primary-600 px-5 py-2 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 disabled:opacity-50"
                        >
                            <span x-show="!saving">"Save Changes"</span>
                            <span x-show="saving" class="flex items-center gap-1.5">
                                <i data-feather="loader" class="h-4 w-4 animate-spin"></i>
                                "Saving..."
                            </span>
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

                <div class="mb-6 border-b border-slate-200">
                    <nav class="-mb-px flex space-x-8" aria-label="Tabs">
                        <button
                            type="button"
                            x-on:click="activeTab = 'general'"
                            x-bind:class="activeTab === 'general' ? 'border-primary-600 text-primary-600 font-semibold' : 'border-transparent text-slate-500 hover:border-slate-300 hover:text-slate-700'"
                            class="flex items-center gap-2 whitespace-nowrap border-b-2 py-3 px-1 text-sm transition-colors"
                        >
                            <i data-feather="globe" class="h-4 w-4"></i>
                            "General"
                        </button>
                        <button
                            type="button"
                            x-on:click="activeTab = 'seo'"
                            x-bind:class="activeTab === 'seo' ? 'border-primary-600 text-primary-600 font-semibold' : 'border-transparent text-slate-500 hover:border-slate-300 hover:text-slate-700'"
                            class="flex items-center gap-2 whitespace-nowrap border-b-2 py-3 px-1 text-sm transition-colors"
                        >
                            <i data-feather="search" class="h-4 w-4"></i>
                            "SEO & Metadata"
                        </button>
                        <button
                            type="button"
                            x-on:click="activeTab = 'email'"
                            x-bind:class="activeTab === 'email' ? 'border-primary-600 text-primary-600 font-semibold' : 'border-transparent text-slate-500 hover:border-slate-300 hover:text-slate-700'"
                            class="flex items-center gap-2 whitespace-nowrap border-b-2 py-3 px-1 text-sm transition-colors"
                        >
                            <i data-feather="mail" class="h-4 w-4"></i>
                            "Email & Delivery"
                        </button>

                    </nav>
                </div>

                <div x-show="loading" class="flex items-center justify-center p-16 text-sm text-slate-500">
                    <i data-feather="loader" class="mr-2 h-5 w-5 animate-spin"></i>
                    "Loading settings..."
                </div>

                <div x-show="!loading" class="space-y-6">
                    <div x-show="activeTab === 'general'" class="rounded-xl border border-slate-200 bg-white p-6 sm:p-8 shadow-sm space-y-6">
                        <div class="border-b border-slate-100 pb-4">
                            <h2 class="text-base font-semibold text-slate-900">"General Information"</h2>
                            <p class="text-xs text-slate-500">"Basic properties that identify and locate your website."</p>
                        </div>

                        <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
                            <div class="sm:col-span-2">
                                <label for="site_name" class="block text-sm font-medium text-slate-700">"Site Name"</label>
                                <input
                                    id="site_name"
                                    type="text"
                                    x-model="form.general.site_name"
                                    x-on:input="markDirty()"
                                    required
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="AvoRed CMS"
                                />
                            </div>

                            <div class="sm:col-span-2">
                                <label for="logo_url" class="block text-sm font-medium text-slate-700">"Logo URL"</label>
                                <input
                                    id="logo_url"
                                    type="text"
                                    x-model="form.general.logo_url"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="/assets/images/avored.svg"
                                />
                            </div>

                            <div>
                                <label for="support_email" class="block text-sm font-medium text-slate-700">"Support Email"</label>
                                <input
                                    id="support_email"
                                    type="email"
                                    x-model="form.general.support_email"
                                    x-on:input="markDirty()"
                                    required
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="support@example.com"
                                />
                            </div>

                            <div>
                                <label for="timezone" class="block text-sm font-medium text-slate-700">"Timezone"</label>
                                <input
                                    id="timezone"
                                    type="text"
                                    x-model="form.general.timezone"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="UTC"
                                />
                            </div>

                            <div>
                                <label for="default_language" class="block text-sm font-medium text-slate-700">"Default Language"</label>
                                <input
                                    id="default_language"
                                    type="text"
                                    x-model="form.general.default_language"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="en"
                                />
                            </div>
                        </div>
                    </div>

                    <div x-show="activeTab === 'seo'" class="rounded-xl border border-slate-200 bg-white p-6 sm:p-8 shadow-sm space-y-6">
                        <div class="border-b border-slate-100 pb-4">
                            <h2 class="text-base font-semibold text-slate-900">"Search Engine & Social Metadata"</h2>
                            <p class="text-xs text-slate-500">"Defaults applied to pages without specific SEO configurations."</p>
                        </div>

                        <div class="space-y-5">
                            <div>
                                <label for="meta_title" class="block text-sm font-medium text-slate-700">"Default Meta Title"</label>
                                <input
                                    id="meta_title"
                                    type="text"
                                    x-model="form.seo.meta_title"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="AvoRed CMS"
                                />
                            </div>

                            <div>
                                <label for="meta_description" class="block text-sm font-medium text-slate-700">"Default Meta Description"</label>
                                <textarea
                                    id="meta_description"
                                    rows="3"
                                    x-model="form.seo.meta_description"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="A modular, blazing fast open-source CMS built with Rust."
                                ></textarea>
                            </div>

                            <div>
                                <label for="og_image_url" class="block text-sm font-medium text-slate-700">"Social Share Image (OG Image URL)"</label>
                                <input
                                    id="og_image_url"
                                    type="text"
                                    x-model="form.seo.og_image_url"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="https://example.com/og-image.jpg"
                                />
                            </div>
                        </div>
                    </div>

                    <div x-show="activeTab === 'email'" class="rounded-xl border border-slate-200 bg-white p-6 sm:p-8 shadow-sm space-y-6">
                        <div class="border-b border-slate-100 pb-4">
                            <h2 class="text-base font-semibold text-slate-900">"Outbound Email Configuration"</h2>
                            <p class="text-xs text-slate-500">"Connection properties for SMTP notification delivery."</p>
                        </div>

                        <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
                            <div>
                                <label for="smtp_host" class="block text-sm font-medium text-slate-700">"SMTP Host"</label>
                                <input
                                    id="smtp_host"
                                    type="text"
                                    x-model="form.email.smtp_host"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="smtp.mailtrap.io"
                                />
                            </div>

                            <div>
                                <label for="smtp_port" class="block text-sm font-medium text-slate-700">"SMTP Port"</label>
                                <input
                                    id="smtp_port"
                                    type="text"
                                    x-model="form.email.smtp_port"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="587"
                                />
                            </div>

                            <div>
                                <label for="sender_name" class="block text-sm font-medium text-slate-700">"Default Sender Name"</label>
                                <input
                                    id="sender_name"
                                    type="text"
                                    x-model="form.email.sender_name"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="AvoRed CMS"
                                />
                            </div>

                            <div>
                                <label for="default_reply_to" class="block text-sm font-medium text-slate-700">"Default Reply-To"</label>
                                <input
                                    id="default_reply_to"
                                    type="email"
                                    x-model="form.email.default_reply_to"
                                    x-on:input="markDirty()"
                                    class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                    placeholder="no-reply@example.com"
                                />
                            </div>
                        </div>
                    </div>
                </div>

                <div
                    x-show="isDirty"
                    class="fixed bottom-6 left-1/2 -translate-x-1/2 flex items-center gap-4 rounded-xl border border-slate-200 bg-white/95 px-6 py-3.5 shadow-xl backdrop-blur-md transition-all z-40"
                >
                    <span class="flex items-center gap-2 text-xs font-medium text-amber-700">
                        <i data-feather="alert-circle" class="h-4 w-4"></i>
                        "You have unsaved changes"
                    </span>
                    <div class="h-4 w-px bg-slate-200"></div>
                    <button
                        type="button"
                        x-on:click="discardChanges()"
                        x-bind:disabled="saving"
                        class="text-xs font-semibold text-slate-600 hover:text-slate-900"
                    >
                        "Discard"
                    </button>
                    <button
                        type="button"
                        x-on:click="saveSettings()"
                        x-bind:disabled="saving"
                        class="inline-flex items-center gap-1.5 rounded-lg bg-primary-600 px-4 py-1.5 text-xs font-semibold text-white shadow-sm hover:bg-primary-700 disabled:opacity-50"
                    >
                        <span x-show="!saving">"Save Changes"</span>
                        <span x-show="saving">"Saving..."</span>
                    </button>
                </div>
            </div>
        </div>
    }
}
