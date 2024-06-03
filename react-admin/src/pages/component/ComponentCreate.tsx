import React, {useState} from "react"
import {Link} from "react-router-dom"
import {PlusIcon, TrashIcon} from "@heroicons/react/24/solid"
import InputField from "../../components/InputField";
import {useStoreComponent} from "./hooks/useStoreComponent";
import {useTranslation} from "react-i18next"
import CreatableFieldType from "../../types/field/CreatableFieldType"
import {AvoRedFieldTypesEnum} from "../../types/field/AvoRedFieldTypesEnum";
import CreatableFieldStoreData from "../../types/field/CreatableFieldStoreData";
import {randomString} from "../../lib/common";

function ComponentCreate() {
    const [fields, setFields] = useState<CreatableFieldType[]>([])
    const [name, setName] = useState('')
    const [identifier, setIdentifier] = useState('')
    const {mutate} = useStoreComponent()
    const [t] = useTranslation("global")

    const addFieldOnClick = (() => {
        var field: CreatableFieldType = {
            id: randomString(),
            field_type: AvoRedFieldTypesEnum.TEXT,
            name: '',
            identifier: ''
        };
        setFields(fields => [...fields, field])
    })

    const setNameOnChange = ((e: React.ChangeEvent<HTMLInputElement>) => {
        setName(e.target.value)
    })
    const setIdentifierOnChange = ((e: React.ChangeEvent<HTMLInputElement>) => {
        setIdentifier(e.target.value)
    })

    const deleteFieldOnClick = ((fieldIndex: string) => {
        var array = [...fields]; // make a separate copy of the array

        array = array.filter((ele) => ele.id !== fieldIndex)
        setFields(array);
    })

    const fieldTypeButtonOnClick = ((fieldIndex: string, fieldTypeValue: string) => {
        setFields(prevFields => {
            return prevFields.map(field => {
                if (field.id === fieldIndex) {
                    field.field_type = fieldTypeValue
                    return field
                }
                return field
            })
        })
    })

    const fieldNameChange = ((fieldIndex: string, fieldValue: React.ChangeEvent<HTMLInputElement>) => {
        setFields(prevFields => {
            return prevFields.map(field => {
                if (field.id === fieldIndex) {
                    field.name = fieldValue.target.value
                    return field
                }
                return field
            })
        })
    })

    const fieldIdentifierChange = ((fieldIndex: string, fieldValue: React.ChangeEvent<HTMLInputElement>) => {
        setFields(prevFields => {
            return prevFields.map(field => {
                if (field.id === fieldIndex) {
                    field.identifier = fieldValue.target.value
                    return field
                }
                return field
            })
        })
    })

    const handleSubmit = (async (e: React.FormEvent) => {
        e.preventDefault()
        const mutateData: CreatableFieldStoreData = {
            name,
            identifier,
            fields
        }
        mutate(mutateData)
    })


    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64 ">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900 dark:text-gray-100">
                            {t("component.information")}
                        </h1>
                        <form onSubmit={handleSubmit}>
                            <div className="mb-4">
                                <InputField
                                    label={t('common.name')}
                                    placeholder={t('common.name')}
                                    value={name}
                                    onChange={setNameOnChange}
                                    autoFocus={true}
                                />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t('common.identifier')}
                                    placeholder={t('common.identifier')}
                                    value={identifier}
                                    onChange={setIdentifierOnChange}
                                />
                            </div>

                            {fields.map((field) => {
                                return (
                                    <div
                                        key={field.id}
                                        className="block ring-1 ring-gray-300 mb-4 rounded p-5"
                                    >
                                        <div className="flex w-full">
                                            <button
                                                type="button"
                                                onClick={() => {
                                                    deleteFieldOnClick(field.id);
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
                                                        fieldTypeButtonOnClick(field.id, AvoRedFieldTypesEnum.TEXT)
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
                                                        fieldTypeButtonOnClick(field.id, AvoRedFieldTypesEnum.TEXTAREA)
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
                                                        fieldTypeButtonOnClick(field.id, AvoRedFieldTypesEnum.SELECT)
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
                                                </div>
                                                <div className="mt-3">
                                                    <InputField
                                                        label={t('pages.component.field_name')}
                                                        placeholder={t('pages.component.field_name')}
                                                        onChange={(e: React.ChangeEvent<HTMLInputElement>) => fieldNameChange(field.id, e)}
                                                    />
                                                </div>
                                                {field.field_type === AvoRedFieldTypesEnum.SELECT ? (
                                                    <div className="mt-3">
                                                        <div className="w-full">
                                                            <h6 className="font-semibold">
                                                                {t("pages.component.options")}
                                                            </h6>
                                                        </div>
                                                        <div className="flex">
                                                            <div className="w-1/2">
                                                                <InputField
                                                                    label={t('pages.component.option_label')}
                                                                    placeholder={t('pages.component.option_label')}
                                                                />
                                                            </div>
                                                            <div
                                                                className="w-1/2 ml-3 w-full">

                                                                <label
                                                                    htmlFor="hs-inline-leading-pricing-select-label"
                                                                    className="text-sm text-gray-600"
                                                                >{t('pages.component.option_value')}
                                                                </label>
                                                                <div className="relative">
                                                                    <InputField
                                                                        placeholder={t('pages.component.option_value')}
                                                                    />
                                                                    <div
                                                                        onClick={(e: React.MouseEvent) => { alert("test")}}
                                                                        className="absolute inset-y-0 end-0 z-40 flex items-center text-gray-500"
                                                                    >

                                                                        <PlusIcon className="text-primary-500 w-6 h-6" />
                                                                    </div>
                                                                </div>


                                                            </div>
                                                        </div>
                                                    </div>
                                                ) : ''}


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

export default ComponentCreate