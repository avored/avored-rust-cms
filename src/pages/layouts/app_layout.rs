use leptos::component;

use crate::pages::protected_routes::AuthContext;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use rust_i18n::t;

#[component]
pub fn AppLayout() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("AuthContext should be provided");
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
        <aside class="w-64 bg-white shadow-md hidden md:flex flex-col">
            <div class="h-16 text-center border-b border-gray-300">
                <h1 class="mt-3 text-2xl font-bold text-primary-800">
                    {t!("brand")}
                </h1>
            </div>
            <nav class="flex-1 p-4 space-y-2">
                <a href="/admin/dashboard"
                    class="group flex items-center px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-md transition"
                    active-class="bg-blue-50 text-blue-600 font-medium">
                    <svg xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24" fill="none" stroke="currentColor"
                        stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" class={class}>
                        <circle cx="12" cy="12" r="10"></circle>
                        <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
                        <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
                        <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
                        <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
                        <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
                        <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
                    </svg>
                    {t!("dashboard")}
                </a>
                <a href="/admin/category"
                    class="group flex items-center px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-md transition"
                    active-class="bg-blue-50 text-blue-600 font-medium">
                    <svg xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2" stroke-linecap="round"
            stroke-linejoin="round" class={class}>
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
            <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
            <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
            <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
            <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
            <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
        </svg>
                    {t!("category")}
                </a>
                <a href="/admin/product"
                    class="group flex items-center px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-md transition"
                    active-class="bg-blue-50 text-blue-600 font-medium">
                    <svg xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2" stroke-linecap="round"
            stroke-linejoin="round" class={class}>
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
            <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
            <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
            <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
            <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
            <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
        </svg>
                    {t!("products")}
                </a>
                <a href="/admin/customer"
                    class="group flex items-center px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-md transition"
                    active-class="bg-blue-50 text-blue-600 font-medium">
                    <svg xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2" stroke-linecap="round"
            stroke-linejoin="round" class={class}>
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
            <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
            <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
            <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
            <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
            <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
        </svg>
                    {t!("customers")}
                </a>

            </nav>
        </aside>


        <div class="flex-1 flex flex-col overflow-hidden">

            <header  class="h-16 bg-white shadow-sm border-b border-gray-300 py-4 flex justify-between items-center">
                <h2 class="text-xl font-semibold text-gray-800">
                   <svg xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2" stroke-linecap="round"
            stroke-linejoin="round" class={class}>
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="14.31" y1="8" x2="20.05" y2="17.94"></line>
            <line x1="9.69" y1="8" x2="21.17" y2="8"></line>
            <line x1="7.38" y1="12" x2="13.12" y2="2.06"></line>
            <line x1="9.69" y1="16" x2="3.95" y2="6.06"></line>
            <line x1="14.31" y1="16" x2="2.83" y2="16"></line>
            <line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
        </svg>
                </h2>

                <div class="cursor-pointer flex items-center justify-center ml-auto mr-5"
                    x-data="{ isOpen: false }"
                >

                    <div
                    x-on:click.away="isOpen = false" x-on:click="isOpen=!isOpen" class="flex items-center justify-center">
                        <img class="h-10 w-10 rounded-md object-cover" src="https://placehold.net/avatar.png" alt="Administrator" />
                        <span class="ml-2 text-gray-800 text-sm">
                            {move || user_name()}
                        </span>


                    <div
                        x-cloak
                        x-show="isOpen"
                        class="absolute top-0 right-0 z-100 mr-5  mt-16 w-48  rounded-md shadow-lg">
                        <div
                            class="overflow-hidden rounded-md bg-white ring-1 ring-black/20 dark:bg-gray-700 dark:ring-black/40"
                        >
                            <div class="flex flex-col gap-1 border-b px-4 py-2 font-medium dark:border-b-gray-600">
                                <span class="text-gray-800 dark:text-gray-300">
                                    {move || user_name()}
                                </span>
                                <span class="truncate text-xs text-gray-400 dark:text-gray-500">
                                    {move || user_email()}
                                </span>
                            </div>
                            <a
                                href="/admin/profile"
                                class="block px-4 py-2 text-sm font-medium text-gray-800 transition duration-150 ease-in-out
                                    hover:bg-black hover:text-white focus:bg-black 
                                    focus:text-white focus:outline-none dark:text-gray-200"
                            >
                                {t!("admin.layout.header.profile_link")}
                            </a>

                            <a
                                href="#"
                                on:click=on_logout_click
                                class="block px-4 py-2 text-sm font-medium text-gray-800 transition duration-150 ease-in-out
                                    hover:bg-black hover:text-white 
                                    focus:bg-black focus:text-white focus:outline-none 
                                    dark:text-gray-200"
                            >
                                {t!("admin.layout.header.logout_link")}
                            </a>
                        </div>
                    </div>
                    </div>
                </div>


                // <button
                //     type="button"
                //     on:click=on_logout_click
                //     class="mr-3 pr-6 cursor-pointer text-sm text-primary-800 hover:text-primary-900 font-medium">
                //     "Logout"
                // </button>
            </header>


            <main class="flex-1 overflow-x-hidden overflow-y-auto bg-gray-50 p-6">
                <Outlet />
            </main>
        </div>

        <div
            x-data="{ showToast: false, message: '', type: 'error' }"
            x-on:notify.window="message = $event.detail.message; type = $event.detail.type || 'error'; showToast = true; setTimeout(() => showToast = false, 5000);"
            class="fixed bottom-4 right-4 z-50"
            x-cloak
        >
            <div
                x-show="showToast"
                x-transition:enter="transition ease-out duration-300"
                x-transition:enter-start="opacity-0 translate-y-2"
                x-transition:enter-end="opacity-100 translate-y-0"
                x-transition:leave="transition ease-in duration-200"
                x-transition:leave-start="opacity-100 translate-y-0"
                x-transition:leave-end="opacity-0 translate-y-2"
                x-bind:class="type === 'error' ? 'bg-red-400' : 'bg-green-500'"
                class="text-white px-5 py-4 rounded-lg shadow-xl flex items-center justify-between min-w-[300px]"
            >
                <span x-text="message" class="font-medium text-xs"></span>
                <button x-on:click="showToast = false" type="button" class="ml-4 text-white hover:text-gray-100 focus:outline-none transition-colors">
                    <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>
        </div>
    </div>
    }
}
