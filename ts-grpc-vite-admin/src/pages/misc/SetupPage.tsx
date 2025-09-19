import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {SetupSchema} from "../../schemas/misc/SetupSchema";
import {useSetupPost} from "../../hooks/misc/UseSetupPost";
import {SetupRequest} from "grpc-avored/misc_pb";
import {SetupType} from "../../types/misc/SetupType";
import ErrorMessage from "../../components/ErrorMessage";

export const SetupPage = () => {
    const [t] = useTranslation("global")

    const {mutate, isPending, error} = useSetupPost()
    const {
        register,
        handleSubmit,
        formState: {errors}
    } = useForm<SetupType>({
        resolver: joiResolver(SetupSchema, {allowUnknown: true})
    });

    const submitHandler = async (data: SetupType) => {
        const request = new SetupRequest()
        request.setEmail(data.email)
        request.setPassword(data.password)

        mutate(request)
    };

    return (
        <>
            <div
                className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
                <div className="sm:mx-auto sm:w-full sm:max-w-md">
                    <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        {t("setup_avored")}
                    </h2>
                </div>
                <div>

                </div>

                <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
                    <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
                        <form className="space-y-5" onSubmit={handleSubmit(submitHandler)}>
                            <div>
                                <div className="mt-3">
                                    <InputField
                                        label={t("email")}
                                        type="text"
                                        name="email"
                                        register={register('email')}
                                        autoFocus
                                    />
                                    <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="email"/>
                                </div>

                                <div className="mt-3">
                                    <InputField
                                        label={t("password")}
                                        type="password"
                                        register={register('password')}
                                    />
                                    <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="email"/>
                                </div>


                                <div className="mt-5">
                                    <button type="submit"
                                            disabled={isPending}
                                            className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
                                        {isPending ? t("loading") : t("submit")}
                                    </button>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </>
    )
}