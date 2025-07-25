use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
            <h1 class="text-4xl font-bold text-primary-700 mb-4">
                "Avored Rust CMS"
            </h1>
            <p class="text-gray-600">
                "Welcome to the Avored Rust CMS admin panel"
            </p>
        </div>
    }
}