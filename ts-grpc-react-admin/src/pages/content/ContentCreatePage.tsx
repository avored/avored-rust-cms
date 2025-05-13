import ErrorMessage from "../../components/ErrorMessage";
import {Link, useSearchParams} from "react-router-dom";
import {ContentSidebar} from "./ContentSidebar";
import {useTranslation} from "react-i18next";
import {UseStoreContentHook} from "../../hooks/content/UseStoreContentHook";
import InputField from "../../components/InputField";
import {UseContentCreateSchema} from "../../schemas/content/UseContentCreateSchema";
import {joiResolver} from "@hookform/resolvers/joi";
import {Controller, useFieldArray, useForm} from "react-hook-form";
import {
    ContentFieldDataType,
    ContentFieldFieldType,
    SaveContentFieldType,
    SaveContentType
} from "../../types/content/ContentType";
import slug from "slug";
import {StoreContentFieldModel, StoreContentRequest, ContentFieldFieldContent as GrpcContentFieldFieldContent} from "../../grpc_generated/content_pb";
import {ContentFieldModal} from "./ContentFieldModal";
import {useState} from "react";
import _ from "lodash";
import AvoRedButton, {ButtonType} from "../../components/AvoRedButton";
import {Cog8ToothIcon, TrashIcon} from "@heroicons/react/16/solid";
import {TextareaField} from "../../components/TextareaField";
import SimpleMDE from "react-simplemde-editor";

export const ContentCreatePage = () => {
    const [t] = useTranslation("global")
    const [searchParams] = useSearchParams()
    const [currentIndex, setCurrentIndex] = useState<number>(0);
    const [isContentFieldModalOpen, setIsContentFieldModalOpen] = useState<boolean>(false);

    const contentType: string = searchParams.get("type") as string
    const {mutate, error} = UseStoreContentHook()


    const convertToContentModal = (() => {
        var contentModal: SaveContentType = {
            name : '',
            identifier: '',
            content_type: contentType,
            content_fields: []
        }

        return contentModal
    })
    const values = convertToContentModal()

    const {
        register,
        handleSubmit,
        setValue,
        getValues,
        trigger,
        control,
        formState: {errors},
    } = useForm<SaveContentType>({
        resolver: joiResolver(UseContentCreateSchema(), {allowUnknown: true}),
        values
    })

    const { fields, append, remove } = useFieldArray({
        control,
        name: "content_fields", //rename fields
    });

    const deleteContentFieldOnClick = (e: any, index: number) => {
        e.preventDefault();
        remove(index);
        setCurrentIndex(0);
    };

    const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
        setValue('identifier', slug(e.currentTarget.value || ''))
    }

    const addFieldButtonOnClick = (async (e: React.MouseEvent<HTMLButtonElement>, max_index: number) => {
        e.preventDefault()
        append({
            name: '',
            identifier: '',
            data_type: ContentFieldDataType.TEXT,
            field_type: ContentFieldFieldType.TEXT,
            field_content: {
                text_value: ""
            }
        })
        await trigger("content_fields");
        setCurrentIndex(max_index);
        setIsContentFieldModalOpen(true)
    })


    const renderField = (field: SaveContentFieldType, index: number) => {
        switch (field.field_type) {
            case ContentFieldFieldType.TEXT:
                return (
                    <div className="mb-4">
                        <InputField
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`content_fields.${index}.field_content.text_value`)}
                        />
                    </div>
                );

            case ContentFieldFieldType.TEXTAREA:
                return (
                    <div className="mb-4">
                        <TextareaField
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`content_fields.${index}.field_content.text_value`)}
                        />
                    </div>
                );

            case ContentFieldFieldType.RICH_TEXT_EDITOR:
                return (
                    <div className="mb-4">
                        <SimpleMDE
                            value={getValues(`content_fields.${index}.field_content.text_value`)}
                            onChange={(e) => {setValue(`content_fields.${index}.field_content.text_value`, e)}}
                        />
                    </div>
                );
        }
    }

    const submitHandler = (async (data: SaveContentType) => {
        const request  = new StoreContentRequest();
        request.setName(data.name)
        request.setIdentifier(data.identifier)
        request.setContentType(contentType)
        const content_field_data_list: Array<StoreContentFieldModel> = [];

        // console.log(data.content_fields)

        data.content_fields.forEach(content_field => {
            const store_content_field_request = new StoreContentFieldModel();
            const content_field_field_content =  new GrpcContentFieldFieldContent();
            content_field_field_content.setTextValue(content_field.field_content.text_value ?? '')

            store_content_field_request.setName(content_field.name);
            store_content_field_request.setIdentifier(content_field.identifier);
            store_content_field_request.setDataType(content_field.data_type as string);
            store_content_field_request.setFieldType(content_field.field_type as string);
            store_content_field_request.setFieldContent(content_field_field_content)

            content_field_data_list.push(store_content_field_request)
        })

        request.setContentFieldsList(content_field_data_list)

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
                        {_.size(fields) > 0 ? (
                            <ContentFieldModal
                                register={register}
                                currentIndex={currentIndex}
                                getValues={getValues}
                                setValue={setValue}
                                trigger={trigger}
                                setIsOpen={setIsContentFieldModalOpen}
                                isOpen={isContentFieldModalOpen}
                                collectionType={contentType}
                            />
                        ) : (
                            <></>
                        )}

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

                        {fields.map((field, index) => {
                            return (
                                <div
                                    key={field.id}
                                    className="hover:ring-1 ring-primary-300 rounded mb-5 flex mt-5 py-3 w-full"
                                >
                                    <Controller
                                        name={`content_fields.${index}`}
                                        render={({field}) => {
                                            return (
                                                <>
                                                    <div className="flex mt-3 w-full justify-center">
                                                        <div className="flex-1 p-3">
                                                            <div className="p-3 bg-gray-200 rounded">
                                                                <div
                                                                    className="flex text-sm w-full border-gray-300 border-b py-2">
                                                                    <div className="flex-1">
                                                                        <span>
                                                                            {field.value.name}
                                                                        </span>
                                                                        <span className="ml-1 text-xs text-gray-500">
                                                                            ({field.value.identifier})
                                                                        </span>
                                                                    </div>
                                                                    <div className="ml-auto flex items-center">
                                                                        <div>
                                                                            <button
                                                                                type="button"
                                                                                className="outline-none"
                                                                                onClick={() => setIsContentFieldModalOpen(true)}
                                                                            >
                                                                                <Cog8ToothIcon className="w-5 h-5"/>
                                                                            </button>
                                                                        </div>
                                                                        <div
                                                                            onClick={(e) =>
                                                                                deleteContentFieldOnClick(e, index)
                                                                            }
                                                                            className="ml-3"
                                                                        >
                                                                            <TrashIcon className="w-4 h-4"/>
                                                                        </div>
                                                                    </div>
                                                                </div>

                                                                <InputField
                                                                    type="hidden"
                                                                    placeholder={t("data_type")}
                                                                    register={register(
                                                                        `content_fields.${index}.data_type`,
                                                                    )}
                                                                />
                                                                <InputField
                                                                    type="hidden"
                                                                    placeholder={t("field_type")}
                                                                    register={register(
                                                                        `content_fields.${index}.field_type`,
                                                                    )}
                                                                />
                                                                {renderField(field.value, index)}

                                                            </div>
                                                        </div>
                                                    </div>
                                                </>
                                            )
                                        }}
                                        control={control}
                                    />

                                </div>
                            )
                        })}

                        <div className="mb-4">
                            <AvoRedButton
                                label="Add"
                                onClick={(e: React.MouseEvent<HTMLButtonElement>) => addFieldButtonOnClick(e, fields.length)}
                                type={ButtonType.button}/>
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
