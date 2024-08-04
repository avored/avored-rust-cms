import React, {ChangeEvent, ReactEventHandler, useState} from "react"
import {Link, useParams} from "react-router-dom"
import {PlusIcon} from "@heroicons/react/24/solid"
import {TrashIcon} from "@heroicons/react/16/solid"
import InputField from "../../components/InputField"
import {useGetComponent} from "./hooks/useGetComponent"
import {useUpdateComponent} from "./hooks/useUpdateComponent"
import {useTranslation} from "react-i18next"
import {Controller, useFieldArray, useForm} from "react-hook-form"
import {joiResolver} from "@hookform/resolvers/joi"
import slug from 'slug'
import {useComponentEditSchema} from "./schemas/component.edit.schema"
import {AvoRedFieldTypesEnum} from "../../types/field/AvoRedFieldTypesEnum"
import IEditableComponent, {
  EditableComponentElementDataType,
  
} from "../../types/component/IEditableComponent";
import {usePutComponentIdentifier} from "./hooks/usePutComponentIdentifier"
import {useComponentPutSchema} from "./schemas/component.put.schema";
import {PutComponentIdentifierType} from "../../types/component/PutComponentIdentifierType";
import ErrorMessage from "../../components/ErrorMessage";

export const  ComponentEditPage = (() => {

    const [t] = useTranslation("global")
    const params = useParams()
    const {mutate, error} = useUpdateComponent(params.component_id ?? '')
    const {data} = useGetComponent(params.component_id ?? '')
    const values = data?.data.component_model

    const {
        control,
        register,
        handleSubmit,
        formState: {errors},
        setValue,
        trigger
    } = useForm<IEditableComponent>({
        resolver: joiResolver(useComponentEditSchema(), {allowUnknown: true}),
        values
    })
    const {
        fields: elements,
        append,
        remove
    } = useFieldArray({
        control,
        name: "elements",
    });

    const {
        register: putComponentRegister,
        getValues: getComponentIdentifierValue
    } = useForm<PutComponentIdentifierType>({
        resolver: joiResolver(useComponentPutSchema(), {allowUnknown: true}),
        values: {
            identifier: data?.data.component_model.identifier
        }
    });
    const {mutate: putComponentIdentifierMutate} = usePutComponentIdentifier(params.component_id ?? '')
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)

    const editableIdentifierOnClick = (() => {
        setIsEditableIdentifier(false)
    })
    const saveIdentifierOnClick = (() => {
        putComponentIdentifierMutate({identifier: getComponentIdentifierValue('identifier')})
        setIsEditableIdentifier(true)
    })

    const cancelIdentifierOnClick = (() => {
        setIsEditableIdentifier(true)
    })

    const addFieldOnClick = (() => {
        append({name: '', identifier: '', element_type: AvoRedFieldTypesEnum.TEXT})
    })

    const deleteElementOnClick = ((elementIndex: number) => {
        remove(elementIndex)
    })

    const optionDeleteActionOnClick = (async (
        e: React.MouseEvent,
        elementIndex: number,
        element_data: Array<EditableComponentElementDataType> | undefined,
        option_index: number
    ) => {
        element_data?.splice(option_index, 1)
        setValue(`elements.${elementIndex}.element_data`, element_data)
        await trigger(`elements.${elementIndex}`)
    })

    const optionAddActionOnClick = (async (
        e: React.MouseEvent,
        elementIndex: number,
        element_data: Array<EditableComponentElementDataType> | undefined
    ) => {
        element_data?.push({label: '', value: ''})
        setValue(`elements.${elementIndex}.element_data`, element_data)
        await trigger(`elements.${elementIndex}`)
    })

    const elementTypeButtonOnClick = (async (
        elementIndex: number,
        fieldTypeValue: string
    ) => {
        setValue(`elements.${elementIndex}.element_type`, fieldTypeValue)
        setValue(`elements.${elementIndex}.element_data`, [{label: '', value: ''}])
        await trigger(`elements.${elementIndex}`)
    })

    const submitHandler = ((data: any) => {
        mutate(data)
    })

    const ElementNameOnChange = (async (e: React.KeyboardEvent<HTMLInputElement>, elementIndex: number) => {
        setValue(`elements.${elementIndex}.identifier`, slug(e.currentTarget.value))
        await trigger(`elements.${elementIndex}`)
    })

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900 dark:text-gray-100">
                            {t("component_information")}
                        </h1>
                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    label={t('name')}
                                    placeholder={t('name')}
                                    name="name"
                                    register={register("name")}
                                    autoFocus={true}
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="name" />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("identifier")}
                                    placeholder={t("identifier")}
                                    name="identifier"
                                    register={putComponentRegister("identifier")}
                                    disabled={isEditableIdentifier}
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="identifier" />
                                <div
                                    className="mt-2"
                                >
                                    {isEditableIdentifier ? (
                                        <>
                                            <span onClick={editableIdentifierOnClick}
                                                  className="text-xs text-blue-600 cursor-pointer">
                                                {t("edit_identifier")}
                                            </span>
                                        </>
                                    ) : (
                                        <>
                                            <button type="button" onClick={saveIdentifierOnClick}
                                                    className="text-xs text-blue-600 cursor-pointer">
                                                {t('save')}
                                            </button>
                                            <button type="button" onClick={cancelIdentifierOnClick}
                                                    className="ml-3 text-xs text-blue-600 cursor-pointer">
                                                {t('cancel')}
                                            </button>
                                        </>
                                    )}
                                </div>
                            </div>

                            {elements.map((element, index) => {
                                return (
                                    <div
                                        key={`element-${index}-key`}
                                        className="block ring-1 ring-gray-300 mb-4 rounded p-5"
                                    >
                                        <div className="flex w-full">
                                            <button
                                                type="button"
                                                onClick={() => {
                                                    deleteElementOnClick(index);
                                                }}
                                                className="ml-auto"
                                            >
                                                <TrashIcon className="w-4 h-4"/>
                                            </button>
                                        </div>

                                        <div className="flex w-full">
                                            <div className="border-r p-5 w-1/3">
                                                <div
                                                    onClick={(e) =>
                                                        elementTypeButtonOnClick(index, AvoRedFieldTypesEnum.TEXT)
                                                    }
                                                    className={`${
                                                        element.element_type === AvoRedFieldTypesEnum.TEXT
                                                            ? "bg-primary-300"
                                                            : "bg-gray-300"
                                                    } ring-1 p-3 mt-3 rounded`}
                                                >
                                                    {t("text")}
                                                </div>
                                                <div
                                                    onClick={(e) =>
                                                        elementTypeButtonOnClick(index, AvoRedFieldTypesEnum.TEXTAREA)
                                                    }
                                                    className={`${
                                                        element.element_type === AvoRedFieldTypesEnum.TEXTAREA
                                                            ? "bg-primary-300"
                                                            : "bg-gray-300"
                                                    } ring-1 p-3 mt-3 rounded`}
                                                >
                                                    {t("textarea")}
                                                </div>
                                                <div
                                                    onClick={(e) =>
                                                        elementTypeButtonOnClick(index, AvoRedFieldTypesEnum.SELECT)
                                                    }
                                                    className={`${
                                                        element.element_type === AvoRedFieldTypesEnum.SELECT
                                                            ? "bg-primary-300"
                                                            : "bg-gray-300"
                                                    } ring-1 p-3 mt-3 rounded`}
                                                >
                                                    {t("select")}
                                                </div>
                                            </div>

                                            <div className="p-3 w-2/3">
                                                <div className="mt-3">
                                                    <Controller
                                                        name={`elements.${index}.element_type`}
                                                        render={({field}) => {
                                                            return (
                                                                <>
                                                                {t!('field_type')}: {field.value}
                                                                </>
                                                            )
                                                        }}
                                                        control={control}
                                                    />

                                                    <InputField
                                                        type="hidden"
                                                        name={`elements.${index}.element_type`}
                                                        register={register(`elements.${index}.element_type`)}
                                                    />
                                                </div>
                                                
                                                <div className="mt-3">
                                                    <InputField
                                                        name={`elements.${index}.name`}
                                                        register={register(`elements.${index}.name`)}
                                                        onKeyUp={e => ElementNameOnChange(e, index)}
                                                        label={t('element_name')}
                                                        placeholder={t('element_name')}
                                                    />
                                                </div>
                                                <div className="mt-3">
                                                    <InputField
                                                        name={`elements.${index}.identifier`}
                                                        register={register(`elements.${index}.identifier`)}
                                                        label={t('element_identifier')}
                                                        placeholder={t('element_identifier')}
                                                    />
                                                </div>
                                                <Controller
                                                    name={`elements.${index}`}
                                                    render={({field: element}) => {
                                                        return (
                                                            element.value.element_type === AvoRedFieldTypesEnum.SELECT ? (
                                                                <div key={`field-type-${element.value.element_type}`}
                                                                     className="mt-3">
                                                                    <div className="w-full">
                                                                        <h6 className="font-semibold">
                                                                            {t("element_options")}
                                                                        </h6>
                                                                    </div>

                                                                    {element.value.element_data?.map((element_option, element_data_index) => {
                                                                        return (
                                                                            <div className="flex w-full"
                                                                                 key={`element-option-key-${element_data_index}-${element.value.identifier}`}>

                                                                                <div className="w-1/2">
                                                                                    <label
                                                                                        htmlFor="hs-inline-leading-pricing-select-label"
                                                                                        className="text-sm text-gray-600"
                                                                                    >{t('element_option_label')}
                                                                                    </label>
                                                                                    <InputField
                                                                                        name={`elements.${index}.element_data.${element_data_index}.label`}
                                                                                        register={register(`elements.${index}.element_data.${element_data_index}.label`)}
                                                                                        placeholder={t('element_option_label')}
                                                                                    />
                                                                                </div>

                                                                                <div className="w-1/2 ml-3">

                                                                                    <label
                                                                                        htmlFor="hs-inline-leading-pricing-select-label"
                                                                                        className="text-sm text-gray-600"
                                                                                    >{t('element_option_value')}
                                                                                    </label>
                                                                                    <div className="relative">

                                                                                        <InputField
                                                                                            name={`elements.${index}.element_data.${element_data_index}.label`}
                                                                                            register={register(`elements.${index}.element_data.${element_data_index}.value`)}
                                                                                            placeholder={t('element_option_value')}
                                                                                        />
                                                                                        <div
                                                                                            onClick={(e: React.MouseEvent) => optionDeleteActionOnClick(e, index, element.value.element_data, element_data_index)}
                                                                                            className="absolute inset-y-0 end-0 z-40 flex items-center text-gray-500"
                                                                                        >
                                                                                            <TrashIcon
                                                                                                className="text-primary-500 w-4 h-4 mr-2"/>
                                                                                        </div>
                                                                                    </div>
                                                                                </div>


                                                                            </div>
                                                                        )
                                                                    })}

                                                                    <div
                                                                        className="mt-4 flex justify-center ring-1 ring-gray-300 rounded p-1"
                                                                        onClick={(e: React.MouseEvent) => optionAddActionOnClick(e, index, element.value.element_data)}>
                                                                        <button className="flex items-center"
                                                                                type="button">
                                                                            <PlusIcon
                                                                                className="text-primary-500 h-6 w-6"/>
                                                                            <span
                                                                                className="text-sm ml-1 text-primary-500">
                                                                                            {t("add_option")}
                                                                                        </span>
                                                                        </button>
                                                                    </div>

                                                                </div>
                                                            ) : <></>
                                                        )
                                                    }}
                                                    control={control}
                                                />


                                            </div>
                                        </div>
                                    </div>
                                );
                            })}

                            <div className="mb-4 flex items-center justify-center ring-1 ring-gray-300 rounded p-3">
                                <button
                                    type="button"
                                    className="flex"
                                    onClick={addFieldOnClick}
                                >
                                    <PlusIcon className="text-primary-500 h-6 w-6"/>
                                    <span className="text-sm ml-1 text-primary-500">
                                        {t("add_field")}
                                    </span>
                                </button>
                            </div>

                            <div className="mt-5 flex items-center">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("save")}
                                </button>
                                <Link
                                    to="/admin/component"
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
    );
})
