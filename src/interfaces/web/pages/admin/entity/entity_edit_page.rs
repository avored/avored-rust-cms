use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use rust_i18n::t;

#[component]
pub fn EntityEditPage() -> impl IntoView {
    let params = use_params_map();
    let entity_id = move || params.read().get("id").unwrap_or_default();

    view! {
        <div x-data=format!("entityEditPage('{}')", entity_id()) class="p-6">
            <div class="mb-6">
                <h1 class="text-2xl font-bold text-gray-800">
                    {t!("edit_entity")}
                </h1>
            </div>

            // Task 1.5: UI form with Alpine data binding
            <div class="rounded-lg border border-gray-200 bg-white p-6 text-center text-gray-500">
                "Entity edit form skeleton (Scaffolding Phase)"
            </div>
        </div>
    }
}
