use leptos::prelude::*;
use rust_i18n::t;

#[component]
pub fn EntityCreatePage() -> impl IntoView {
    view! {
        <div x-data="entityCreatePage()" class="p-6">
            <div class="mb-6">
                <h1 class="text-2xl font-bold text-gray-800">
                    {t!("create_entity")}
                </h1>
            </div>

            // Task 1.5: UI form with Alpine data binding
            <div class="rounded-lg border border-gray-200 bg-white p-6 text-center text-gray-500">
                "Entity create form skeleton (Scaffolding Phase)"
            </div>
        </div>
    }
}
