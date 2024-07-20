import { Link } from "react-router-dom";
import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import ErrorMessage from "../../components/ErrorMessage";
import {CreatableModelType} from "../../types/model/CreatableModelType";
import {useStoreModel} from "./hooks/useStoreModel";
import {usePageCreateSchema} from "./schemas/PageCreateSchema";

export const ModelCreatePage = () => {
    const [t] = useTranslation("global")
    const {mutate, error} = useStoreModel()
    const {
        register,
        handleSubmit,
        formState: {errors},
    } = useForm<CreatableModelType>({
        resolver: joiResolver(usePageCreateSchema(), {allowUnknown: true}),
    })

    const submitHandler = ((data: CreatableModelType) => {
        mutate(data)
    })
    return (
        <>
            <div className="flex-1 bg-white">
                <div className="px-5 pl-64 ">
                    <div className="w-full">
                        <div className="block rounded-lg p-6">
                              <h1 className="text-xl font-semibold mb-4 text-gray-900">
                                {t("model_information")}
                              </h1>

                              <form onSubmit={handleSubmit(submitHandler)}>
                                    <div className="mb-4">
                                          <InputField
                                            label={t("name")}
                                            placeholder={t("name")}
                                            name="name"
                                            register={register("name")}
                                            autoFocus={true}
                                          />
                                        <ErrorMessage
                                            frontendErrors={errors}
                                            backendErrors={error}
                                            identifier="name"
                                        />
                                    </div>
                                    <div className="mb-4">
                                        <InputField
                                            label={t("identifier")}
                                            placeholder={t("identifier")}
                                            name="identifier"
                                            register={register("identifier")}
                                        />
                                        <ErrorMessage
                                            frontendErrors={errors}
                                            backendErrors={error}
                                            identifier="identifier"
                                        />
                                    </div>
                                    <div className="flex items-center">
                                      <button
                                        type="submit"
                                        className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                      >
                                        {t("save")}
                                      </button>
                                      <Link
                                        to="/admin/model"
                                        className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                                      >
                                        {t("cancel")}
                                      </Link>
                                    </div>
                              </form>
                        </div>
                  </div>
                </div>
            </div>
        </>
    );
};
