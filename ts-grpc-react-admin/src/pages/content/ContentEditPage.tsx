import {useTranslation} from "react-i18next";
import {ContentSidebar} from "./ContentSidebar";
import InputField from "../../components/InputField";
import {joiResolver} from "@hookform/resolvers/joi";
import {
    ContentFieldDataType,
    ContentFieldFieldType,
    ContentFieldType, SaveContentFieldType,
    ContentSelectFieldData,
    SaveContentType
} from "../../types/content/ContentType";
import {Controller, useFieldArray, useForm} from "react-hook-form";
import {UseContentEditSchema} from "../../schemas/content/UseContentEditSchema";
import {UseUpdateContentHook} from "../../hooks/content/UseUpdateContentHook";
import {
    ContentFieldData as GrpcContentFieldData,
    ContentSelectFieldData as GrpcContentSelectFieldData,
    ContentFieldFieldContent as GrpcContentFieldFieldContent,
    GetContentRequest,
    PutContentIdentifierRequest, StoreContentFieldModel,
    UpdateContentFieldModel,
    UpdateContentRequest
} from "../../grpc_generated/content_pb";
import {Link, useParams, useSearchParams} from "react-router-dom";
import {UseGetContentHook} from "../../hooks/content/UseGetContentHook";
import {useState} from "react";
import {UsePutContentIdentifierHook} from "../../hooks/content/UsePutContentIdentifierHook";
import {ContentFieldModal} from "./ContentFieldModal";
import _ from "lodash";
import {Cog8ToothIcon, TrashIcon} from "@heroicons/react/16/solid";
import AvoRedButton, {ButtonType} from "../../components/AvoRedButton";
import {TextareaField} from "../../components/TextareaField";
import SimpleMDE from "react-simplemde-editor";
import "easymde/dist/easymde.min.css";
import ErrorMessage from "../../components/ErrorMessage";
import {Field, Select} from "@headlessui/react";
import {ChevronDownIcon} from "@heroicons/react/24/solid";
import clsx from 'clsx'

export const ContentEditPage = () => {
    const [t] = useTranslation("global")
    const [searchParams] = useSearchParams()
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true);
    const [currentIndex, setCurrentIndex] = useState<number>(0);
    const [isContentFieldModalOpen, setIsContentFieldModalOpen] = useState<boolean>(false);

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

    const content_content_field_list = get_content_api_response?.data?.data?.contentFieldsList ?? [];
    if (values) {
        values.content_fields = [];
        content_content_field_list.map(content_field => {
            const grpc_content_field: ContentFieldType =  {
                name: content_field.name,
                identifier: content_field.identifier,
                data_type: content_field.dataType as ContentFieldDataType,
                field_type: content_field.fieldType as ContentFieldFieldType,
                field_content: {
                    text_value: content_field.fieldContent?.textValue ?? '',
                    int_value: content_field.fieldContent?.intValue ?? 0,
                    float_value: content_field.fieldContent?.floatValue ?? 0,
                }
            }

            if (content_field.fieldType as ContentFieldFieldType === ContentFieldFieldType.SELECT) {
                const select_option_data_list = content_field?.fieldData?.contentSelectFieldOptionsList ?? [];
                const options: Array<ContentSelectFieldData> = select_option_data_list as Array<unknown> as ContentSelectFieldData[];

                grpc_content_field.field_data = {
                    content_select_field_options: options
                }
            }

            values.content_fields.push(grpc_content_field)

            return grpc_content_field
        })
    }
    
    const {
        register,
        handleSubmit,
        getValues,
        formState: {errors},
        control,
        setValue,
        trigger,
    } = useForm<SaveContentType>({
        resolver: joiResolver(UseContentEditSchema(), {allowUnknown: true}),
        values
    })

    const { fields, append, remove } = useFieldArray({
        control,
        name: "content_fields", //rename fields
    });

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

    const deleteContentFieldOnClick = (e: any, index: number) => {
        e.preventDefault();
        remove(index);
        setCurrentIndex(0);
    };

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

            case ContentFieldFieldType.NUMBER_TEXT_FIELD:
                return (
                    <div className="mb-4">
                        <InputField
                            type="number"
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`content_fields.${index}.field_content.int_value`)}
                        />
                    </div>
                );

            case ContentFieldFieldType.FLOAT_TEXT_FIELD:
                return (
                    <div className="mb-4">
                        <InputField
                            type="number"
                            step="any"
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`content_fields.${index}.field_content.float_value`)}
                        />
                    </div>
                );

            case ContentFieldFieldType.SELECT:
                return (
                    <div className="mb-4">
                        <div className="w-full">
                            <Field>
                                <label className="text-sm text-gray-600">
                                    <label className="text-sm text-gray-600">
                                        {t!("field_content")}
                                    </label>
                                </label>
                                <div className="relative">
                                    <Select
                                        {...register(
                                            `content_fields.${index}.field_content.text_value`,
                                        )}
                                        className={clsx(
                                            'ring-1 ring-gray-400  px-2 py-2 bg-gray-300  mt-3 block w-full appearance-none rounded-lg border-none bg-gray-400.5 text-sm/6',
                                            'focus:not-data-focus:outline-none data-focus:outline-2 data-focus:-outline-offset-2 data-focus:outline-white/25',
                                            // Make the text of each option black on Windows

                                        )}
                                    >
                                            {field.field_data?.content_select_field_options?.map((option) => {
                                                return (
                                                    <option key={option.value} value={option.value}>
                                                        {option.label}
                                                    </option>
                                                );
                                            })}
                                    </Select>
                                    <ChevronDownIcon
                                        className="group pointer-events-none absolute mt-2 mr-2 top-1 right-1 inset-3 w-5 h-5 text-gray-400"
                                        aria-hidden="true"
                                    />
                                </div>
                            </Field>

                        </div>
                    </div>
                );
        }
    }

    const submitHandler = (async (data: SaveContentType) => {
        const request = new UpdateContentRequest();
        request.setContentId(content_id)
        request.setContentType(contentType)
        request.setName(data.name)

        const content_field_data_list: Array<UpdateContentFieldModel> = [];
        data.content_fields.forEach(content_field => {
            var float_value = content_field.field_content.float_value ?? 0;

            const update_content_field_request = new StoreContentFieldModel();
            const content_field_field_content =  new GrpcContentFieldFieldContent();
            content_field_field_content.setTextValue(content_field.field_content.text_value ?? '')
            content_field_field_content.setIntValue(content_field.field_content.int_value ?? 0)
            content_field_field_content.setFloatValue(float_value)
            content_field_field_content.setArrayValueList([])

            update_content_field_request.setName(content_field.name);
            update_content_field_request.setIdentifier(content_field.identifier);
            update_content_field_request.setDataType(content_field.data_type as string);
            update_content_field_request.setFieldType(content_field.field_type as string);
            update_content_field_request.setFieldContent(content_field_field_content)

            if (content_field.field_type === ContentFieldFieldType.SELECT) {
                const content_field_options_data = new GrpcContentFieldData();

                content_field.field_data?.content_select_field_options?.forEach((option, index) => {
                    const grpc_option = new GrpcContentSelectFieldData();
                    grpc_option.setLabel(option.label);
                    grpc_option.setValue(option.value);
                    content_field_options_data.addContentSelectFieldOptions(grpc_option, index);
                })

                update_content_field_request.setFieldData(content_field_options_data)

            }
            content_field_data_list.push(update_content_field_request)
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
                            <InputField
                                placeholder={t("name")}
                                label={t("name")}
                                name="name"
                                register={register("name")}
                            />
                            <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="name" />
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
                                                                        <span
                                                                            className="ml-1 text-xs text-gray-500">
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
