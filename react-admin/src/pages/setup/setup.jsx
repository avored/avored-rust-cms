import InputField from "../../components/InputField";
import {useState} from "react";
import {useNavigate} from "react-router-dom";
import axios from "axios";

function Setup() {
    const [email, setEmail] = useState("admin@admin.com");
    const [password, setPassword] = useState("admin123");
    const redirect = useNavigate();


    const handleSubmit = async (e) => {
        e.preventDefault();
        const response = await axios({
            url: "http://localhost:8080/api/setup",
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            data: JSON.stringify({email: email, password: password}),
        });

        if (response.data.status) {
            return redirect("/admin/login");
        }
    };

    return (
        <div
            className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
            <div className="sm:mx-auto sm:w-full sm:max-w-md">
                <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
                    Setup AvoRed
                </h2>
            </div>
            <div>

            </div>

            <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
                <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
                    <form className="space-y-5" onSubmit={handleSubmit} action="/api/setup" method="POST">
                        <div>
                            <div className="mt-3">
                                <InputField
                                    label="Email Address"
                                    type="email"
                                    name="email"
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                    autoFocus
                                />
                            </div>

                            <div className="mt-3">
                                <InputField
                                    label="Password"
                                    type="password"
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                />
                            </div>


                            <div className="mt-5">
                                <button type="submit"
                                        className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
                                    Submit
                                </button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    )
}

export default Setup