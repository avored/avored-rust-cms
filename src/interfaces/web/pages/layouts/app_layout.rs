use leptos::component;

use crate::interfaces::web::protected_routes::AuthContext;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use rust_i18n::t;

#[component]
pub fn AppLayout() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("AuthContext should be provided");
    let is_menu_open = RwSignal::new(false);
    let navigate = leptos_router::hooks::use_navigate();

    let is_logged_in = auth_context.is_logged_in;
    let auth_ready = auth_context.auth_ready;

    Effect::new(move |_| {
        let ready = auth_ready.get();
        let logged_in = is_logged_in.get();
        if ready && !logged_in {
            navigate("/auth/login", Default::default());
        }
    });

    let user_name = move || {
        auth_context
            .logged_in_user
            .get()
            .map(|user| user.name)
            .unwrap_or_default()
    };
    let user_email = move || {
        auth_context
            .logged_in_user
            .get()
            .map(|user| user.email)
            .unwrap_or_default()
    };

    let class = "w-5 h-5";

    let on_logout_click = move |ev: MouseEvent| {
        ev.prevent_default();
        auth_context.is_logged_in.set(false);
        auth_context.auth_token.set(String::new());
        auth_context.logged_in_user.set(None);

        #[cfg(target_arch = "wasm32")]
        {
            let navigate = leptos_router::hooks::use_navigate();
            use gloo_storage::{LocalStorage, Storage};
            LocalStorage::delete("auth_token");
            navigate("/auth/login", Default::default());
        }
    };

    view! {
        <div class="flex h-screen bg-gray-100">
            <aside class="hidden w-64 flex-col bg-white shadow-md md:flex">
                <div class="h-16 border-b border-gray-300 text-center">
                    <h1 class="mt-3 text-2xl ml-4 px-3 font-bold flex text-primary-800">
                        <img src="/assets/images/avored.svg" class="h-8 w-8" /> 
                        <span class="text-primary-500 font-semibold ml-1">"Avored"</span>
                    </h1>
                </div>

                <nav class="flex-1 space-y-2 p-4">
                    <a href="/admin/dashboard" class="group flex items-center rounded-md px-4 py-2 text-gray-700 transition hover:bg-gray-100" active-class="bg-blue-50 text-blue-600 font-medium">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
                            <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
                            <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
                            <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
                            <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
                            <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
                        </svg>
                        <span class="ml-2">{t!("dashboard")}</span>
                    </a>

                    <a href="/admin/category" class="group flex items-center rounded-md px-4 py-2 text-gray-700 transition hover:bg-gray-100" active-class="bg-blue-50 text-blue-600 font-medium">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
                            <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
                            <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
                            <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
                            <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
                            <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
                        </svg>
                        <span class="ml-2">{t!("category")}</span>
                    </a>

                    <a href="/admin/product" class="group flex items-center rounded-md px-4 py-2 text-gray-700 transition hover:bg-gray-100" active-class="bg-blue-50 text-blue-600 font-medium">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
                            <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
                            <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
                            <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
                            <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
                            <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
                        </svg>
                        <span class="ml-2">{t!("products")}</span>
                    </a>

                    <a href="/admin/customer" class="group flex items-center rounded-md px-4 py-2 text-gray-700 transition hover:bg-gray-100" active-class="bg-blue-50 text-blue-600 font-medium">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
                            <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
                            <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
                            <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
                            <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
                            <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
                        </svg>
                        <span class="ml-2">{t!("customers")}</span>
                    </a>
                </nav>
            </aside>

            <div class="flex flex-1 flex-col overflow-hidden">
                <header class="flex h-16 items-center justify-between border-b border-gray-300 bg-white py-4 shadow-sm">
                    <h2 class="text-xl font-semibold text-gray-800">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
                            <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
                            <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
                            <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
                            <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
                            <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
                        </svg>
                    </h2>

                    <div class="relative ml-auto mr-5">
                        <div
                            on:click=move |_| is_menu_open.update(|value| *value = !*value)
                            class="flex cursor-pointer items-center justify-center"
                        >
                            <img class="h-10 w-10 rounded-md object-cover" src="https://placehold.net/avatar.png" alt="Administrator" />
                            <span class="ml-2 text-sm text-gray-800">{move || user_name()}</span>
                        </div>

                        {move || {
                            if is_menu_open.get() {
                                view! {
                                    <div class="absolute right-0 top-14 z-50 w-48 rounded-md shadow-lg">
                                        <div class="overflow-hidden rounded-md bg-white ring-1 ring-black/20 dark:bg-gray-700 dark:ring-black/40">
                                            <div class="flex flex-col gap-1 border-b px-4 py-2 font-medium dark:border-b-gray-600">
                                                <span class="text-gray-800 dark:text-gray-300">{user_name()}</span>
                                                <span class="truncate text-xs text-gray-400 dark:text-gray-500">{user_email()}</span>
                                            </div>

                                            <a href="/admin/profile" class="block px-4 py-2 text-sm font-medium text-gray-800 transition duration-150 ease-in-out hover:bg-black hover:text-white focus:bg-black focus:text-white focus:outline-none dark:text-gray-200">
                                                {t!("profile")}
                                            </a>

                                            <a href="#" on:click=on_logout_click class="block px-4 py-2 text-sm font-medium text-gray-800 transition duration-150 ease-in-out hover:bg-black hover:text-white focus:bg-black focus:text-white focus:outline-none dark:text-gray-200">
                                                {t!("logout")}
                                            </a>
                                        </div>
                                    </div>
                                }
                                .into_any()
                            } else {
                                ().into_view().into_any()
                            }
                        }}
                    </div>
                </header>

                <main class="flex-1 overflow-x-hidden overflow-y-auto bg-gray-50 p-6">
                    <Outlet />
                </main>
            </div>
        </div>
    }
}
