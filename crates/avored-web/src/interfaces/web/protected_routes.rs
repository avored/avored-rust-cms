#[cfg(target_arch = "wasm32")]
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use gloo_storage::Storage;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct LoggedInUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[cfg(target_arch = "wasm32")]
fn decode_jwt_payload(token: &str) -> Option<serde_json::Value> {
    let mut parts = token.split('.');
    let _header = parts.next()?;
    let payload = parts.next()?;
    let _signature = parts.next()?;

    let padded =
        payload.replace('-', "+").replace('_', "/") + &"=".repeat((4 - payload.len() % 4) % 4);

    let decoded = URL_SAFE_NO_PAD.decode(padded).ok()?;
    serde_json::from_slice::<serde_json::Value>(&decoded).ok()
}

#[cfg(target_arch = "wasm32")]
fn read_logged_in_user_from_token(token: &str) -> Option<LoggedInUser> {
    let claims = decode_jwt_payload(token)?;
    let claims = claims.as_object()?;

    let exp = claims
        .get("exp")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(i64::MAX);
    let now = chrono::Utc::now().timestamp();

    if exp <= now {
        return None;
    }

    let id = claims
        .get("id")
        .or_else(|| claims.get("sub"))
        .or_else(|| claims.get("user_id"))
        .or_else(|| claims.get("userId"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .unwrap_or_default();

    let name = claims
        .get("name")
        .or_else(|| claims.get("full_name"))
        .or_else(|| claims.get("fullName"))
        .or_else(|| claims.get("display_name"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .unwrap_or_default();

    let email = claims
        .get("email")
        .or_else(|| claims.get("mail"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .unwrap_or_default();

    if id.is_empty() && name.is_empty() && email.is_empty() {
        return None;
    }

    Some(LoggedInUser { id, name, email })
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
        let stored_user = gloo_storage::LocalStorage::get::<LoggedInUser>("auth_user")
            .ok()
            .filter(|user| !(user.id.is_empty() && user.name.is_empty() && user.email.is_empty()));

        match storage_token {
            Some(token) if !token.trim().is_empty() => {
                let resolved_user = read_logged_in_user_from_token(&token)
                    .or(stored_user)
                    .filter(|user| {
                        !(user.id.is_empty() && user.name.is_empty() && user.email.is_empty())
                    });

                if resolved_user.is_some() {
                    (true, token, resolved_user)
                } else {
                    (false, String::new(), None)
                }
            }
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

        log::info!(
            "PROTECTED ROUTE CHECK: ready={}, logged_in={}",
            ready,
            logged_in
        );

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
