use leptos::prelude::*;
use rust_i18n::t;

#[component]
pub fn EntityIndexPage() -> impl IntoView {
    view! {
        <div x-data="entityIndexPage()" class="p-6">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h1 class="text-2xl font-bold text-gray-800">
                        {t!("entities")}
                    </h1>
                    <p class="text-sm text-gray-500">
                        "Manage your dynamic content types and attributes"
                    </p>
                </div>
                <a
                    href="/admin/entity/create"
                    class="rounded-md bg-primary-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-primary-700 transition"
                >
                    {t!("create_entity")}
                </a>
            </div>

            // Task 1.5: UI table with Alpine data binding
            <div class="rounded-lg border border-gray-200 bg-white p-6 text-center text-gray-500">
                "Entity list skeleton (Scaffolding Phase)"
            </div>
        </div>
    }
}
