function Login() {
    return (
        <div
            className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
            <div className="sm:mx-auto sm:w-full sm:max-w-md">
                <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
                    Sign into your account
                </h2>
            </div>
            <div>

            </div>

            <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
                <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
                    <form className="space-y-5" action="/admin/login" method="POST">
                        <div>
                            <label htmlFor="email_address" className="text-sm text-gray-600">
                                Email Address
                            </label>
                            <div className="mt-1">
                                <input
                                    type="email"
                                    name="email"
                                    required
                                    autoFocus
                                    className="appearance-none rounded-md ring-1 ring-primary-300 relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-primary-500 sm:text-sm focus:z-10"
                                    placeholder="Email Address"
                                />
                            </div>
                        </div>
                        <div>
                            <label htmlFor="password" className="text-sm text-gray-600">
                                Password
                            </label>
                            <div className="mt-1">
                                <input
                                    type="password"
                                    name="password"
                                    required
                                    className="appearance-none rounded-md ring-1 ring-primary-300 relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-primary-500 sm:text-sm focus:z-10"
                                    placeholder="Password"
                                />
                            </div>
                        </div>

                        <div className="flex items-center justify-end">
                            <div className="text-sm">
                                <a href="#"
                                   className="font-medium text-primary-600 hover:text-primary-500">
                                    Forgot your password?
                                </a>
                            </div>
                        </div>

                        <div>
                            <button type="submit"
                                    className="group relative bg-primary-600 w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                            >
                                Sign in
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    )
}

export default Login