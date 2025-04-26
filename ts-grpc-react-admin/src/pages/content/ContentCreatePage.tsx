import ErrorMessage from "../../components/ErrorMessage";
import {Link, useSearchParams} from "react-router-dom";
import {ContentSidebar} from "./ContentSidebar";
import {useTranslation} from "react-i18next";
import {UseStoreContentHook} from "../../hooks/content/UseStoreContentHook";
import InputField from "../../components/InputField";
import {UseContentCreateSchema} from "../../schemas/content/UseContentCreateSchema";
import {joiResolver} from "@hookform/resolvers/joi";
import {useForm} from "react-hook-form";
import {SaveContentType} from "../../types/content/ContentType";
import slug from "slug";
import {StoreContentRequest} from "../../grpc_generated/content_pb";

export const ContentCreatePage = () => {
    const [t] = useTranslation("global")
    const [searchParams] = useSearchParams()
    const contentType: string = searchParams.get("type") as string
    const {mutate, error} = UseStoreContentHook()
    const {
        register,
        handleSubmit,
        setValue,
        formState: {errors},
    } = useForm<SaveContentType>({
        resolver: joiResolver(UseContentCreateSchema(), {allowUnknown: true})
    })

    const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
        setValue('identifier', slug(e.currentTarget.value || ''))
    }


    const submitHandler = (async (data: SaveContentType) => {
        const request  = new StoreContentRequest();
        request.setName(data.name)
        request.setIdentifier(data.identifier)
        request.setContentType(contentType)
        mutate(request)
    })

    return(
        <>
            <div className="flex w-full">
                <div className="p-5 w-64 bg-gray-50 min-h-screen">
                    <ContentSidebar />
                </div>
                <div className="p-5 flex-1">
                    <form onSubmit={handleSubmit(submitHandler)}>


                        <div className="mb-4">
                            <InputField type="hidden" register={register("content_type")} value={contentType}  />
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
                                to={`/admin/content?type=${contentType}`}
                                className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                            >
                                {t("cancel")}
                            </Link>
                        </div>
                    </form>
                </div>
            </div>
        </>
    )
}