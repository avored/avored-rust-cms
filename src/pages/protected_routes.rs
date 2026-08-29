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
}

// Function to provide the AuthContext
pub fn provide_auth_context() {
    let is_logged_in = RwSignal::new(false);
    let auth_token = RwSignal::new(String::new());
    let logged_in_user = RwSignal::new(None);

    // On the client, read from local storage
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            if let Ok(token) = gloo_storage::LocalStorage::get::<String>("auth_token") {
                auth_token.set(token.clone());
                is_logged_in.set(!token.trim().is_empty());

                if !token.trim().is_empty() {
                    let token_for_request = token.clone();
                    let _logged_in_user_signal = logged_in_user;

                    leptos::task::spawn_local(async move {
                        let request = gloo_net::http::Request::get("/api/admin/profile")
                            .header("Authorization", format!("Bearer {}", token_for_request).as_str());

                        match request.send().await {
                            Ok(response) if response.ok() => {
                                // if let Ok(profile_response) = response.json::<GetProfileResponse>().await {
                                //     logged_in_user_signal.set(Some(LoggedInUser {
                                //         id: profile_response.data.id.to_string(),
                                //         name: profile_response.data.name,
                                //         email: profile_response.data.email,
                                //     }));
                                // }
                            }
                            Ok(_) => {}
                            Err(err) => {
                                log::warn!("Failed to fetch logged-in profile: {:?}", err);
                            }
                        }
                    });
                } else {
                    logged_in_user.set(None);
                }
            } else {
                logged_in_user.set(None);
            }
        });
    }

    provide_context(AuthContext {
        is_logged_in,
        auth_token,
        logged_in_user,
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
