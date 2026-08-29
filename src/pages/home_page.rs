use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1 class="text-2xl font-bold">"Avored CMS!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
