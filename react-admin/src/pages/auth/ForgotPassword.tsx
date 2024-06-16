import logo from "../../assets/logo_only.svg"
import InputField from "../../components/InputField"
import {useForm} from "react-hook-form"
import {joiResolver} from "@hookform/resolvers/joi"
import {forgotPasswordSchema} from "./schemas/forgotPassword.schema"
import _ from "lodash"
import {useForgotPassword} from "./hooks/useForgotPassword"
import {useTranslation} from "react-i18next"
import IErrorMessage from "../../types/common/IError"
import IForgotPasswordPost from "../../types/auth/IForgotPasswordPost"
import {changeLocale} from "../../lib/common"

function ForgotPassword() {
    const [t, i18n] = useTranslation("global")
    const {
        register,
        handleSubmit
    } = useForm<IForgotPasswordPost>({
        resolver: joiResolver(forgotPasswordSchema),
    })
    const {
        mutate,
        isPending,
        error
    } = useForgotPassword()

    const isErrorExist = (key: string) => {
        return _.findIndex(
            _.get(error, "response.data.errors", []),
            (err: IErrorMessage) => err.key === key
        );
    }

    const getErrorMessage = (key: string) => {
        return _.get(
            error,
            "response.data.errors." + isErrorExist("email") + ".message"
        );
    }

    const submitHandler = (data: IForgotPasswordPost) => {
        mutate(data)
    }

    return (
        <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
            <div className="flex justify-center">
                <img src={logo} className="w-20 h-20" alt="Avored Rust Cms"/>
            </div>

            <div className="sm:mx-auto sm:w-full sm:max-w-md">
                <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
                    {t("forgot_password")}
                </h2>
            </div>

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
                            {/*{isErrorExist("email") >= 0 && (*/}
                            {/*  <ErrorMessage message={getErrorMessage("email")} />*/}
                            {/*)}*/}
                        </div>

                        <div>
                            <button
                                type="submit"
                                className="group relative bg-primary-600 w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                            >
                                {isPending ? "Loading..." : t("forgot_password")}
                            </button>
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

export default ForgotPassword;
