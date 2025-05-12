import logo from "../../assets/logo_only.svg"
import {useTranslation} from "react-i18next";
import InputField from "../../components/InputField";
import ErrorMessage from "../../components/ErrorMessage";
import AvoRedButton from "../../components/AvoRedButton";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {ForgotPasswordPostType} from "../../types/auth/LoginPostType";
import {UseForgotPasswordSchema} from "../../schemas/auth/UseForgotPasswordSchema";
import {UseForgotPasswordHook} from "../../hooks/auth/useForgotPasswordHook";
import {ForgotPasswordRequest} from "../../grpc_generated/auth_pb";

export const ForgotPasswordPage = () => {
    const [t] = useTranslation("global")

    const {
        register,
        handleSubmit,
        formState: { errors }
    } = useForm<ForgotPasswordPostType>({
        resolver: joiResolver(UseForgotPasswordSchema()),
    })
    const {
        mutate,
        isPending,
        error
    } = UseForgotPasswordHook()

    const submitHandler = (data: ForgotPasswordPostType) => {
        const request = new ForgotPasswordRequest();
        request.setEmail(data.email)
        mutate(request)
    }

    return (
        <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
            <div className="flex justify-center">
                <img src={logo} className="w-20 h-20" alt={t("avored_cms_rust")}  />
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
                                label={t("email")}
                                type="text"
                                name="email"
                                autoFocus={true}
                                register={register("email")}
                            />
                            <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="email"/>
                        </div>

                        <div>
                            <AvoRedButton
                                onClick={handleSubmit}
                                label={t("forgot_password")}
                                isPending={isPending}
                                className="bg-primary-600 hover:bg-primary-500  focus:ring-primary-500"
                            />
                        </div>

                        <div className="text-gray-600 text-center text-sm">
                            {t("need_to_change_language")}
                            <select
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
    )
}
