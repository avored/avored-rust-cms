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

                <form x-on:submit.prevent="handleSubmit" class="space-y-4">
                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            {t!("name")}
                        </label>
                        <input
                            type="text"
                            x-model="name"
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            placeholder={t!("name")}
                        />
                    </div>

                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            {t!("email")}
                        </label>
                        <input
                            type="email"
                            x-model="email"
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            placeholder={t!("email_address")}
                        />
                    </div>

                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            {t!("password")}
                        </label>
                        <input
                            type="password"
                            x-model="password"
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            placeholder={t!("password")}
                        />
                    </div>

                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            {t!("confirm_password")}
                        </label>
                        <input
                            type="password"
                            x-model="confirmPassword"
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            placeholder={t!("confirm_password")}
                        />
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
