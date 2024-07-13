import logo from "../../assets/logo_only.svg"
import { useParams } from "react-router-dom"
import InputField from "../../components/InputField"
import { useForm } from "react-hook-form"
import { joiResolver } from "@hookform/resolvers/joi"
import { useResetPassword } from "./hooks/useResetPassword"
import {useResetPasswordSchema} from "./schemas/resetPassword.schema"
import { useTranslation } from "react-i18next"
import IResetPasswordPost from "../../types/auth/IResetPasswordPost"
import {changeLocale} from "../../lib/common";
import ErrorMessage from "../../components/ErrorMessage";
import AvoRedButton from "../../components/AvoRedButton";

function ResetPassword() {
    const token = useParams().token;
    const {
        register,
        handleSubmit,
        formState: { errors }
    } = useForm<IResetPasswordPost>({
        resolver: joiResolver(useResetPasswordSchema()),
    });
    const [t, i18n] = useTranslation("global");
    const { mutate, isPending, error } = useResetPassword();

    const submitHandler = (data: IResetPasswordPost) => {
        mutate(data);
    };

    return (
        <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
            <div className="flex justify-center">
                <img src={logo} className="w-20 h-20" alt={t("avored_rust_cms")} />
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
                                label={t("email")}
                                type="text"
                                name="email"
                                autoFocus={true}
                                register={register("email")}
                            />
                            <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="email"/>
                        </div>
                        <div>
                            <InputField
                                label={t("password")}
                                type="password"
                                name="password"
                                register={register("password")}
                            />
                            <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="password"/>
                        </div>
                        <div>
                            <InputField
                                label={t("confirmation_password")}
                                type="password"
                                name="confirm_password"
                                register={register("confirm_password")}
                            />
                            <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="confirm_password"/>
                        </div>
                        <div>
                            <InputField
                                type="hidden"
                                name="token"
                                value={token}
                                register={register("token")}
                            />
                        </div>

                        <div>
                            <AvoRedButton
                                label={t("reset_password")}
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

export default ResetPassword;
