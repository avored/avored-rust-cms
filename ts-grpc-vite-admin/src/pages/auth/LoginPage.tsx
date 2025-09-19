import {useTranslation} from "react-i18next";
import {SubmitHandler, useForm} from "react-hook-form";
import {LoginPostType} from "../../types/auth/LoginPostType";
import {joiResolver} from "@hookform/resolvers/joi";
import {UseLoginSchema} from "../../schemas/auth/UseLoginSchema";
import {UseLoginHook} from "../../hooks/auth/UseLoginHook";
import InputField from "../../components/InputField";
import ErrorMessage from "../../components/ErrorMessage";
import {Link} from "react-router-dom";
import AvoRedButton from "../../components/AvoRedButton";
import {LoginRequest} from "grpc-avored/auth_pb";

export const LoginPage = (() => {
    const [t, i18n] = useTranslation("global")
    const {
        register,
        handleSubmit,
        formState: { errors }
    } = useForm<LoginPostType>({
        resolver: joiResolver(UseLoginSchema()),
    })
    const {
        mutate,
        isPending,
        error
    } = UseLoginHook()

    const changeLocale = ((i18n: any, target: any) => {
        console.log("changeLocale", target, i18n)
    })
    const submitHandler: SubmitHandler<LoginPostType> = (data) => {
        var request = new LoginRequest();
        request.setEmail(data.email)
        request.setPassword(data.password)

        mutate(request);
    }
    return(
        <>
            <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
                <div className="flex justify-center">
                    <img src="/avored.svg" className="w-20 h-20" alt={t("avored_rust_cms")} />
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
                                    onClick={handleSubmit}
                                    label={t("sign_in")}
                                    isPending={isPending}
                                    className="bg-primary-600 hover:bg-primary-500 focus:ring-primary-500"
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
        </>
    )
})
