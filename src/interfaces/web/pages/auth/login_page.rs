use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use rust_i18n::t;
use serde::{Deserialize, Serialize};

use crate::interfaces::web::protected_routes::{AuthContext, LoggedInUser};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginApiResponse {
    pub token: String,
    pub user: LoginUser,
    pub authenticated: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let email = RwSignal::new("demo@avored.local".to_string());
    let password = RwSignal::new("password1234".to_string());
    let error = RwSignal::new(String::new());
    let submitting = RwSignal::new(false);
    let auth_context = use_context::<AuthContext>().expect("AuthContext should be provided");
    let navigate = use_navigate();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if submitting.get() {
            return;
        }

        let email_value = email.get();
        let password_value = password.get();
        let auth_context = auth_context.clone();
        let navigate = navigate.clone();

        submitting.set(true);
        error.set(String::new());

        leptos::task::spawn_local(async move {
            let request_body = serde_json::to_string(&LoginRequest {
                email: email_value,
                password: password_value,
            })
            .unwrap_or_default();

            let request = gloo_net::http::Request::post("/api/auth/login")
                .header("Content-Type", "application/json")
                .body(request_body)
                .unwrap();

            match request.send().await {
                Ok(response) if response.ok() => match response.json::<LoginApiResponse>().await {
                    Ok(login_response) => {
                        if login_response.authenticated {
                            let token = login_response.token.clone();
                            let user = login_response.user.clone();

                            let _ = LocalStorage::set("auth_token", token.clone());

                            auth_context.is_logged_in.set(true);
                            auth_context.auth_token.set(token.clone());
                            auth_context.logged_in_user.set(Some(LoggedInUser {
                                id: user.id,
                                name: user.name,
                                email: user.email,
                            }));

                            log::info!("Login successful, redirecting to /admin");
                            navigate("/admin", Default::default());
                        } else {
                            error.set("Demo login failed.".to_string());
                        }
                    }
                    Err(err) => {
                        error.set(format!("Failed to parse login response: {err}"));
                    }
                },
                Ok(response) => {
                    let payload = response.text().await.unwrap_or_default();
                    error.set(format!("Login failed: {payload}"));
                }
                Err(err) => {
                    error.set(format!("Unable to reach login API: {err}"));
                }
            }

            submitting.set(false);
        });
    };

    view! {
        <div class="flex min-h-screen items-center justify-center bg-slate-100 px-4">
            <div class="w-full max-w-md rounded-lg border border-slate-200 bg-white p-8 shadow-lg">
                <div class="mb-6 text-center">
                    <h1 class="text-3xl font-bold text-slate-800">
                        { t!("welcome_avored") }
                    </h1>
                </div>

                <form on:submit=on_submit class="space-y-4">
                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            { t!("email") }
                        </label>
                        <input
                            type="email"
                            prop:value=move || email.get()
                            on:input=move |ev| email.set(event_target_value(&ev))
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            placeholder={t!("email_address")}
                        />
                    </div>

                    <div>
                        <label class="mb-1 block text-sm font-medium text-slate-700">
                            { t!("password") }
                        </label>
                        <input
                            type="password"
                            prop:value=move || password.get()
                            on:input=move |ev| password.set(event_target_value(&ev))
                            class="w-full rounded-md border border-slate-300 px-3 py-2 outline-none ring-0 transition focus:border-primary-500"
                            placeholder={t!("password")}
                        />
                    </div>


                    {move || {
                        if !error.get().is_empty() {
                            view! {
                                <div class="rounded-md border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">
                                    {error.get()}
                                </div>
                            }
                            .into_any()
                        } else {
                            ().into_view().into_any()
                        }
                    }}
                    
                    <button
                        type="submit"
                        disabled=move || submitting.get()
                        class="w-full rounded-md bg-primary-500 px-4 py-2 text-sm font-semibold text-white transition hover:bg-primary-600 disabled:cursor-not-allowed disabled:bg-primary-300"
                    >
                        {move || if submitting.get() { t!("signing_in") } else { t!("sign_in") }}
                    </button>
                </form>
            </div>
        </div>
    }
}
