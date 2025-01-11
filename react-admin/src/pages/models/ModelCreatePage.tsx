import {Link} from "react-router-dom";
import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import {Controller, useFieldArray, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import ErrorMessage from "../../components/ErrorMessage";
import {CreatableModelType} from "../../types/model/CreatableModelType";
import {useStoreModel} from "./hooks/useStoreModel";
import {useModelCreateSchema} from "./schemas/ModelCreateSchema";
import {ModelSidebar} from "./ModelSidebar";

export const ModelCreatePage = () => {
    const [t] = useTranslation("global")
    const {mutate, error} = useStoreModel()
    const {
        control,
        register,
        handleSubmit,
        formState: {errors},
        trigger,
    } = useForm<CreatableModelType>({
        resolver: joiResolver(useModelCreateSchema(), {allowUnknown: true}),
    })

    const { fields, append, remove } = useFieldArray({
        control,
        name: "model_fields", //rename fields
    });

    const addModelAddOnClick = (async (e: React.MouseEvent<HTMLButtonElement>) => {
        e.preventDefault()
        append({
            name: ""
        })
        await trigger("model_fields")
    })

    const submitHandler = ((data: CreatableModelType) => {
        mutate(data)
    })
    return (
        <>
            <div className="px-5">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("model_information")}
                        </h1>
                        <div className="flex w-full">
                            <div className="w-64">
                                <ModelSidebar />
                            </div>

                            <div className="ml-3 flex-1">
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
                                    {fields.map((field, index) => {
                                        return (
                                            <div key={`model-field-key-${index}`} className="block">
                                                <div className="mb-4">
                                                    <InputField
                                                        label={t("name")}
                                                        placeholder={t("name")}
                                                        name="name"
                                                        register={register(`model_fields.${index}.name`)}
                                                    />
                                                    <ErrorMessage
                                                        frontendErrors={errors}
                                                        backendErrors={error}
                                                        identifier={`model_fields.${index}.name`}
                                                    />
                                                </div>
                                            </div>
                                        )
                                    })}

                                    <div className="mb-4">
                                        <button type="button" onClick={e => addModelAddOnClick(e)}>
                                            add on click
                                        </button>

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
            </div>
        </>
    );
};
