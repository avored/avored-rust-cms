import React from "react"
import {Link, useParams} from "react-router-dom"
import {PlusIcon} from "@heroicons/react/24/solid"
import {TrashIcon} from "@heroicons/react/16/solid"
import InputField from "../../components/InputField"
import {useGetComponent} from "./hooks/useGetComponent"
import {useUpdateComponent} from "./hooks/useUpdateComponent"
import {useTranslation} from "react-i18next"
import {Controller, useFieldArray, useForm} from "react-hook-form"
import {joiResolver} from "@hookform/resolvers/joi"
import {ComponentEditSchema} from "./schemas/component.edit.schema"
import {AvoRedFieldTypesEnum} from "../../types/field/AvoRedFieldTypesEnum"
import IEditableComponent from "../../types/field/IEditableComponent"
import {ErrorMessage} from "../../components/ErrorMessage"
import {IOptionField} from "../../types/field/IEditableField"

function ComponentEdit() {

    const [t] = useTranslation("global")
    const params = useParams()
    const {mutate} = useUpdateComponent(params.component_id ?? '')
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
        resolver: joiResolver(ComponentEditSchema, {allowUnknown: true}),
        values
    })
    const {
        fields,
        append,
        remove
    } = useFieldArray({
        control,
        name: "fields",
    });

    const addFieldOnClick = (() => {
        append({id: '', name: '', identifier: '', field_type: AvoRedFieldTypesEnum.TEXT})
    })

    const deleteFieldOnClick = ((fieldIndex: number) => {
        remove(fieldIndex)
    })
    const optionDeleteActionOnClick = ((
        e: React.MouseEvent,
        fieldIndex: number,
        field_data: Array<IOptionField> | undefined,
        option_index: number
    ) => {
        field_data?.splice(option_index, 1)
        setValue(`fields.${fieldIndex}.field_data`, field_data)
        trigger(`fields.${fieldIndex}`)
    })
    const optionAddActionOnClick = ((e: React.MouseEvent, fieldIndex: number, field_data: Array<IOptionField> | undefined) => {
        field_data?.push({label: '', value: ''})
        setValue(`fields.${fieldIndex}.field_data`, field_data)
        trigger(`fields.${fieldIndex}`)
    })
    const fieldTypeButtonOnClick = ((fieldIndex: number, fieldTypeValue: string) => {
        setValue(`fields.${fieldIndex}.field_type`, fieldTypeValue)
        setValue(`fields.${fieldIndex}.field_data`, [{label: '', value: ''}])
        trigger(`fields.${fieldIndex}`)
    })

    const getErrorMessage = ((name: string) => {
        console.log(errors, name)
        return ''
    })

    const submitHandler = ((data: any) => {
        // console.log(data)
        mutate(data)
    })

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900 dark:text-gray-100">
                            {t("component.information")}
                        </h1>
                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    label={t('common.name')}
                                    placeholder={t('common.name')}
                                    name="name"
                                    register={register("name")}
                                    autoFocus={true}
                                />
                                <ErrorMessage message={getErrorMessage('name')}/>
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t('common.identifier')}
                                    placeholder={t('common.identifier')}
                                    name="identifier"
                                    register={register("identifier")}
                                />
                            </div>

                            {fields.map((field, index) => {
                                return (
                                    <div
                                        key={`field-${index}-key`}
                                        className="block ring-1 ring-gray-300 mb-4 rounded p-5"
                                    >
                                        <div className="flex w-full">
                                            <button
                                                type="button"
                                                onClick={() => {
                                                    deleteFieldOnClick(index);
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
                                                        fieldTypeButtonOnClick(index, AvoRedFieldTypesEnum.TEXT)
                                                    }
                                                    className={`${
                                                        field.field_type === AvoRedFieldTypesEnum.TEXT
                                                            ? "bg-primary-300"
                                                            : "bg-gray-300"
                                                    } ring-1 p-3 mt-3 rounded`}
                                                >
                                                    {t("common.text")}
                                                </div>
                                                <div
                                                    onClick={(e) =>
                                                        fieldTypeButtonOnClick(index, AvoRedFieldTypesEnum.TEXTAREA)
                                                    }
                                                    className={`${
                                                        field.field_type === AvoRedFieldTypesEnum.TEXTAREA
                                                            ? "bg-primary-300"
                                                            : "bg-gray-300"
                                                    } ring-1 p-3 mt-3 rounded`}
                                                >
                                                    {t("common.textarea")}
                                                </div>
                                                <div
                                                    onClick={(e) =>
                                                        fieldTypeButtonOnClick(index, AvoRedFieldTypesEnum.SELECT)
                                                    }
                                                    className={`${
                                                        field.field_type === AvoRedFieldTypesEnum.SELECT
                                                            ? "bg-primary-300"
                                                            : "bg-gray-300"
                                                    } ring-1 p-3 mt-3 rounded`}
                                                >
                                                    {t("common.select")}
                                                </div>
                                            </div>

                                            <div className="p-3 w-2/3">
                                                <div className="mt-3">
                                                    Field Type: {field.field_type}
                                                    <InputField
                                                        name={`fields.${index}.field_type`}
                                                        register={register(`fields.${index}.field_type`)}
                                                    />
                                                </div>
                                                <div className="mt-3">
                                                    <InputField
                                                        type="text"
                                                        name={register(`fields.${index}.id`)}
                                                        register={register(`fields.${index}.id`)}
                                                    />
                                                </div>
                                                <div className="mt-3">
                                                    <InputField
                                                        name={`fields.${index}.name`}
                                                        register={register(`fields.${index}.name`)}
                                                        label={t('pages.component.field_name')}
                                                        placeholder={t('pages.component.field_name')}
                                                    />
                                                </div>
                                                <div className="mt-3">
                                                    <InputField
                                                        name={`fields.${index}.identifier`}
                                                        register={register(`fields.${index}.identifier`)}
                                                        label={t('pages.component.field_identifier')}
                                                        placeholder={t('pages.component.field_identifier')}
                                                    />
                                                </div>
                                                <Controller
                                                    name={`fields.${index}`}
                                                    render={({field}) => {
                                                        return (
                                                            field.value.field_type === AvoRedFieldTypesEnum.SELECT ? (
                                                                <div key={`field-type-${field.value.field_type}`}
                                                                     className="mt-3">
                                                                    <div className="w-full">
                                                                        <h6 className="font-semibold">
                                                                            {t("pages.component.options")}
                                                                        </h6>
                                                                    </div>

                                                                    {field.value.field_data?.map((field_option, field_option_index) => {
                                                                        return (
                                                                            <div className="flex w-full"
                                                                                 key={`field-option-key-${field_option_index}-${field.value.id}`}>

                                                                                <div className="w-1/2">
                                                                                    <label
                                                                                        htmlFor="hs-inline-leading-pricing-select-label"
                                                                                        className="text-sm text-gray-600"
                                                                                    >{t('pages.component.option_label')}
                                                                                    </label>
                                                                                    <InputField
                                                                                        register={register(`fields.${index}.field_data.${field_option_index}.label`)}
                                                                                        placeholder={t('pages.component.option_label')}
                                                                                    />
                                                                                </div>

                                                                                <div className="w-1/2 ml-3">

                                                                                    <label
                                                                                        htmlFor="hs-inline-leading-pricing-select-label"
                                                                                        className="text-sm text-gray-600"
                                                                                    >{t('pages.component.option_value')}
                                                                                    </label>
                                                                                    <div className="relative">

                                                                                        <InputField
                                                                                            register={register(`fields.${index}.field_data.${field_option_index}.value`)}
                                                                                            placeholder={t('pages.component.option_value')}
                                                                                        />
                                                                                        <div
                                                                                            onClick={(e: React.MouseEvent) => optionDeleteActionOnClick(e, index, field.value.field_data, field_option_index)}
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
                                                                        onClick={(e: React.MouseEvent) => optionAddActionOnClick(e, index, field.value.field_data)}>
                                                                        <button className="flex items-center"
                                                                                type="button">
                                                                            <PlusIcon
                                                                                className="text-primary-500 h-6 w-6"/>
                                                                            <span
                                                                                className="text-sm ml-1 text-primary-500">
                                                                                            {t("pages.component.add_option")}
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
                                        {t("common.add_field")}
                                    </span>
                                </button>
                            </div>

                            <div className="mt-5 flex items-center">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("common.save")}
                                </button>
                                <Link
                                    to="/admin/component"
                                    className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                                >
                                    {t("common.cancel")}
                                </Link>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default ComponentEdit
