use leptos::prelude::*;
use rust_i18n::t;

#[component]
pub fn SetupPage() -> impl IntoView {
    view! {
        <div x-data="setupPage()" class="flex min-h-screen items-center justify-center bg-slate-100 px-4">
            <div class="w-full max-w-md rounded-lg border border-slate-200 bg-white p-8 shadow-lg">
                <div class="mb-6 text-center">
                    <h1 class="text-3xl font-bold text-slate-800">
                        {t!("setup_avored")}
                    </h1>
                </div>

                // General error banner
                <template x-if="errorMessage">
                    <div class="mb-4 rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" role="alert">
                        <span x-text="errorMessage"></span>
                    </div>
                </template>

                <form x-on:submit.prevent="handleSubmit" class="space-y-4">
                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            {t!("name")}
                        </label>
                        <input
                            type="text"
                            x-model="name"
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            x-bind:class="fieldError('name') ? 'border-red-400 focus:border-red-500' : ''"
                            placeholder={t!("name")}
                        />
                        <template x-if="fieldError('name')">
                            <p class="mt-1 text-xs text-red-600" x-text="fieldError('name')"></p>
                        </template>
                    </div>

                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            {t!("email")}
                        </label>
                        <input
                            type="email"
                            x-model="email"
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            x-bind:class="fieldError('email') ? 'border-red-400 focus:border-red-500' : ''"
                            placeholder={t!("email_address")}
                        />
                        <template x-if="fieldError('email')">
                            <p class="mt-1 text-xs text-red-600" x-text="fieldError('email')"></p>
                        </template>
                    </div>

                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            {t!("password")}
                        </label>
                        <input
                            type="password"
                            x-model="password"
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            x-bind:class="fieldError('password') ? 'border-red-400 focus:border-red-500' : ''"
                            placeholder={t!("password")}
                        />
                        <template x-if="fieldError('password')">
                            <p class="mt-1 text-xs text-red-600" x-text="fieldError('password')"></p>
                        </template>
                    </div>

                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            {t!("confirm_password")}
                        </label>
                        <input
                            type="password"
                            x-model="confirmPassword"
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            x-bind:class="fieldError('confirm_password') ? 'border-red-400 focus:border-red-500' : ''"
                            placeholder={t!("confirm_password")}
                        />
                        <template x-if="fieldError('confirm_password')">
                            <p class="mt-1 text-xs text-red-600" x-text="fieldError('confirm_password')"></p>
                        </template>
                    </div>

                    <button
                        type="submit"
                        class="btn btn-primary"
                        x-bind:disabled="submitting"
                    >
                        { t!("setup") }
                    </button>
                </form>
            </div>
        </div>
    }
}
