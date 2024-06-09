import {useState} from "react";
import {Link} from "react-router-dom";
import {PlusIcon} from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";
import InputField from "../../components/InputField";
import _ from "lodash";
import {useComponentAll} from "./hooks/useComponentAll";
import {useStorePage} from "./hooks/useStorePage";
import {useTranslation} from "react-i18next";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {PageCreateSchema} from "./schemas/page.create.schema"
import PageComponentTable from "./PageComponentTable";

function PageCreate() {
    const [isComponentTableModalOpen, setIsComponentTableModalOpen] =
        useState(false);
    const [pageComponents, setPageComponents] = useState([]);
    const [page, setPage] = useState({});
    const [t] = useTranslation("global");

    const {
        control,
        register,
        handleSubmit,
        formState: {errors},
        setValue,
        getValues
    } = useForm({
        resolver: joiResolver(PageCreateSchema, {allowUnknown: true}),
    });

    const component_all_api_response = useComponentAll();
    const components = _.get(component_all_api_response, "data.data", []);

    const {mutate} = useStorePage();

    const getFormattedDate = (date) => {
        var date_obj = new Date(date);
        return `${date_obj.getFullYear()}-${
            date_obj.getMonth() + 1
        }-${date_obj.getDate()}`;
    };

    const renderComponentFieldType = (componentField) => {
        switch (componentField.field_type) {
            case "textarea":
                return <div>Textarea</div>;
            case "radio":
                return (
                    <div className="p-3">
                        <input id={componentField.identifier} type="radio" value={componentField.identifier}
                               className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-primary-500 dark:focus:ring-primary-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"/>
                        <label htmlFor={componentField.identifier}
                               className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300">
                            {componentField.name}
                        </label>
                    </div>
                );
            case "text":
            default:
                return (
                    <div>
                        <InputField
                            label={componentField.name}
                            type="text"
                            name={componentField.identifier}
                            onChange={(e) =>
                                componentFieldContentOnChange(componentField.id, e.target.value)
                            }
                        />
                    </div>
                );
        }
    };

    const renderComponentField = (componentField) => {
        return (
            <div className="ring-1 my-2 ring-gray-200" key={componentField.id}>
                {renderComponentFieldType(componentField)}
            </div>
        );
    };

    const componentSelected = (e, componentId) => {
        e.preventDefault();
        const selectedComponent = components.find(
            (component) => component.id === componentId
        );

        pageAddComponentSelected(selectedComponent);
        setIsComponentTableModalOpen(false);

        setPageComponents((pageComponents) => [
            ...pageComponents,
            selectedComponent,
        ]);
    };

    const addComponentOnClick = () => {
        setIsComponentTableModalOpen(true);
    };

    const pageModelOnClose = () => {
        setIsComponentTableModalOpen(false);
    };

    const pageNameOnChange = (value) => {
        setPage({
            ...page,
            name: value,
        });
    };
    const pageIdentifierOnChange = (value) => {
        setPage({
            ...page,
            identifier: value,
        });
    };
    const pageAddComponentSelected = (component) => {
        var componentContent = {};
        componentContent.id = component.id;
        componentContent.name = component.name;
        componentContent.identifier = component.identifier;
        componentContent.component_fields_content = [];

        component.fields.forEach((field) => {
            var componentFieldContent = {};

            componentFieldContent.id = field.id;
            componentFieldContent.name = field.name;
            componentFieldContent.identifier = field.identifier;
            componentFieldContent.field_type = field.field_type;
            componentFieldContent.field_content = "";

            componentContent.component_fields_content.push(componentFieldContent);
        });

        if (_.isEmpty(_.get(page, "components_content"))) {
            page["components_content"] = [];
        }
        page.components_content.push(componentContent);
    };

    const componentFieldContentOnChange = (componentFieldId, value) => {
        page.components_content.forEach((componentContent) => {
            componentContent.component_fields_content.forEach(
                // Removed variable "componentField"
                (componentFieldContent) => {
                    if (componentFieldContent.id === componentFieldId) {
                        componentFieldContent.field_content = value;
                    }
                }
            );
        });
        const updatedComponentContent = page.components_content;

        setPage({
            ...page,
            components_content: updatedComponentContent,
        });
    };

    const renderComponent = (pageComponent) => {
        return (
            <div
                key={pageComponent.id}
                className="my-5 ring-1 ring-gray-200 rounded p-3"
            >
                <div>component name: {pageComponent.name}</div>
                <div>component identifier: {pageComponent.identifier}</div>
                <div>
                    Component Fields
                    {pageComponent.fields.map((componentField) => {
                        return renderComponentField(componentField);
                    })}
                </div>
            </div>
        );
    };

    const submitHandler = async (data) => {
        // e.preventDefault();
        console.log(data)
        // mutate(page);
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
                                    autofocus={true}
                                    placeholder="Name"
                                    register={register("name")}
                                    className="border p-2 rounded w-full"
                                />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    type="text"
                                    placeholder="Identifier"
                                    register={register("identifier")}
                                    className="border p-2 rounded w-full"
                                />
                            </div>

                            <div>
                                {pageComponents.map((pageComponent) => {
                                    return renderComponent(pageComponent);
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
                                        <PageComponentTable components={components} componentSelected={componentSelected} />
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

export default PageCreate;
