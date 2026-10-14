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

    let class = "w-5 h-5 shrink-0";

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
        <div x-data="appLayout()" class="flex h-screen bg-gray-100">
            <aside
                class="hidden flex-col bg-white shadow-md transition-all duration-300 md:flex w-16 lg:w-64"
                x-bind:class="{ 'w-16 md:w-16 lg:w-16': isSidebarCollapsed, 'w-16 md:w-16 lg:w-64': !isSidebarCollapsed }"
            >
                <div class="h-16 border-b border-gray-300 flex items-center justify-center lg:justify-start overflow-hidden">
                    <h1
                        class="text-2xl font-bold flex items-center text-primary-800 transition-all px-2 md:justify-center lg:px-4 lg:ml-2 lg:justify-start"
                        x-bind:class="{ 'justify-center lg:justify-center lg:ml-0': isSidebarCollapsed, 'justify-center lg:justify-start': !isSidebarCollapsed }"
                    >
                        <img src="/assets/images/avored.svg" class="h-8 w-8 shrink-0" alt="Avored" />
                        <span
                            class="hidden lg:inline text-primary-500 font-semibold ml-1 whitespace-nowrap"
                            x-bind:class="{ '!hidden': isSidebarCollapsed }"
                        >
                            "Avored"
                        </span>
                    </h1>
                </div>

                <nav
                    class="flex-1 space-y-2 py-4 px-2 lg:px-4"
                    x-bind:class="{ 'px-2 lg:px-2': isSidebarCollapsed, 'px-2 lg:px-4': !isSidebarCollapsed }"
                >
                    <a
                        href="/admin"
                        class="group flex items-center rounded-md py-2 text-gray-700 transition hover:bg-gray-100 justify-center lg:justify-start px-2 lg:px-4"
                        x-bind:class="{ 'justify-center lg:justify-center lg:px-2': isSidebarCollapsed, 'justify-center lg:justify-start lg:px-4': !isSidebarCollapsed }"
                        active-class="bg-blue-50 text-blue-600 font-medium"
                        title={t!("dashboard")}
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                            <polyline points="9 22 9 12 15 12 15 22"></polyline>
                        </svg>
                        <span
                            class="hidden lg:inline ml-2 whitespace-nowrap"
                            x-bind:class="{ '!hidden': isSidebarCollapsed }"
                        >
                            {t!("dashboard")}
                        </span>
                    </a>

                    <a
                        href="/admin/entity"
                        class="group flex items-center rounded-md py-2 text-gray-700 transition hover:bg-gray-100 justify-center lg:justify-start px-2 lg:px-4"
                        x-bindx-bind:class="{ 'justify-center lg:justify-center lg:px-2': isSidebarCollapsed, 'justify-center lg:justify-start lg:px-4': !isSidebarCollapsed }"
                        active-class="bg-blue-50 text-blue-600 font-medium"
                        title={t!("entities")}
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
                            <polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline>
                            <line x1="12" y1="22.08" x2="12" y2="12"></line>
                        </svg>
                        <span
                            class="hidden lg:inline ml-2 whitespace-nowrap"
                            x-bindx-bind:class="{ '!hidden': isSidebarCollapsed }"
                        >
                            {t!("entities")}
                        </span>
                    </a>

                    <a
                        href="/admin/collections"
                        class="group flex items-center rounded-md py-2 text-gray-700 transition hover:bg-gray-100 justify-center lg:justify-start px-2 lg:px-4"
                        x-bindx-bind:class="{ 'justify-center lg:justify-center lg:px-2': isSidebarCollapsed, 'justify-center lg:justify-start lg:px-4': !isSidebarCollapsed }"
                        active-class="bg-blue-50 text-blue-600 font-medium"
                        title={t!("collections")}
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <polygon points="12 2 2 7 12 12 22 7 12 2"></polygon>
                            <polyline points="2 17 12 22 22 17"></polyline>
                            <polyline points="2 12 12 17 22 12"></polyline>
                        </svg>
                        <span
                            class="hidden lg:inline ml-2 whitespace-nowrap"
                            x-bindx-bind:class="{ '!hidden': isSidebarCollapsed }"
                        >
                            {t!("collections")}
                        </span>
                    </a>

                    <a
                        href="/admin/email-templates"
                        class="group flex items-center rounded-md py-2 text-gray-700 transition hover:bg-gray-100 justify-center lg:justify-start px-2 lg:px-4"
                        x-bind:class="{ 'justify-center lg:justify-center lg:px-2': isSidebarCollapsed, 'justify-center lg:justify-start lg:px-4': !isSidebarCollapsed }"
                        active-class="bg-blue-50 text-blue-600 font-medium"
                        title={t!("email_templates")}
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <path d="M4 4h16v16H4z"></path>
                            <path d="M4 7l8 6 8-6"></path>
                        </svg>
                        <span
                            class="hidden lg:inline ml-2 whitespace-nowrap"
                            x-bind:class="{ '!hidden': isSidebarCollapsed }"
                        >
                            {t!("email_templates")}
                        </span>
                    </a>

                    <a
                        href="/admin/workflows"
                        class="group flex items-center rounded-md py-2 text-gray-700 transition hover:bg-gray-100 justify-center lg:justify-start px-2 lg:px-4"
                        x-bind:class="{ 'justify-center lg:justify-center lg:px-2': isSidebarCollapsed, 'justify-center lg:justify-start lg:px-4': !isSidebarCollapsed }"
                        active-class="bg-blue-50 text-blue-600 font-medium"
                        title="Workflows"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <rect x="3" y="3" width="6" height="6" rx="1"></rect>
                            <rect x="15" y="15" width="6" height="6" rx="1"></rect>
                            <path d="M9 6h3a3 3 0 0 1 3 3v6"></path>
                            <path d="M12 15H9a3 3 0 0 0-3 3v3"></path>
                        </svg>
                        <span
                            class="hidden lg:inline ml-2 whitespace-nowrap"
                            x-bind:class="{ '!hidden': isSidebarCollapsed }"
                        >
                            "Workflows"
                        </span>
                    </a>

                    <a
                        href="/admin/api-keys"
                        class="group flex items-center rounded-md py-2 text-gray-700 transition hover:bg-gray-100 justify-center lg:justify-start px-2 lg:px-4"
                        x-bind:class="{ 'justify-center lg:justify-center lg:px-2': isSidebarCollapsed, 'justify-center lg:justify-start lg:px-4': !isSidebarCollapsed }"
                        active-class="bg-blue-50 text-blue-600 font-medium"
                        title="API Keys"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={class}>
                            <path d="M21 2l-2 2m-2-2l2 2m0 0l-7.5 7.5a5 5 0 1 0 2 2L21 6.5V4h-2.5L17 5.5"></path>
                            <circle cx="8" cy="16" r="3"></circle>
                        </svg>
                        <span
                            class="hidden lg:inline ml-2 whitespace-nowrap"
                            x-bind:class="{ '!hidden': isSidebarCollapsed }"
                        >
                            "API Keys"
                        </span>
                    </a>
                </nav>
            </aside>

            <div class="flex flex-1 flex-col overflow-hidden">
                <header class="flex h-16 items-center justify-between border-b border-gray-300 bg-white py-4 px-4 shadow-sm">
                    <button
                        type="button"
                        x-on:click="toggleSidebar()"
                        class="p-1 rounded-md text-gray-600 hover:text-gray-900 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-primary-500"
                        title="Toggle Sidebar"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6">
                            <line x1="3" y1="12" x2="21" y2="12"></line>
                            <line x1="3" y1="6" x2="21" y2="6"></line>
                            <line x1="3" y1="18" x2="21" y2="18"></line>
                        </svg>
                    </button>

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
