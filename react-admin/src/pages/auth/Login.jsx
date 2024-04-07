import { useEffect, useState } from "react";
import logo from "../../assets/logo_only.svg";
import { useNavigate } from "react-router-dom";
import { isEmpty } from "lodash";
import InputField from "../../components/InputField";
import apiClient from "../../ApiClient";
import _ from 'lodash';

function Login() {
  const [email, setEmail] = useState("admin@admincom");
  const [password, setPassword] = useState("admin123");
  const redirect = useNavigate();

  const [token, setToken] = useState("");

  useEffect(() => {
    /* to do make sure it execute once only..*/
    const token = localStorage.getItem("AUTH_TOKEN");
    if (!isEmpty(token)) {
      return redirect("/admin");
    }
  }, []);

  const handleSubmit = async (e) => {
    e.preventDefault();
    var data = JSON.stringify({ email: email, password: password });
    const response =
        await apiClient.post("/login", data)
            .catch((errors) => {
              console.log(errors)
            });

    console.log(_.get(response, 'data.status'))
    // if (_.get(response, 'data.status')) {
    //   localStorage.setItem("AUTH_TOKEN", response.data);
    //   return redirect("/admin");
    // }
  };

  return (
    <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
      <div className="flex justify-center">
        <img src={logo} className="w-20 h-20" alt="Avored Rust Cms" />
      </div>

      <div className="sm:mx-auto sm:w-full sm:max-w-md">
        <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
          Sign into your account
        </h2>
      </div>
      <div></div>

      <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
        <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
          <form
            onSubmit={handleSubmit}
            className="space-y-5"
            action="/admin/login"
            method="POST"
          >
            <InputField
              label="Email Address"
              type="email"
              name="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              autoFocus
            />
            <InputField
              label="Password"
              type="password"
              name="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              autoFocus="false"
            />
            <div className="flex items-center justify-end">
              <div className="text-sm">
                <a
                  href="#"
                  className="font-medium text-primary-600 hover:text-primary-500"
                >
                  Forgot your password?
                </a>
              </div>
            </div>

            <div>
              <button
                type="submit"
                className="group relative bg-primary-600 w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              >
                Sign in
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}

export default Login;
