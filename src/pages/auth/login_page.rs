use leptos::prelude::*;
use rust_i18n::t;
#[cfg(target_arch = "wasm32")]
use crate::pages::protected_routes::AuthContext;



#[cfg(feature = "ssr")]
pub mod ssr {
    use crate::infra::grpc::auth_user::{auth_client::AuthClient, LoginRequest};
    use leptos::prelude::ServerFnError;


    pub async fn login_user_grpc(email: String, password: String) -> Result<String, ServerFnError> {
        let mut client = AuthClient::connect("http://127.0.0.1:3000")
            .await
            .map_err(|e| ServerFnError::new(format!("Connection failed: {}", e)))?;

        let request = tonic::Request::new(LoginRequest { email, password });

        let response = client
            .login_user(request)
            .await
            .map_err(|e| ServerFnError::new(format!("gRPC error: {}", e)))?;

        let inner = response.into_inner();
        if inner.status {
            let grpc_login_user_response = inner.data.unwrap_or_default();
            let json = serde_json::to_string(&grpc_login_user_response).map_err(|e| ServerFnError::new(format!("JSON error: {}", e)))?;
            Ok(json)
        } else {
            Err(ServerFnError::new("Login failed: invalid response status"))
        }
    }
}

#[server(endpoint = "login-user")]
pub async fn login_user(email: String, password: String) -> Result<String, ServerFnError> {
    self::ssr::login_user_grpc(email, password).await
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_action = ServerAction::<LoginUser>::new();
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        login_action.dispatch(LoginUser {
            email: email.get(),
            password: password.get(),
        });
    };

    let login_result = login_action.value();

    #[cfg(target_arch = "wasm32")]
    let auth_context = use_context::<AuthContext>().expect("AuthContext should be provided");

    Effect::new(move |_| {
        if let Some(Ok(token)) = login_result.get() {
            if !token.is_empty() {
                #[cfg(target_arch = "wasm32")]
                {
                    use crate::pages::protected_routes::AuthData;
                    use gloo_storage::{LocalStorage, Storage};
                    let _ = LocalStorage::set("avored_admin_token", token.clone());
                    if let Ok(auth_data) = serde_json::from_str::<AuthData>(&token) {
                        auth_context.auth_token.set(auth_data.token);
                        auth_context.is_logged_in.set(true);
                        auth_context.full_name.set(auth_data.admin_user.full_name);
                        auth_context.is_super_admin.set(auth_data.admin_user.is_super_admin);
                    }

                    let navigate = leptos_router::hooks::use_navigate();
                    navigate("/admin/dashboard", Default::default());
                }
            }
        }
    });

    view! {
        <div class="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
                <div class="flex justify-center">
                    <img src="/public/images/avored.svg" class="w-20 h-20" alt="avored_rust_cms" />
                </div>

                <div class="sm:mx-auto sm:w-full sm:max-w-md">
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        {t!("sign_into_your_account")}
                    </h2>
                </div>
                /*** empty div for spacing ***/
                <div></div>


                <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
                    <div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
                        <form class="space-y-5" on:submit=on_submit>
                            <div>
                                <label class="block text-sm font-medium text-gray-500 mb-1" for="email">
                                    {t!("email_address")}
                                </label>
                                <input
                                    id="email"
                                    type="text"
                                    name="email"
                                    autofocus=true
                                    on:input=move |ev| email.set(event_target_value(&ev))
                                    prop:value=email
                                    class="appearance-none rounded-md ring-1 ring-gray-400
                                            relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900
                                            active::ring-primary-500
                                            focus:ring-primary-500 focus:outline-none focus:z-10
                                            disabled:bg-gray-200 disabled:opacity-70
                                            sm:text-sm"
                                />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-500 mb-1" for="password">
                                    {t!("password")}
                                </label>
                                <input
                                    id="password"
                                    type="password"
                                    on:input=move |ev| password.set(event_target_value(&ev))
                                    prop:value=password
                                    class="appearance-none rounded-md ring-1 ring-gray-400
                                            relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900
                                            active::ring-primary-500
                                            focus:ring-primary-500 focus:outline-none focus:z-10
                                            disabled:bg-gray-200 disabled:opacity-70
                                            sm:text-sm"
                                    name="password"
                                />
                            </div>
                            <div class="flex items-center justify-end">
                                <div class="text-sm">
                                    <a
                                        href="/admin/forgot-password"
                                        class="font-medium text-primary-600 hover:text-primary-500"
                                    >
                                        {t!("forgot_your_password")}
                                    </a>
                                </div>
                            </div>

                            <div>
                                <button
                                    type="submit"
                                    disabled=login_action.pending().get()
                                    class="w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white focus:outline-none focus:ring-2 focus:ring-offset-2 bg-primary-600 hover:bg-primary-500 focus:ring-primary-500"
                                >
                                    {move || if login_action.pending().get() {
                                        t!("signing_in")
                                    } else {
                                        t!("sign_in")
                                    }}
                                </button>
                            </div>

                            <Suspense>
                                {move || login_result.get().map(|res| match res {
                                    Ok(token) => view! {
                                        <div class="text-green-600 text-sm mt-2">
                                            {format!("Login successful! Token: {}", token)}
                                        </div>
                                    }.into_any(),
                                    Err(e) => view! {
                                        <div class="text-red-600 text-sm mt-2">
                                            {format!("Error: {}", e)}
                                        </div>
                                    }.into_any(),
                                })}
                            </Suspense>
                        </form>
                    </div>
                </div>
            </div>
    }
}
