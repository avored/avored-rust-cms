use leptos::component;

// use crate::components::svg::dashboard_icon::DashboardIcon;
// use crate::components::svg::menu_icon::MenuIcon;
// use crate::components::svg::shopping_cart::ShoppingCartIcon;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn AppLayout() -> impl IntoView {
    
    let on_logout_click = move |ev: MouseEvent| {
        ev.prevent_default();
        #[cfg(target_arch = "wasm32")]
        {
            let navigate = leptos_router::hooks::use_navigate();
            use gloo_storage::{LocalStorage, Storage};
            LocalStorage::delete("avored_admin_token");
            navigate("/auth/login", Default::default());
        }
    };

    let auth_context = use_context::<crate::pages::protected_routes::AuthContext>().expect("AuthContext should be provided");
    let full_name = auth_context.full_name;
    let is_super_admin = auth_context.is_super_admin;

    view! {
        <div class="flex h-screen bg-gray-100">
            // Sidebar
            <aside class="w-64 bg-slate-900 text-white shrink-0 hidden md:flex flex-col">
                <div class="p-6 flex items-center gap-3">
                    <img src="/public/images/avored.svg" class="w-8 h-8" alt="AvoRed Logo" />
                    <span class="text-xl font-bold tracking-tight">"AvoRed CMS"</span>
                </div>
                
                <nav class="flex-1 px-4 space-y-1 mt-4">
                    <a href="/admin/dashboard" class="flex items-center gap-3 px-3 py-2 rounded-lg bg-primary-600 text-white">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                        </svg>
                        <span>"Dashboard"</span>
                    </a>
                    <a href="/admin/catalog/categories" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" />
                        </svg>
                        <span>"Categories"</span>
                    </a>
                    <a href="/admin/catalog/products" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                        </svg>
                        <span>"Products"</span>
                    </a>
                    <a href="/admin/orders" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                        </svg>
                        <span>"Orders"</span>
                    </a>
                    <div class="pt-4 pb-2 px-3 text-xs font-semibold text-gray-500 uppercase tracking-wider">
                        "System"
                    </div>
                    <a href="/admin/entity" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 7.5l-9-5.25L3 7.5m18 0l-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9" />
                        </svg>
                        <span>"Entity"</span>
                    </a>    
                    <a href="/admin/users" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13.471 14.172a4 4 0 015.658 0L21 17.5" />
                        </svg>
                        <span>"Users"</span>
                    </a>
                    <a href="/admin/settings" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924-1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                        <span>"Settings"</span>
                    </a>
                </nav>
            </aside>

            // Main Content
            <div class="flex-1 flex flex-col overflow-hidden">
                // Header
                <header class="bg-white border-b border-gray-200 h-16 flex items-center justify-between px-6 shrink-0">
                    <div class="md:hidden">
                        <button class="text-gray-500 focus:outline-none">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </button>
                    </div>
                    
                    <div class="flex-1 px-4 hidden md:block">
                        <div class="relative max-w-md">
                            <span class="absolute inset-y-0 left-0 pl-3 flex items-center">
                                <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                                </svg>
                            </span>
                            <input
                                class="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-gray-50 placeholder-gray-500 focus:outline-none focus:ring-1 focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
                                placeholder="Search..."
                                type="search"
                            />
                        </div>
                    </div>

                    <div class="flex items-center gap-4">
                        <button class="p-1 text-gray-400 hover:text-gray-500 focus:outline-none relative">
                            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                            </svg>
                            <span class="absolute top-0 right-0 block h-2 w-2 rounded-full bg-red-500 ring-2 ring-white"></span>
                        </button>
                        
                        <div class="flex items-center gap-3 border-l pl-4 border-gray-200">
                            <div class="text-right hidden sm:block">
                                <div class="text-sm font-medium text-gray-900">{full_name}</div>
                                <div class="text-xs text-gray-500">
                                    {move || if is_super_admin.get() { "Super Admin" } else { "Admin" }}
                                </div>
                            </div>
                            <img
                                class="h-8 w-8 rounded-full border border-gray-300 cursor-pointer"
                                src=move || format!("https://ui-avatars.com/api/?name={}&background=0D8ABC&color=fff", full_name.get())
                                alt="User Profile"
                                on:click=on_logout_click
                            />
                        </div>
                    </div>
                </header>

                // Main Content Area
                <main class="flex-1 overflow-y-auto bg-gray-50 p-6">
                    <Outlet />
                </main>
            </div>
        </div>
    }
}
