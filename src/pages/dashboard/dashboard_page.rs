use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
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
    }
}
