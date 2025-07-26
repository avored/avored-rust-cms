use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div>
            "App layout"
            <Outlet />
        </div>
    }
}