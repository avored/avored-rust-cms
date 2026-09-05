use leptos::{prelude::*};
use leptos_router::hooks::use_navigate;
// use gloo_storage::{LocalStorage, Storage};
// use anyhow::Result;
#[cfg(target_arch = "wasm32")]
use gloo_storage::Storage;


#[derive(Clone, Default)]
pub struct AuthContext {
    pub is_logged_in: RwSignal<bool>,
    pub auth_token: RwSignal<String>,
    pub full_name: RwSignal<String>,
    pub is_super_admin: RwSignal<bool>,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct AdminUserContext {
    pub full_name: String,
    pub email: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: String,
    pub updated_at: String,

}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthData {
    pub admin_user: AdminUserContext,
    pub token: String,
}

// Function to provide the AuthContext
pub fn provide_auth_context() {
    let is_logged_in = RwSignal::new(false);
    let auth_token = RwSignal::new(String::new());
    let full_name = RwSignal::new(String::new());
    let is_super_admin = RwSignal::new(false);

    // On the client, read from local storage
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            // Need to handle this carefully to avoid panic on server if this code path is somehow reached,
            // but cfg! guards it.
            // Also need to ensure gloo_storage is available or use runtime check if needed, but strict cfg is safer.
            // Using a block to contain the usage.
            if let Ok(value) = gloo_storage::LocalStorage::get::<String>("avored_admin_token") {
                if let Ok(auth_data) = serde_json::from_str::<AuthData>(&value) {
                    auth_token.set(auth_data.token.clone());
                    is_logged_in.set(!auth_data.token.is_empty());
                    full_name.set(auth_data.admin_user.full_name.clone());
                    is_super_admin.set(auth_data.admin_user.is_super_admin);
                }
            }
        });
    }
    provide_context(AuthContext {
        is_logged_in,
        auth_token,
        full_name,
        is_super_admin,
    });
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
