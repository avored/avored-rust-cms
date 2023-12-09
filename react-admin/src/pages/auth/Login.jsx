import {useEffect, useState} from "react";
import logo from "../../assets/logo_only.svg";
import {useNavigate} from "react-router-dom";
import {isEmpty} from "lodash";

function Login() {
    const [email, setEmail] = useState('admin@admin.com');
    const [password, setPassword] = useState('admin123');
    const redirect = useNavigate()

    const [token, setToken] = useState('');

    useEffect(()=>{
        /* to do make sure it execute once only..*/
        const token = localStorage.getItem("AUTH_TOKEN");
        if (!isEmpty(token)) {
            return redirect("/admin");
        }
    },[]);

    const handleSubmit = (async (e) => {
        e.preventDefault()
        const response = (await fetch('http://localhost:8080/api/login', {
            method: 'post',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({email: email, password: password})
        }))
        const login_response = await response.json()
        if (login_response.status) {
            localStorage.setItem("AUTH_TOKEN", login_response.data)
            return redirect("/admin");
        }
    })


    return (
        <div
            className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
            <div className="flex justify-center">
                <img src={logo} className="w-20 h-20" alt="Avored Rust Cms"/>
            </div>

            <div className="sm:mx-auto sm:w-full sm:max-w-md">
                <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
                    Sign into your account
                </h2>
            </div>
            <div>

            </div>

            <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
                <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
                    <form onSubmit={handleSubmit} className="space-y-5" action="/admin/login" method="POST">
                        <div>
                            <label htmlFor="email_address" className="text-sm text-gray-600">
                                Email Address
                            </label>
                            <div className="mt-1">
                                <input
                                    type="email"
                                    name="email"
                                    required
                                    value={email}
                                    onChange={e => setEmail(e.target.value)}
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
                                    value={password}
                                    onChange={e => setPassword(e.target.value)}
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