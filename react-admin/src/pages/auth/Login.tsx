import logo from "../../assets/logo_only.svg"
import { Link } from "react-router-dom"
import InputField from "../../components/InputField"
import {SubmitHandler, useForm} from "react-hook-form"
import { joiResolver } from "@hookform/resolvers/joi"
import { useLogin } from "./hooks/useLogin"
import {useLoginSchema} from "./schemas/login.schema"
import ErrorMessage from "../../components/ErrorMessage"
import { useTranslation } from "react-i18next"
import ILoginPost from "../../types/auth/ILoginPost"
import {changeLocale} from "../../lib/common"
import AvoRedButton from "../../components/AvoRedButton";

function Login() {
  const [t, i18n] = useTranslation("global")
  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm<ILoginPost>({
    resolver: joiResolver(useLoginSchema()),
  })
  const {
    mutate,
    isPending,
    error
  } = useLogin()

  const submitHandler: SubmitHandler<ILoginPost> = (data) => {
    mutate(data);
  }

  return (
    <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
      <div className="flex justify-center">
        <img src={logo} className="w-20 h-20" alt={t("avored_rust_cms")} />
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
                label={t("email")}
                type="text"
                name="email"
                autoFocus={true}
                register={register("email")}
              />
              <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="email" />
            </div>
            <div>
              <InputField
                label={t("password")}
                type="password"
                name="password"
                register={register("password")}
              />
              <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="password" />
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
              <AvoRedButton
                  label={t("sign_in")}
                  isPending={isPending}
                  className="bg-primary-600 hover:bg-primary-500  focus:ring-primary-500"
              />
            </div>

            <div className="text-gray-600 text-center text-sm">
              {t("need_to_change_language")}
              <select
                onChange={(e) => changeLocale(i18n, e.target.value)}
                className="outline-none border-none appearance-none pr-8"
              >
                <option>{t("en")}</option>
                <option>{t('fr')}</option>
              </select>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}

export default Login;
