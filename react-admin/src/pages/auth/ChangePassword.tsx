import {useEffect} from "react"
import logo from "../../assets/logo_only.svg"
import {useNavigate, useParams} from "react-router-dom"
import { isEmpty } from "lodash"
import InputField from "../../components/InputField"
import {useForm} from 'react-hook-form'
import {joiResolver} from '@hookform/resolvers/joi'
import _ from 'lodash'
import { ErrorMessage } from "../../components/ErrorMessage"
import { useChangePassword } from "./hooks/useChangePassword"
import { changePasswordSchema } from "./schemas/changePassword.schema"
import { useTranslation } from "react-i18next"
import IChangePasswordPost from "../../types/auth/IChangePasswordPost";
import IErrorMessage from "../../types/common/IError";

function ChangePassword() {
  const redirect = useNavigate();
  const token = useParams().token
  const {register, handleSubmit} = useForm<IChangePasswordPost>({
    resolver: joiResolver(changePasswordSchema)
  });
  const [t] = useTranslation("global");
  const {mutate, isPending, error} = useChangePassword();

  const isErrorExist = (key: string) => {
    return _.findIndex(_.get(error, 'response.data.errors', []), ((err: IErrorMessage) => err.key === key))
  }

  const getErrorMessage = (key: string) => {
    return _.get(error, "response.data.errors." + isErrorExist('email') + ".message"   )
  }

  const submitHandler = (data: IChangePasswordPost) => {
    mutate(data);
  };

  return (
    <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
      <div className="flex justify-center">
        <img src={logo} className="w-20 h-20" alt="Avored Rust Cms" />
      </div>

      <div className="sm:mx-auto sm:w-full sm:max-w-md">
        <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
          {t("reset_password")}
        </h2>
      </div>
      <div></div>

      <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
        <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
          <form onSubmit={handleSubmit(submitHandler)} className="space-y-5">
            <div>
              <InputField
                label={t("New password")}
                type="password"
                name="new_password"
                register={register("new_password")}
              />
              {isErrorExist("password") >= 0 && (
                <ErrorMessage message={getErrorMessage("password")} />
              )}
            </div>
            <div>
              <InputField
                label="Confirm Password"
                type="password"
                name="confirm_password"
                register={register("confirm_password")}
              />
              {isErrorExist("confirm_password") >= 0 && (
                <ErrorMessage message={getErrorMessage("confirm_password")} />
              )}
            </div>

            <div>
              <button
                type="submit"
                className="group relative bg-primary-600 w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              >
                {isPending ? "Loading..." : t("Change password")}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}

export default ChangePassword;
