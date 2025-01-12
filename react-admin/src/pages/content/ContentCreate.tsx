import {useTranslation} from "react-i18next";
import {ContentSidebar} from "./ContentSidebar";
import {Link, useSearchParams} from "react-router-dom";
import InputField from "../../components/InputField";
import ErrorMessage from "../../components/ErrorMessage";
import React from "react";
import {CreatableContentType} from "../../types/content/ContentType";
import slug from "slug";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {useContentCreateSchema} from "./schemas/useContentCreateSchema";
import {useStoreContent} from "./hooks/useStoreContent";

export const ContentCreate = (() => {
    const [t] = useTranslation("global");
    const [searchParams] = useSearchParams()
    const {mutate, error} = useStoreContent()
    const collectionType: string = searchParams.get("type") as string

    const submitHandler = ((data: CreatableContentType) => {
        setValue("type", collectionType)
        mutate(data)
    })

    const {
        register,
        handleSubmit,
        setValue,
        formState: {errors},
    } = useForm<CreatableContentType>({
        resolver: joiResolver(useContentCreateSchema(), {allowUnknown: true})
    })

    const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
        setValue('identifier', slug(e.currentTarget.value || ''))
    }

    return (
        <div className="flex w-full">
            <div className="p-5 w-64 bg-gray-50 min-h-screen">
                <ContentSidebar />
            </div>
            <div className="p-5 flex-1">
                <form onSubmit={handleSubmit(submitHandler)}>
                    <div className="mb-4">
                        <InputField type="hidden" register={register("type")}  />
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
                            to="/admin/collections"
                            className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                        >
                            {t("cancel")}
                        </Link>
                    </div>
                </form>
            </div>
        </div>

    )
})