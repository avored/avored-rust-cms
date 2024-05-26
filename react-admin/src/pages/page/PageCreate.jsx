import { useState } from "react";
import { Link } from "react-router-dom";
import { PlusIcon } from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";
import InputField from "../../components/InputField";
import _ from "lodash";
import { useComponentAll } from "./hooks/useComponentAll";
import { useStorePage } from "./hooks/useStorePage";
import { useTranslation } from "react-i18next";

function PageCreate() {
  const [isComponentTableModalOpen, setIsComponentTableModalOpen] =
    useState(false);
  const [pageComponents, setPageComponents] = useState([]);
  const [page, setPage] = useState({});
  const [t] = useTranslation("global");

  const component_all_api_response = useComponentAll();
  const components = _.get(component_all_api_response, "data.data", []);

  const { mutate } = useStorePage();

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

  const handleSubmit = async (e) => {
    e.preventDefault();

    mutate(page);
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
            <form onSubmit={handleSubmit}>
              <div className="mb-4">
                <input
                  type="text"
                  placeholder="Name"
                  value={_.get(page, "name", "")}
                  onChange={(e) => pageNameOnChange(e.target.value)}
                  className="border p-2 rounded w-full"
                />
              </div>
              <div className="mb-4">
                <input
                  type="text"
                  placeholder="Identifier"
                  value={_.get(page, "identifier", "")}
                  onChange={(e) => pageIdentifierOnChange(e.target.value)}
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
                  <PlusIcon className="text-primary-500 h-6 w-6" />
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
                    <table className="min-w-full bg-white shadow-md rounded">
                      <thead>
                        <tr className="bg-gray-700 text-white">
                          <th className="py-3 px-4 rounded-l font-semibold text-left">
                            {t("common.id")}
                          </th>
                          <th className="py-3 px-4 font-semibol text-left">
                            {t("common.name")}
                          </th>
                          <th className="py-3 px-4 font-semibol text-left">
                            {t("common.identifier")}
                          </th>
                          <th className="py-3 px-4 font-semibol text-left">
                            {t("common.created_at")}
                          </th>
                          <th className="py-3 px-4 font-semibol text-left">
                            {t("common.updated_at")}
                          </th>
                          <th className="py-3 px-4 font-semibol text-left">
                            {t("common.created_by")}
                          </th>
                          <th className="py-3 px-4 font-semibol text-left">
                            {t("common.updated_by")}
                          </th>
                          <th className="py-3 px-4 rounded-r font-semibol text-left">
                            {t("common.action")}
                          </th>
                        </tr>
                      </thead>
                      <tbody className="">
                        {components.map((component) => {
                          return (
                            <tr key={component.id} className="border-b">
                              <td className="py-3 px-4">{component.id}</td>
                              <td className="py-3 px-4">{component.name}</td>
                              <td className="py-3 px-4">
                                {component.identifier}
                              </td>
                              <td className="py-3 px-4">
                                {getFormattedDate(component.created_at)}
                              </td>
                              <td className="py-3 px-4">
                                {getFormattedDate(component.updated_at)}
                              </td>
                              <td className="py-3 px-4">
                                {component.created_by}
                              </td>
                              <td className="py-3 px-4">
                                {component.updated_by}
                              </td>
                              <td className="py-3 px-4">
                                <button
                                  type="button"
                                  className="font-medium text-primary-600 hover:text-primary-800"
                                  onClick={(e) =>
                                    componentSelected(e, component.id)
                                  }
                                >
                                  {t("common.select")}
                                </button>
                              </td>
                            </tr>
                          );
                        })}
                      </tbody>
                    </table>
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
