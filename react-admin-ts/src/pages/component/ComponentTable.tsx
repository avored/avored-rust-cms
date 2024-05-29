import { Link } from "react-router-dom";
import { useComponentTable } from "./hooks/useComponentTable";
import _ from "lodash";
import { useTranslation } from "react-i18next";

interface Component {
  id: number;
  name: string;
  identifier: string;
  created_at: string;
  updated_at: string;
  created_by: string;
  updated_by: string;
}

interface ApiResponse {
  data: {
    data: {
      data: Component[];
    };
  };
}

function ComponentTable() {
  const component_api_table_response: ApiResponse | undefined =
    useComponentTable() as unknown as ApiResponse | undefined;
  const components: Component[] = _.get(
    component_api_table_response,
    "data.data.data",
    []
  ) as Component[];
  const [t] = useTranslation("global");

  const getFormattedDate = (date: string): string => {
    var date_obj = new Date(date);

    return `${date_obj.getFullYear()}-${
      date_obj.getMonth() + 1
    }-${date_obj.getDate()}`;
  };

  return (
    <div className="flex-1 bg-white">
      <div className="px-5 ml-64">
        <div className="flex items-center">
          <div className="p-5 text-2xl font-semibold text-primary-500">
            {t("component.components")}
          </div>
          <Link
            className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
            to="/admin/component-create"
          >
            {t("common.create")}
          </Link>
        </div>

        <div className="overflow-x-hidden">
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
                    <td className="py-3 px-4">{component.identifier}</td>
                    <td className="py-3 px-4">
                      {getFormattedDate(component.created_at)}
                    </td>
                    <td className="py-3 px-4">
                      {getFormattedDate(component.updated_at)}
                    </td>
                    <td className="py-3 px-4">{component.created_by}</td>
                    <td className="py-3 px-4">{component.updated_by}</td>
                    <td className="py-3 px-4">
                      <Link
                        className="font-medium text-primary-600 hover:text-primary-800"
                        to={`/admin/component-edit/${component.id}`}
                      >
                        {t("common.edit")}
                      </Link>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

export default ComponentTable;
