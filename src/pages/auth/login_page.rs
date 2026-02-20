use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
                <div class="flex justify-center">
                    <img src="/public/images/avored.svg" class="w-20 h-20" alt="avored_rust_cms" />
                </div>

                <div class="sm:mx-auto sm:w-full sm:max-w-md">
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        {"sign_into_your_account"}
                    </h2>
                </div>
                <div></div>


                <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
                    <div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
                        <form class="space-y-5">
                            <div>
                                <input
                                    label="email"
                                    type="text"
                                    name="email"
                                    autoFocus
                                    class="appearance-none rounded-md ring-1 ring-gray-400
                                            relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900
                                            active::ring-primary-500
                                            focus:ring-primary-500 focus:outline-none focus:z-10
                                            disabled:bg-gray-200 disabled:opacity-70
                                            sm:text-sm"
                                />
                            </div>
                            <div>
                                <input
                                    label="password"
                                    type="password"
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
                                        "forgot_your_password"
                                    </a>
                                </div>
                            </div>

                            <div>
                                <button
                                    class="w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white focus:outline-none focus:ring-2 focus:ring-offset-2 bg-primary-600 hover:bg-primary-500 focus:ring-primary-500"
                                >
                                    "sign_in"
                                </button>
                            </div>

                            // <div class="text-gray-600 text-center text-sm">
                            //     "need_to_change_language"
                            //     <select
                            //         class="outline-none border-none appearance-none pr-8"
                            //     >
                            //         <option>"en"</option>
                            //         <option>{t('fr')}</option>
                            //     </select>
                            // </div>
                        </form>
                    </div>
                </div>
            </div>
    }
}
