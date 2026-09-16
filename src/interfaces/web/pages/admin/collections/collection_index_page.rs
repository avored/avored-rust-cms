use leptos::prelude::*;
use rust_i18n::t;

#[component]
pub fn CollectionIndexPage() -> impl IntoView {
    view! {
        <div x-data="collectionIndexPage()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-7xl">
                {t!("collections")}

                
            </div>
        </div>
    }
}
