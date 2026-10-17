use leptos::prelude::*;

#[component]
pub fn AdminProfilePage() -> impl IntoView {
    view! {
        <div x-data="adminProfilePage()" x-init="init()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-4xl">
                <div class="mb-8">
                    <h1 class="text-3xl font-semibold tracking-tight text-slate-900">"Admin Profile"</h1>
                    <p class="mt-2 text-sm text-slate-500">
                        "Manage your administrator profile details and account password."
                    </p>
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
                            x-on:click="activeTab = 'profile'"
                            x-bind:class="activeTab === 'profile' ? 'border-primary-600 text-primary-600 font-medium' : 'border-transparent text-slate-500 hover:border-slate-300 hover:text-slate-700'"
                            class="whitespace-nowrap border-b-2 py-3 px-1 text-sm transition-colors"
                        >
                            "Profile Details"
                        </button>
                        <button
                            type="button"
                            x-on:click="activeTab = 'password'"
                            x-bind:class="activeTab === 'password' ? 'border-primary-600 text-primary-600 font-medium' : 'border-transparent text-slate-500 hover:border-slate-300 hover:text-slate-700'"
                            class="whitespace-nowrap border-b-2 py-3 px-1 text-sm transition-colors"
                        >
                            "Change Password"
                        </button>
                    </nav>
                </div>

                <div x-show="activeTab === 'profile'" class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <div x-show="loading" class="flex items-center justify-center p-12 text-sm text-slate-500">
                        <i data-feather="loader" class="mr-2 h-5 w-5 animate-spin"></i>
                        "Loading profile details..."
                    </div>

                    <form x-show="!loading" x-on:submit.prevent="submitProfile()" class="p-6 sm:p-8 space-y-6">
                        <div>
                            <label for="name" class="block text-sm font-medium text-slate-700">"Full Name"</label>
                            <input
                                id="name"
                                type="text"
                                x-model="profileForm.name"
                                required
                                class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                placeholder="Your Name"
                            />
                        </div>

                        <div>
                            <label for="email" class="block text-sm font-medium text-slate-700">"Email Address"</label>
                            <input
                                id="email"
                                type="email"
                                x-model="profileForm.email"
                                required
                                class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                placeholder="admin@example.com"
                            />
                        </div>

                        <div class="flex justify-end pt-4 border-t border-slate-100">
                            <button
                                type="submit"
                                x-bind:disabled="submittingProfile"
                                class="inline-flex items-center gap-2 rounded-lg bg-primary-600 px-4 py-2 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 disabled:opacity-50"
                            >
                                <span x-show="!submittingProfile">"Save Changes"</span>
                                <span x-show="submittingProfile">"Saving..."</span>
                            </button>
                        </div>
                    </form>
                </div>

                <div 
                    x-show="activeTab === 'password'" 
                    class="overflow-hidden rounded-xl border border-slate-200 bg-white shadow-sm">
                    <form x-on:submit.prevent="submitPassword()" class="p-6 sm:p-8 space-y-6">
                        <div>
                            <label for="current_password" class="block text-sm font-medium text-slate-700">"Current Password"</label>
                            <input
                                id="current_password"
                                type="password"
                                x-model="passwordForm.current_password"
                                required
                                class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                placeholder="••••••••"
                            />
                        </div>

                        <div>
                            <label for="new_password" class="block text-sm font-medium text-slate-700">"New Password"</label>
                            <input
                                id="new_password"
                                type="password"
                                x-model="passwordForm.new_password"
                                required
                                class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                placeholder="Minimum 6 characters"
                            />
                        </div>

                        <div>
                            <label for="confirm_password" class="block text-sm font-medium text-slate-700">"Confirm New Password"</label>
                            <input
                                id="confirm_password"
                                type="password"
                                x-model="passwordForm.confirm_password"
                                required
                                class="mt-1 block w-full rounded-lg border border-slate-300 px-3 py-2 text-sm text-slate-900 shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                                placeholder="Repeat new password"
                            />
                        </div>

                        <div class="flex justify-end pt-4 border-t border-slate-100">
                            <button
                                type="submit"
                                x-bind:disabled="submittingPassword"
                                class="inline-flex items-center gap-2 rounded-lg bg-primary-600 px-4 py-2 text-sm font-semibold text-white shadow-sm transition hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 disabled:opacity-50"
                            >
                                <span x-show="!submittingPassword">"Update Password"</span>
                                <span x-show="submittingPassword">"Updating..."</span>
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
