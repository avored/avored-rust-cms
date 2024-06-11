import React, {useState} from "react";
import {Link, useParams} from "react-router-dom";
import {PlusIcon} from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";
import InputField from "../../components/InputField";
import _ from "lodash";
import {useComponentAll} from "./hooks/useComponentAll";
import {useGetPage} from "./hooks/useGetPage";
import {useUpdatePage} from "./hooks/useUpdatePage";
import {useTranslation} from "react-i18next";
import {useFieldArray, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {PageCreateSchema} from "./schemas/page.create.schema";
import PageComponentTable from "./PageComponentTable";

import IEditablePage, {
    IEditablePageComponentFieldModel,
    IEditablePageComponentModel
} from "../../types/page/IEditablePage";

function PageEdit() {
    const [isComponentTableModalOpen, setIsComponentTableModalOpen] =
        useState(false);
    const [pageComponents, setPageComponents] = useState([]);
    const [page, setPage] = useState({});
    const params = useParams();
    const [t] = useTranslation("global");

    const component_all_api_response = useComponentAll();
    const components = _.get(component_all_api_response, "data.data", []);

    const {mutate} = useUpdatePage(params.page_id ?? '');

    const {data} = useGetPage(params.page_id ?? '');
    const values = data?.data.page_model


    const {
        control,
        register,
        handleSubmit,
        formState: {errors}
    } = useForm<IEditablePage>({
        resolver: joiResolver(PageCreateSchema, {allowUnknown: true}),
        values
    });

    const {
        fields: components_content,
        append
    } = useFieldArray({
        control,
        name: "components_content",
    });

    const renderComponentFieldType = (componentField: IEditablePageComponentFieldModel, pageComponentIndex: number, pageComponentFieldIndex: number) => {
        switch (componentField.field_type) {
            case "textarea":
                return <div>Textarea</div>;
            case "text":
            default:
                return (
                    <div>
                        <InputField
                            label={componentField.name}
                            type="text"
                            name={componentField.identifier}
                            register={register(`components_content.${pageComponentIndex}.fields.${pageComponentFieldIndex}.field_content`)}
                        />
                    </div>
                );
        }
    };

    const renderComponentField = (componentField: IEditablePageComponentFieldModel, pageComponentIndex: number, pageComponentFieldIndex: number) => {
        return (
            <div className="ring-1 my-2 ring-gray-200" key={componentField.id}>
                {renderComponentFieldType(componentField, pageComponentIndex, pageComponentFieldIndex)}
            </div>
        );
    };

    const componentSelected = (e: React.MouseEvent, componentId: string) => {
        e.preventDefault();
        const selectedComponent = components.find(
            (component: IEditablePageComponentModel) => component.id === componentId
        );
        setIsComponentTableModalOpen(false);

        append(selectedComponent)

    };

    const addComponentOnClick = () => {
        setIsComponentTableModalOpen(true);
    };

    const pageModelOnClose = () => {
        setIsComponentTableModalOpen(false);
    };


    const renderComponent = (pageComponent: IEditablePageComponentModel, pageComponentIndex: number) => {
        return (
            <div
                key={pageComponent.id}
                className="my-5 ring-1 ring-gray-200 rounded p-3"
            >
                <div>component name: {pageComponent.name}</div>
                <div>component identifier: {pageComponent.identifier}</div>
                <div>
                    Component Fields
                    {components_content.map((componentField, pageComponentFieldIndex) => {
                        return renderComponentField(componentField, pageComponentIndex, pageComponentFieldIndex);
                    })}
                </div>
            </div>
        );
    };

    const submitHandler = async (data: IEditablePage) => {
        mutate(data)
    };

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64 ">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("pages.information")}
                        </h1>
                        {/*<p className="text-gray-600 dark:text-gray-300 mb-6">Use a permanent address where you can*/}
                        {/*    receive mail.</p>*/}
                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    placeholder="Name"
                                    name="name"
                                    register={register('name')}
                                    className="border p-2 rounded w-full"
                                />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    placeholder="Identifier"
                                    name="identifier"
                                    register={register('identifier')}
                                    className="border p-2 rounded w-full"
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
                                        Add Component
                                      </span>
                                </button>
                            </div>

                            <AvoredModal
                                closeModal={pageModelOnClose}
                                modal_header="Select Component"
                                modal_body={
                                    <div className="text-primary-500">
                                        <PageComponentTable components={components}
                                                            componentSelected={componentSelected}/>
                                    </div>
                                }
                                isOpen={isComponentTableModalOpen}
                            ></AvoredModal>

                            <div className="flex items-center">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("common.save")}
                                </button>
                                <Link
                                    to="/admin/page"
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

export default PageEdit;
