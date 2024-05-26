import { useEffect } from "react";
import logo from "../../assets/logo_only.svg";
import { useNavigate } from "react-router-dom";
import { isEmpty } from "lodash";
import InputField from "../../components/InputField";
import { useForm } from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import { forgotPasswordSchema } from "./schemas/forgotPassword.schema";
import _ from "lodash";
import { ErrorMessage } from "../../components/ErrorMessage";
import { useForgotPassword } from "./hooks/useForgotPassword";
import { useTranslation } from "react-i18next";

function ForgotPassword() {
  const redirect = useNavigate();
  const [t] = useTranslation("global");
  const { register, handleSubmit } = useForm({
    // Removed "formState: { errors }" from the object
    resolver: joiResolver(forgotPasswordSchema),
  });
  const { mutate, isPending, error } = useForgotPassword();

  const isErrorExist = (key) => {
    return _.findIndex(
      _.get(error, "response.data.errors", []),
      (err) => err.key === key
    );
  };

  const getErrorMessage = (key) => {
    return _.get(
      error,
      "response.data.errors." + isErrorExist("email") + ".message"
    );
  };

  // TODO: enhance to make this as an HOC
  // for "protecting" routes/pages
  useEffect(() => {
    /* to do make sure it execute once only..*/
    const token = localStorage.getItem("AUTH_TOKEN");
    if (!isEmpty(token)) {
      return redirect("/admin");
    }
  }, [redirect]); // Added "redirect" to the dependency array

  const submitHandler = (data) => {
    mutate(data);
  };

  return (
    <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
      <div className="flex justify-center">
        <img src={logo} className="w-20 h-20" alt="Avored Rust Cms" />
      </div>

      <div className="sm:mx-auto sm:w-full sm:max-w-md">
        <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
          {t("forgot_password")}
        </h2>
      </div>
      <div></div>

      <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
        <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
          <form onSubmit={handleSubmit(submitHandler)} className="space-y-5">
            <div>
              <InputField
                label={t("email_address")}
                type="text"
                name="email"
                autoFocus={true}
                register={register("email")}
              />
              {isErrorExist("email") >= 0 && (
                <ErrorMessage message={getErrorMessage("email")} />
              )}
            </div>

            <div>
              <button
                type="submit"
                className="group relative bg-primary-600 w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              >
                {isPending ? "Loading..." : t("forgot_password")}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}

export default ForgotPassword;
