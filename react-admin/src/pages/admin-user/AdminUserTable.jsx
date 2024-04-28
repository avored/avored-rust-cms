import {Link} from "react-router-dom"
import _ from "lodash"
import {useAdminUserTable} from "./hooks/useAdminUserTable"

function AdminUserTable() {

    const getFormattedDate = ((date) => {
        let dateObject = new Date(date);
        return `${dateObject.getFullYear()}-${dateObject.getMonth()}-${dateObject.getDate()}`;
    })
    const adminUserTableResponse = useAdminUserTable();
    const adminUsers = _.get(adminUserTableResponse, 'data.data.data', [])

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 ml-64">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        Admin Users
                    </div>
                    <Link
                        className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                        to="/admin/admin-user-create">
                        Create
                    </Link>
                </div>

                <div className="overflow-x-auto">
                    <table className="w-full bg-white shadow-md rounded">
                        <thead>
                        <tr className="bg-gray-700 text-white">
                            <th className="py-3 px-4 rounded-l font-semibold text-left">ID</th>
                            <th className="py-3 px-4 font-semibol text-left">FullName</th>
                            <th className="py-3 px-4 font-semibol text-left">Email</th>
                            <th className="py-3 px-4 font-semibol text-left">Created at</th>
                            <th className="py-3 px-4 font-semibol text-left">Updated at</th>
                            <th className="py-3 px-4 font-semibol text-left">Created by</th>
                            <th className="py-3 px-4 font-semibol text-left">Updated by</th>
                            <th className="py-3 px-4 rounded-r font-semibol text-left">Action</th>
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
                                        <Link className="font-medium text-primary-600 hover:text-primary-800"
                                              to={`/admin/admin-user-edit/${adminUser.id}`}>
                                            Edit
                                        </Link>

                                    </td>
                                </tr>
                            )
                        })}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    )
}

export default AdminUserTable