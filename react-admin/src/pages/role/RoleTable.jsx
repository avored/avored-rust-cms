import {Link} from "react-router-dom";
import {useRoleTable} from "./hooks/useRoleTable";
import _ from 'lodash';

export default function RoleTable() {

    const getFormattedDate = ((date) => {
        const date_obj = new Date(date);

        return `${date_obj.getFullYear()}-${("0" + (date_obj.getMonth() + 1)).slice("-2")}-${("0" + (date_obj.getDate())).slice("-2")}`;
    })

    const role_api_table_response = useRoleTable()
    const roles = _.get(role_api_table_response, 'data.data.data', [])

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 ml-64">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        Roles
                    </div>
                    <Link className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                        to="/admin/role-create">
                        Create
                    </Link>
                </div>

                <div className="overflow-x-hidden">
                    <table className="min-w-full bg-white shadow-md rounded">
                        <thead>
                        <tr className="bg-gray-700 text-white">
                            <th className="py-3 px-4 rounded-l font-semibold text-left">ID</th>
                            <th className="py-3 px-4 font-semibol text-left">Name</th>
                            <th className="py-3 px-4 font-semibol text-left">Identifier</th>
                            <th className="py-3 px-4 font-semibol text-left">Created at</th>
                            <th className="py-3 px-4 font-semibol text-left">Updated at</th>
                            <th className="py-3 px-4 font-semibol text-left">Created by</th>
                            <th className="py-3 px-4 font-semibol text-left">Updated by</th>
                            <th className="py-3 px-4 rounded-r font-semibol text-left">Action</th>
                        </tr>
                        </thead>
                        <tbody className="">
                        {roles.map((role) => {
                            return (
                                <tr key={role.id} className="border-b">
                                    <td className="py-3 px-4">{role.id}</td>
                                    <td className="py-3 px-4">{role.name}</td>
                                    <td className="py-3 px-4">{role.identifier}</td>
                                    <td className="py-3 px-4">
                                        {getFormattedDate(role.created_at)}
                                    </td>
                                    <td className="py-3 px-4">
                                        {getFormattedDate(role.updated_at)}
                                    </td>
                                    <td className="py-3 px-4">{role.created_by}</td>
                                    <td className="py-3 px-4">{role.updated_by}</td>
                                    <td className="py-3 px-4">
                                        <Link className="font-medium text-primary-600 hover:text-primary-800"
                                              to={`/admin/role-edit/${role.id}`}>
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