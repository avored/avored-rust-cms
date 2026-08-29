use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
// use gloo_storage::{LocalStorage, Storage};
// use anyhow::Result;
#[cfg(target_arch = "wasm32")]
use gloo_storage::Storage;


#[derive(Clone, Default, Debug)]
pub struct LoggedInUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Default)]
pub struct AuthContext {
    pub is_logged_in: RwSignal<bool>,
    pub auth_token: RwSignal<String>,
    pub logged_in_user: RwSignal<Option<LoggedInUser>>,
    pub auth_ready: RwSignal<bool>,
}

// Function to provide the AuthContext
pub fn provide_auth_context() {
    #[cfg(target_arch = "wasm32")]
    let (initial_logged_in, initial_token, initial_user) = {
        let storage_token = gloo_storage::LocalStorage::get::<String>("auth_token").ok();
        match storage_token {
            Some(token) if !token.trim().is_empty() => (
                true,
                token,
                Some(LoggedInUser {
                    id: "demo-user-id".to_string(),
                    name: "Demo User".to_string(),
                    email: "demo@avored.local".to_string(),
                }),
            ),
            _ => (false, String::new(), None),
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let (initial_logged_in, initial_token, initial_user) = (false, String::new(), None);

    let is_logged_in: RwSignal<bool> = RwSignal::new(initial_logged_in);
    let auth_token = RwSignal::new(initial_token);
    let logged_in_user = RwSignal::new(initial_user);
    let auth_ready = RwSignal::new(true);

    log::info!("AUTH INIT: logged_in={}", initial_logged_in);

    provide_context(AuthContext {
        is_logged_in,
        auth_token,
        logged_in_user,
        auth_ready,
    });
}

#[component]
pub fn ProtectedRoute<C, F>(children: C, fallback: F) -> impl IntoView
where
    C: Fn() -> Vec<AnyView> + 'static + Send,
    F: Fn() -> AnyView + 'static + Send,
{
    let auth_context = use_context::<AuthContext>().expect("AuthContext should be provided");
    let is_logged_in = auth_context.is_logged_in;
    let auth_ready = auth_context.auth_ready;
    let navigate = use_navigate();

    Effect::new(move |_| {
        let ready = auth_ready.get();
        let logged_in = is_logged_in.get();

        log::info!("PROTECTED ROUTE CHECK: ready={}, logged_in={}", ready, logged_in);

        if ready && !logged_in {
            log::info!("PROTECTED ROUTE: redirecting to /auth/login");
            navigate("/auth/login", Default::default());
        }
    });

    view! {
        {move || {
            if auth_ready.get() && is_logged_in.get() {
                children().into_any()
            } else {
                fallback().into_any()
            }
        }}
    }
}
