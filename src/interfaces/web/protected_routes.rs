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
    let is_logged_in: RwSignal<bool> = RwSignal::new(false);
    let auth_token = RwSignal::new(String::new());
    let logged_in_user = RwSignal::new(None);
    let auth_ready = RwSignal::new(false);

    log::info!("AUTH READY: logged_in={}", is_logged_in.get());

    #[cfg(target_arch = "wasm32")]
    {
        let storage_token = gloo_storage::LocalStorage::get::<String>("auth_token").ok();

        match storage_token {
            Some(token) if !token.trim().is_empty() => {
                auth_token.set(token.clone());
                is_logged_in.set(true);
                logged_in_user.set(Some(LoggedInUser {
                    id: "demo-user-id".to_string(),
                    name: "Demo User".to_string(),
                    email: "demo@avored.local".to_string(),
                }));
                log::info!("AUTH INIT: token found, value={}", token);
            }
            _ => {
                log::info!("AUTH INIT: no token found in localStorage");
            }
        }
    }

    auth_ready.set(true);
    log::info!("AUTH READY: logged_in={}", is_logged_in.get());

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

    move || {
        if auth_ready.get() && is_logged_in.get() {
            children().into_any()
        } else {
            fallback().into_any()
        }
    }
}
