import {Link} from "react-router-dom";
import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import {Controller, useFieldArray, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import ErrorMessage from "../../components/ErrorMessage";
import {
    CollectionFieldDataType, CollectionFieldFieldType,
    SavableCollectionType
} from "../../types/collection/CreatableCollectionType";
import {useStoreCollection} from "./hooks/useStoreCollection";
import {useCollectionCreateSchema} from "./schemas/CollectionCreateSchema";
import React, {useState} from "react";
import slug from "slug";
import AvoRedButton, {ButtonType} from "../../components/AvoRedButton";
import _ from "lodash";
import {CollectionFieldModal} from "./CollectionFieldModal";
import {Cog8ToothIcon, TrashIcon} from "@heroicons/react/24/solid";

export const CollectionCreate = () => {
    const [t] = useTranslation("global")
    const {mutate, error} = useStoreCollection()
    const [isCollectionFieldModalOpen, setIsCollectionFieldModalOpen] = useState<boolean>(false);
    const [currentIndex, setCurrentIndex] = useState<number>(0);

    const {
        register,
        handleSubmit,
        setValue,
        formState: {errors},
        getValues,
        control,
        trigger
    } = useForm<SavableCollectionType>({
        resolver: joiResolver(useCollectionCreateSchema(), {allowUnknown: true}),
    })

    const { fields, append, remove } = useFieldArray({
        control,
        name: "collection_fields", //rename fields
    });

    const submitHandler = ((data: SavableCollectionType) => {
        // console.log(data)
        mutate(data)
    })

    const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
        setValue('identifier', slug(e.currentTarget.value || ''))
    }

    const deleteCollectionFieldOnClick = (e: any, index: number) => {
        e.preventDefault();
        remove(index);
        setCurrentIndex(0);
    };

    const addFieldButtonOnClick = (async (e: React.MouseEvent<HTMLButtonElement>, max_index: number) => {
        e.preventDefault()
        append({
            name: '',
            identifier: '',
            data_type: CollectionFieldDataType.TEXT,
            field_type: CollectionFieldFieldType.TEXT,
        })
        await trigger("collection_fields");
        setCurrentIndex(max_index);
        setIsCollectionFieldModalOpen(true)
    })


    return (
        <>
            <div className="px-5">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("collection_information")}
                        </h1>

                        <form onSubmit={handleSubmit(submitHandler)}>

                            {_.size(fields) > 0 ? (
                                <CollectionFieldModal
                                    register={register}
                                    currentIndex={currentIndex}
                                    getValues={getValues}
                                    setValue={setValue}
                                    trigger={trigger}
                                    setIsOpen={setIsCollectionFieldModalOpen}
                                    isOpen={isCollectionFieldModalOpen}                           />
                            ) : (
                                <></>
                            )}

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

                            {fields.map((field, index) => {
                                return (
                                    <div
                                        key={field.id}
                                        className="hover:ring-1 ring-primary-300 rounded mb-5 flex mt-5 py-3 w-full"
                                    >
                                        <Controller
                                            name={`collection_fields.${index}`}
                                            render={({field: collection_field}) => {
                                                return (
                                                    <>
                                                        <div className="flex mt-3 w-full justify-center">
                                                            <div className="flex-1 p-3">
                                                                <div className="p-3 bg-gray-200 rounded">
                                                                    <div
                                                                        className="flex text-sm w-full border-gray-300 border-b py-2">
                                                                        <div className="flex-1">
                                                                                <span>
                                                                                    {collection_field.value.name}
                                                                                </span>
                                                                            <span
                                                                                className="ml-1 text-xs text-gray-500">
                                                                                    ({collection_field.value.identifier})
                                                                                </span>
                                                                        </div>
                                                                        <div className="ml-auto flex items-center">
                                                                            <div>
                                                                                <button
                                                                                    type="button"
                                                                                    className="outline-none"
                                                                                    onClick={() => setIsCollectionFieldModalOpen(true)}
                                                                                >
                                                                                    <Cog8ToothIcon className="w-5 h-5"/>
                                                                                </button>
                                                                            </div>
                                                                            <div
                                                                                onClick={(e) =>
                                                                                    deleteCollectionFieldOnClick(e, index)
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
                                                                            `collection_fields.${index}.data_type`,
                                                                        )}
                                                                    />
                                                                    <InputField
                                                                        type="hidden"
                                                                        placeholder={t("field_type")}
                                                                        register={register(
                                                                            `collection_fields.${index}.field_type`,
                                                                        )}
                                                                    />

                                                                </div>
                                                            </div>
                                                        </div>
                                                    </>
                                                );
                                            }}
                                            control={control}
                                        />
                                    </div>
                                );
                            })}


                            {/* todo uncomment below to enabled the add field button */}
                            {/*<div className="mb-4">*/}
                            {/*    <AvoRedButton*/}
                            {/*        label="Add"*/}
                            {/*        onClick={(e: React.MouseEvent<HTMLButtonElement>) => addFieldButtonOnClick(e, fields.length)}*/}
                            {/*        type={ButtonType.button}/>*/}
                            {/*</div>*/}

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
            </div>
        </>
    );
};
