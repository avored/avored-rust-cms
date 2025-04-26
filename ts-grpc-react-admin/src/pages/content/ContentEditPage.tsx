import {useTranslation} from "react-i18next";
import {ContentSidebar} from "./ContentSidebar";
import InputField from "../../components/InputField";
import { joiResolver } from "@hookform/resolvers/joi";
import {SaveContentType} from "../../types/content/ContentType";
import {useForm} from "react-hook-form";
import {UseContentEditSchema} from "../../schemas/content/UseContentEditSchema";
import {UseUpdateContentHook} from "../../hooks/content/UseUpdateContentHook";
import {GetContentRequest, PutContentIdentifierRequest, UpdateContentRequest} from "../../grpc_generated/content_pb";
import {Link, useParams, useSearchParams} from "react-router-dom";
import {UseGetContentHook} from "../../hooks/content/UseGetContentHook";
import {useState} from "react";
import {UsePutContentIdentifierHook} from "../../hooks/content/UsePutContentIdentifierHook";

export const ContentEditPage = () => {
    const [t] = useTranslation("global")
    const [searchParams] = useSearchParams()
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true);
    const params = useParams()
    const content_id = params.content_id as string;
    const contentType: string = searchParams.get("type") as string

    const {mutate, error} = UseUpdateContentHook()
    const { mutate: putContentIdentifierMutate } = UsePutContentIdentifierHook();

    const request = new GetContentRequest()
    request.setContentType(contentType)
    request.setContentId(content_id)
    const get_content_api_response = UseGetContentHook(request)
    const get_content_model = get_content_api_response?.data?.data ?? [];
    const values: SaveContentType = get_content_model as unknown as SaveContentType;


    const {
        register,
        handleSubmit,
        getValues,
        formState: {errors},
    } = useForm<SaveContentType>({
        resolver: joiResolver(UseContentEditSchema(), {allowUnknown: true}),
        values
    })

    const editableIdentifierOnClick = () => {
        setIsEditableIdentifier(false);
    };
    const saveIdentifierOnClick = () => {
        const request = new PutContentIdentifierRequest();
        request.setContentType(contentType);
        request.setContentId(content_id);
        request.setIdentifier(getValues("identifier"));
        putContentIdentifierMutate(request);

        setIsEditableIdentifier(true);
    };

    const cancelIdentifierOnClick = () => {
        setIsEditableIdentifier(true);
    };

    const submitHandler = (async (data: SaveContentType) => {
        const request = new UpdateContentRequest();
        request.setContentId(content_id)
        request.setContentType(contentType)
        request.setName(data.name)

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
                            <InputField
                                placeholder={t("name")}
                                label={t("name")}
                                name="name"
                                register={register("name")}
                            />
                        </div>

                        <div className="mb-4">
                            <InputField
                                placeholder={t("identifier")}
                                name="identifier"
                                register={register("identifier")}
                                disabled={isEditableIdentifier}
                            />
                            <div className="mt-2">
                                {isEditableIdentifier ? (
                                    <>
                          <span
                              onClick={editableIdentifierOnClick}
                              className="text-xs text-blue-600 cursor-pointer"
                          >
                            {t("edit_identifier")}
                          </span>
                                    </>
                                ) : (
                                    <>
                                        <button
                                            type="button"
                                            onClick={saveIdentifierOnClick}
                                            className="text-xs text-blue-600 cursor-pointer"
                                        >
                                            {t("save")}
                                        </button>
                                        <button
                                            type="button"
                                            onClick={cancelIdentifierOnClick}
                                            className="ml-3 text-xs text-blue-600 cursor-pointer"
                                        >
                                            {t("cancel")}
                                        </button>
                                    </>
                                )}
                            </div>
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