import {Link} from "react-router-dom"
import _ from "lodash"
import {useAdminUserTable} from "./hooks/useAdminUserTable"
import { useTranslation } from "react-i18next"

function AdminUserTable() {

    const getFormattedDate = ((date) => {
        let dateObject = new Date(date);
        return `${dateObject.getFullYear()}-${dateObject.getMonth()}-${dateObject.getDate()}`;
    })
    const adminUserTableResponse = useAdminUserTable();
    const adminUsers = _.get(adminUserTableResponse, 'data.data.data', [])
    const [t] = useTranslation("global");

    return (
      <div className="flex-1 bg-white">
        <div className="px-5 ml-64">
          <div className="flex items-center">
            <div className="p-5 text-2xl font-semibold text-primary-500">
              {t("admin_user.admin_users")}
            </div>
            <Link
              className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              to="/admin/admin-user-create"
            >
              {t("common.create")}
            </Link>
          </div>

          <div className="overflow-x-auto">
            <table className="w-full bg-white shadow-md rounded">
              <thead>
                <tr className="bg-gray-700 text-white">
                  <th className="py-3 px-4 rounded-l font-semibold text-left">
                    {t("common.id")}
                  </th>
                  <th className="py-3 px-4 font-semibol text-left">
                    {t("common.fulname")}
                  </th>
                  <th className="py-3 px-4 font-semibol text-left">
                    {t("common.email")}
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
                {adminUsers.map((adminUser) => {
                  return (
                    <tr key={adminUser.id} className="border-b">
                      <td className="py-3 px-4">{adminUser.id}</td>
                      <td className="py-3 px-4">{adminUser.full_name}</td>
                      <td className="py-3 px-4">{adminUser.email}</td>
                      <td className="py-3 px-4">
                        {getFormattedDate(adminUser.created_at)}
                      </td>
                      <td className="py-3 px-4">
                        {getFormattedDate(adminUser.updated_at)}
                      </td>
                      <td className="py-3 px-4">{adminUser.created_by}</td>
                      <td className="py-3 px-4">{adminUser.updated_by}</td>
                      <td className="py-3 px-4">
                        <Link
                          className="font-medium text-primary-600 hover:text-primary-800"
                          to={`/admin/admin-user-edit/${adminUser.id}`}
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

export default AdminUserTable