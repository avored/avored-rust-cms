import {Link} from "react-router-dom";
import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import ErrorMessage from "../../components/ErrorMessage";
import {CreatableCollectionType} from "../../types/collection/CreatableCollectionType";
import {useStoreCollection} from "./hooks/useStoreCollection";
import {useCollectionCreateSchema} from "./schemas/CollectionCreateSchema";
import React from "react";
import slug from "slug";

export const CollectionCreate = () => {
    const [t] = useTranslation("global")
    const {mutate, error} = useStoreCollection()
    const {
        register,
        handleSubmit,
        setValue,
        formState: {errors},
    } = useForm<CreatableCollectionType>({
        resolver: joiResolver(useCollectionCreateSchema(), {allowUnknown: true}),
    })

    const submitHandler = ((data: CreatableCollectionType) => {
        mutate(data)
    })

    const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
        setValue('identifier', slug(e.currentTarget.value || ''))
    }

    return (
        <>
            <div className="px-5">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("collection_information")}
                        </h1>

                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    label={t("name")}
                                    placeholder={t("name")}
                                    name="name"
                                    register={register("name")}
                                    onKeyUp={e => onNameChange(e)}
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
                                    to="/admin/collection"
                                    className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                                >
                                    {t("cancel")}
                                </Link>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </>
    );
};
