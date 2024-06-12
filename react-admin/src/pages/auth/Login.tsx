
import logo from "../../assets/logo_only.svg";
import {  Link } from "react-router-dom";
import InputField from "../../components/InputField";
import {SubmitHandler, useForm} from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import { useLogin } from "./hooks/useLogin";
import { loginSchema } from "./schemas/login.schema";
import _ from "lodash";
import { ErrorMessage } from "../../components/ErrorMessage";
import { useTranslation } from "react-i18next";
import IErrorMessage from "../../types/common/IError";
import ILoginPost from "../../types/auth/ILoginPost";

function Login() {
  const [t, i18] = useTranslation("global");
  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm<ILoginPost>({
    resolver: joiResolver(loginSchema),
  })
  const {
    mutate,
    isPending,
    error
  } = useLogin()

  const localeChange = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    await i18.changeLanguage(e.target.value)
  }

  const isErrorExist = (key: string) => {
        let message = _.get(errors, key + '.message');
        if (message) {
          return 1;
        }
    return _.findIndex(_.get(error, 'response.data.errors', []), ((err: IErrorMessage) => err.key === key))
  }

  const getErrorMessage = (key: string) => {
    let message = _.get(errors, key + '.message');
    if (message) {
      return message;
    }
    return _.get(error, "response.data.errors." + isErrorExist('email') + ".message"   )
  }

  const submitHandler: SubmitHandler<ILoginPost> = (data) => {
    mutate(data);
  };

  return (
    <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
      <div className="flex justify-center">
        <img src={logo} className="w-20 h-20" alt="Avored Rust Cms" />
      </div>

      <div className="sm:mx-auto sm:w-full sm:max-w-md">
        <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
          {t("sign_into_your_account")}
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
              <InputField
                label={t("password")}
                type="password"
                name="password"
                register={register("password")}
              />
              {isErrorExist("password") >= 0 && (
                <ErrorMessage message={getErrorMessage("password")} />
              )}
            </div>
            <div className="flex items-center justify-end">
              <div className="text-sm">
                <Link
                  to={`/admin/forgot-password`}
                  className="font-medium text-primary-600 hover:text-primary-500"
                >
                  {t("forgot_your_password")}
                </Link>
              </div>
            </div>

            <div>
              <button
                type="submit"
                className="group relative bg-primary-600 w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              >
                {isPending ? "Loading..." : t("sign_in")}
              </button>
            </div>

            <div className="text-gray-600 text-center text-sm">
              {t("need_to_change_language")}
              <select
                onChange={(e) => localeChange(e)}
                className="outline-none border-none appearance-none pr-8"
              >
                <option>en</option>
                <option>fr</option>
              </select>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}

export default Login;
