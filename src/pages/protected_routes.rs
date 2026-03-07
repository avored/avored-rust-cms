use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
// use gloo_storage::{LocalStorage, Storage};
// use anyhow::Result;
#[cfg(target_arch = "wasm32")]
use gloo_storage::Storage;

#[derive(Clone, Default)]
pub struct AuthContext {
    pub is_logged_in: RwSignal<bool>,
    pub auth_token: RwSignal<String>,
}

// Function to provide the AuthContext
pub fn provide_auth_context() {
    let is_logged_in = RwSignal::new(false);
    let auth_token = RwSignal::new(String::new());
    // On the client, read from local storage
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            // Need to handle this carefully to avoid panic on server if this code path is somehow reached,
            // but cfg! guards it.
            // Also need to ensure gloo_storage is available or use runtime check if needed, but strict cfg is safer.
            // Using a block to contain the usage.
            if let Ok(value) = gloo_storage::LocalStorage::get::<String>("avored_admin_token") {
                auth_token.set(value.clone());
                is_logged_in.set(!value.is_empty());
            }
        });
    }
    provide_context(AuthContext { is_logged_in, auth_token });
}

#[component]
pub fn ProtectedRoute(children: ChildrenFn, fallback: ChildrenFn) -> impl IntoView
{
    let auth_context = use_context::<AuthContext>().expect("AuthContext should be provided");
    let is_logged_in = auth_context.is_logged_in;
    let navigate = use_navigate();

    Effect::new(move |_| {
        if !is_logged_in.get() {
            // Use navigate for client-side redirection
            navigate("/auth/login", Default::default());
        }
    });

    move || {
        if is_logged_in.get() {
            children().into_any()
        } else {
            // Render a fallback on the server or while waiting for client-side check
            fallback().into_any()
        }
    }
}
