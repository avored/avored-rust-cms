import InputField from "../../components/InputField";
import {useState} from "react";
import {useStoreSetup} from "./hooks/useStoreSetup";
import _ from "lodash";
import {ErrorMessage} from "../../components/ErrorMessage";

function Setup() {
    const [email, setEmail] = useState("admin@admin.com")
    const [password, setPassword] = useState("admin123")
    const {mutate, isPending, error} = useStoreSetup()

    const isErrorExist = (key) => {
        return _.findIndex(_.get(error, 'response.data.errors', []), ((err) => err.key === key))
    }

    const getErrorMessage = (key) => {
        return _.get(error, "response.data.errors." + isErrorExist('email') + ".message"   )
      }
    const handleSubmit = async (e) => {
        e.preventDefault()
        mutate({email: email, password: password})
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
                                    type="text"
                                    name="email"
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                    autoFocus
                                />
                                {(isErrorExist('email') >=0) && <ErrorMessage message={getErrorMessage('email')} />}
                            </div>

                            <div className="mt-3">
                                <InputField
                                    label="Password"
                                    type="password"
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                />
                                {(isErrorExist('email') >=0) && <ErrorMessage message={getErrorMessage('email')} />}
                            </div>


                            <div className="mt-5">
                                <button type="submit"
                                        disabled={isPending}
                                        className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
                                    {isPending ? "Loading..." : "Submit"}
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