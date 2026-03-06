use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="flex h-screen bg-gray-100">
            // Sidebar
            <aside class="w-64 bg-slate-900 text-white shrink-0 hidden md:flex flex-col">
                <div class="p-6 flex items-center gap-3">
                    <img src="/public/images/avored.svg" class="w-8 h-8" alt="AvoRed Logo" />
                    <span class="text-xl font-bold tracking-tight">"AvoRed CMS"</span>
                </div>
                
                <nav class="flex-1 px-4 space-y-1 mt-4">
                    <a href="/dashboard" class="flex items-center gap-3 px-3 py-2 rounded-lg bg-primary-600 text-white">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 text-white rounded-md shadow-lg p-1-1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                        </svg>
                        <span>"Dashboard"</span>
                    </a>
                    <a href="/catalog/categories" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" />
                        </svg>
                        <span>"Categories"</span>
                    </a>
                    <a href="/catalog/products" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                        </svg>
                        <span>"Products"</span>
                    </a>
                    <a href="/orders" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                        </svg>
                        <span>"Orders"</span>
                    </a>
                    <div class="pt-4 pb-2 px-3 text-xs font-semibold text-gray-500 uppercase tracking-wider">
                        "System"
                    </div>
                    <a href="/users" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13.471 14.172a4 4 0 015.658 0L21 17.5" />
                        </svg>
                        <span>"Users"</span>
                    </a>
                    <a href="/settings" class="flex items-center gap-3 px-3 py-2 rounded-lg text-gray-300 hover:bg-slate-800 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
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
                                <div class="text-sm font-medium text-gray-900">"Admin User"</div>
                                <div class="text-xs text-gray-500">"Super Admin"</div>
                            </div>
                            <img
                                class="h-8 w-8 rounded-full border border-gray-300"
                                src="https://ui-avatars.com/api/?name=Admin+User&background=0D8ABC&color=fff"
                                alt="User Profile"
                            />
                        </div>
                    </div>
                </header>

                // Main Content Area
                <main class="flex-1 overflow-y-auto bg-gray-50 p-6">
                    <div class="mb-8">
                        <h1 class="text-2xl font-bold text-gray-900">"Dashboard"</h1>
                        <p class="text-gray-500">"Welcome back, here's what's happening today."</p>
                    </div>

                    // Stats Grid
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
                            <div class="flex items-center justify-between mb-4">
                                <div class="p-2 bg-blue-50 text-blue-600 rounded-lg">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                                    </svg>
                                </div>
                                <span class="text-green-500 text-sm font-medium">"+12.5%"</span>
                            </div>
                            <div class="text-3xl font-bold text-gray-900">"1,280"</div>
                            <div class="text-sm text-gray-500 mt-1">"Total Orders"</div>
                        </div>

                        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
                            <div class="flex items-center justify-between mb-4">
                                <div class="p-2 bg-green-50 text-green-600 rounded-lg">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                </div>
                                <span class="text-green-500 text-sm font-medium">"+8.2%"</span>
                            </div>
                            <div class="text-3xl font-bold text-gray-900">"$42,500"</div>
                            <div class="text-sm text-gray-500 mt-1">"Total Revenue"</div>
                        </div>

                        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
                            <div class="flex items-center justify-between mb-4">
                                <div class="p-2 bg-purple-50 text-purple-600 rounded-lg">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                                    </svg>
                                </div>
                                <span class="text-red-500 text-sm font-medium">"-3.1%"</span>
                            </div>
                            <div class="text-3xl font-bold text-gray-900">"850"</div>
                            <div class="text-sm text-gray-500 mt-1">"New Customers"</div>
                        </div>

                        <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
                            <div class="flex items-center justify-between mb-4">
                                <div class="p-2 bg-orange-50 text-orange-600 rounded-lg">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                    </svg>
                                </div>
                                <span class="text-green-500 text-sm font-medium">"+5.4%"</span>
                            </div>
                            <div class="text-3xl font-bold text-gray-900">"15,200"</div>
                            <div class="text-sm text-gray-500 mt-1">"Page Views"</div>
                        </div>
                    </div>

                    // Recent Orders Table
                    <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                        <div class="px-6 py-4 border-b border-gray-100 flex items-center justify-between">
                            <h2 class="font-bold text-gray-900">"Recent Orders"</h2>
                            <button class="text-primary-600 text-sm font-medium hover:underline">"View all"</button>
                        </div>
                        <div class="overflow-x-auto">
                            <table class="w-full text-left">
                                <thead class="bg-gray-50">
                                    <tr>
                                        <th class="px-6 py-3 text-xs font-semibold text-gray-500 uppercase">"Order ID"</th>
                                        <th class="px-6 py-3 text-xs font-semibold text-gray-500 uppercase">"Customer"</th>
                                        <th class="px-6 py-3 text-xs font-semibold text-gray-500 uppercase">"Date"</th>
                                        <th class="px-6 py-3 text-xs font-semibold text-gray-500 uppercase">"Amount"</th>
                                        <th class="px-6 py-3 text-xs font-semibold text-gray-500 uppercase">"Status"</th>
                                    </tr>
                                </thead>
                                <tbody class="divide-y divide-gray-100">
                                    <tr class="hover:bg-gray-50 transition-colors">
                                        <td class="px-6 py-4 text-sm font-medium text-gray-900">"#ORD-001"</td>
                                        <td class="px-6 py-4 text-sm text-gray-500">"John Doe"</td>
                                        <td class="px-6 py-4 text-sm text-gray-500">"Oct 24, 2023"</td>
                                        <td class="px-6 py-4 text-sm font-medium text-gray-900">"$250.00"</td>
                                        <td class="px-6 py-4">
                                            <span class="px-2 py-1 text-xs font-medium bg-green-100 text-green-700 rounded-full">"Completed"</span>
                                        </td>
                                    </tr>
                                    <tr class="hover:bg-gray-50 transition-colors">
                                        <td class="px-6 py-4 text-sm font-medium text-gray-900">"#ORD-002"</td>
                                        <td class="px-6 py-4 text-sm text-gray-500">"Jane Smith"</td>
                                        <td class="px-6 py-4 text-sm text-gray-500">"Oct 23, 2023"</td>
                                        <td class="px-6 py-4 text-sm font-medium text-gray-900">"$120.50"</td>
                                        <td class="px-6 py-4">
                                            <span class="px-2 py-1 text-xs font-medium bg-blue-100 text-blue-700 rounded-full">"Processing"</span>
                                        </td>
                                    </tr>
                                    <tr class="hover:bg-gray-50 transition-colors">
                                        <td class="px-6 py-4 text-sm font-medium text-gray-900">"#ORD-003"</td>
                                        <td class="px-6 py-4 text-sm text-gray-500">"Michael Brown"</td>
                                        <td class="px-6 py-4 text-sm text-gray-500">"Oct 22, 2023"</td>
                                        <td class="px-6 py-4 text-sm font-medium text-gray-900">"$450.00"</td>
                                        <td class="px-6 py-4">
                                            <span class="px-2 py-1 text-xs font-medium bg-yellow-100 text-yellow-700 rounded-full">"Pending"</span>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}
