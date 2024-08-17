import React, { useState } from "react";
import { Link } from "react-router-dom";
import { PlusIcon } from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";
import InputField from "../../components/InputField";
import _ from "lodash";
import { useComponentAll } from "./hooks/useComponentAll";
import { useStorePage } from "./hooks/useStorePage";
import { useTranslation } from "react-i18next";
import { useFieldArray, useForm } from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import { usePageCreateSchema } from "./schemas/page.create.schema";
import PageComponentTable from "./PageComponentTable";
import IComponentModel from "../../types/component/IComponentModel";
import ICreatablePage, {
    ICreatablePageComponentElementModel,
    ICreatablePageComponentModel,
} from "../../types/page/ICreatablePage";
import ErrorMessage from "../../components/ErrorMessage";
import AvoRedMultiSelectField from "../../components/AvoRedMultiSelectField";
import slug from 'slug'

function PageCreate() {
    const [isComponentTableModalOpen, setIsComponentTableModalOpen] =
        useState(false);
    const [t] = useTranslation("global");

    const {
        control,
        register,
        handleSubmit,
        formState: { errors },
        setValue,
        getValues,
        getFieldState,
        watch
    } = useForm<ICreatablePage>({
        resolver: joiResolver(usePageCreateSchema(), { allowUnknown: true }),
    });

    const { fields: components_content, append } = useFieldArray({
        control,
        name: "components_content", //rename fields
    });

    const setSelectedElementDataOption = ((selected: Array<string>, pageComponentIndex: number, componentElementIndex: number) => {
        let val = selected.pop() ?? ''

        // @todo fix this onese
        // Currently all data types is text but will be depend on element_type???
        setValue(`components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_data_type`, 'TEXT')
        setValue(`components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`, val)
    })

    const getSelectElementDataOptionCurrentValue = ((pageComponentIndex: number, componentElementIndex: number) => {

        let val  = getValues(`components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`)
        if (val) {
            return [val]
        }
        return []
    })

    const component_all_api_response = useComponentAll();
    const components = _.get(component_all_api_response, "data.data", []);
    const { mutate, error } = useStorePage();

    const renderComponentElementType = (
        componentElement: ICreatablePageComponentElementModel,
        pageComponentIndex: number,
        componentElementIndex: number,
    ) => {
        switch (componentElement.element_type) {
            case "textarea":
                return (
                    <div className="p-3">
            <textarea
                {...register(
                    `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`,
                )}
            ></textarea>
                        <label
                            htmlFor={componentElement.identifier}
                            className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                        >
                            {componentElement.name}
                        </label>
                    </div>
                );
            case "select":
                return (
                    <div className="p-3">
                        <AvoRedMultiSelectField
                            label="test dropdown"
                            selectedOption={getSelectElementDataOptionCurrentValue(pageComponentIndex, componentElementIndex)}
                            options={componentElement.element_data ?? []}
                            onChangeSelectedOption={((val: Array<string>) => setSelectedElementDataOption(val, pageComponentIndex, componentElementIndex))}
                        ></AvoRedMultiSelectField>
                        <label
                            htmlFor={componentElement.identifier}
                            className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                        >
                            {componentElement.name}
                        </label>
                    </div>
                );
            case "radio":
                return (
                    <div className="p-3">
                        <input
                            id={componentElement.identifier}
                            type="radio"
                            value={componentElement.identifier}
                            className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-primary-500 dark:focus:ring-primary-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                        />
                        <label
                            htmlFor={componentElement.identifier}
                            className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                        >
                            {componentElement.name}
                        </label>
                    </div>
                );
            case "text":
            default:
                return (
                    <div>
                        <InputField
                            name={`components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`}
                            label={componentElement.name}
                            register={register(
                                `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`,
                            )}
                        />
                        <InputField
                            type="hidden"
                            value="TEXT"
                            register={register(
                                `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_data_type`,
                            )}
                        />
                    </div>
                );
        }
    };

    const renderComponentElement = (
        componentElement: ICreatablePageComponentElementModel,
        pageComponentIndex: number,
        componentElementIndex: number,
    ) => {
        return (
            <div className="ring-1 my-2 ring-gray-200" key={componentElement.identifier}>
                {JSON.stringify(componentElement)}
                {renderComponentElementType(
                    componentElement,
                    pageComponentIndex,
                    componentElementIndex,
                )}
            </div>
        );
    };

    const componentSelected = (e: React.MouseEvent, componentId: string) => {
        e.preventDefault();
        const selectedComponent = components.find(
            (component: IComponentModel) => component.id === componentId,
        );
        setIsComponentTableModalOpen(false);

        append(selectedComponent);
    };

    const addComponentOnClick = () => {
        setIsComponentTableModalOpen(true);
    };

    const pageModelOnClose = () => {
        setIsComponentTableModalOpen(false);
    };

    const renderComponent = (
        pageComponent: ICreatablePageComponentModel,
        pageComponentIndex: number,
    ) => {
        return (
            <div
                key={pageComponent.id}
                className="my-5 ring-1 ring-gray-200 rounded p-3"
            >
                <div>component name: {pageComponent.name}</div>
                <div>component identifier: {pageComponent.identifier}</div>
                <div>
                    Component Fields
                    {pageComponent.elements.map((componentElement, componentElementIndex) => {
                        return renderComponentElement(
                            componentElement,
                            pageComponentIndex,
                            componentElementIndex,
                        );
                    })}
                </div>
            </div>
        );
    };

    const submitHandler = async (data: ICreatablePage) => {
        // console.log(data)
        mutate(data);
    };

    const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
        setValue('identifier', slug(e.currentTarget.value || ''))
    }

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64 ">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("page_information")}
                        </h1>

                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    autoFocus={true}
                                    placeholder={t("name")}
                                    register={register("name")}
                                    onKeyUp={e => onNameChange(e)}
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
                                    register={register("identifier")}
                                />
                                <ErrorMessage
                                    frontendErrors={errors}
                                    backendErrors={error}
                                    identifier="identifier"
                                />
                            </div>

                            <div>
                                {components_content.map((pageComponent, pageComponentIndex) => {
                                    return renderComponent(pageComponent, pageComponentIndex);
                                })}
                            </div>

                            <div className="mb-4 flex items-center justify-center ring-1 ring-gray-400 rounded p-5">
                                <button
                                    type="button"
                                    className="flex"
                                    onClick={addComponentOnClick}
                                >
                                    <PlusIcon className="text-primary-500 h-6 w-6"/>
                                    <span className="text-sm ml-1 text-primary-500">
                                        {t("add_component")}
                                    </span>
                                </button>
                            </div>

                            <AvoredModal
                                closeModal={pageModelOnClose}
                                modal_header={t("select_component")}
                                modal_body={
                                    <div className="text-primary-500">
                                        <PageComponentTable
                                            components={components}
                                            componentSelected={componentSelected}
                                        />
                                    </div>
                                }
                                isOpen={isComponentTableModalOpen}
                            ></AvoredModal>

                            <div className="flex items-center">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("save")}
                                </button>
                                <Link
                                    to="/admin/page"
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
}

export default PageCreate;
