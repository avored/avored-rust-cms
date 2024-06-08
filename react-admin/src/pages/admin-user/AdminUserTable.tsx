import {Link} from "react-router-dom"
import _ from "lodash"
import {useAdminUserTable} from "./hooks/useAdminUserTable"
import {useTranslation} from "react-i18next"
import IAdminUserModel from "../../types/admin-user/IAdminUserModel"
import {createColumnHelper, flexRender, getCoreRowModel, TableState, useReactTable} from "@tanstack/react-table"
import {getFormattedDate} from "../../lib/common";
import AvoRedTable from "../../components/AvoRedTable"
import IRoleModel from "../../types/admin-user/IRoleModel";

function AdminUserTable() {

    const adminUserTableResponse = useAdminUserTable();
    const adminUsers: Array<IAdminUserModel> = _.get(adminUserTableResponse, 'data.data.data', [])

    const [t] = useTranslation("global");

    const columnHelper = createColumnHelper<IAdminUserModel>()
    const columns = [
        columnHelper.accessor('id', {
            cell: info => info.getValue(),
            header: t("common.id")
        }),
        columnHelper.accessor('full_name', {
            cell: info => info.getValue(),
            header: t("common.full_name")
        }),
        columnHelper.accessor('email', {
            cell: info => info.getValue(),
            header: t("common.email")
        }),
        columnHelper.accessor('is_super_admin', {
            cell: info => info.getValue(),
            header: t("common.is_super_admin"),
        }),
        columnHelper.accessor('roles', {
            cell: info => getRoleNames(info.getValue()),
            header: t("common.role")
        }),
        columnHelper.accessor('created_at', {
            id: "created_at",
            cell: info => getFormattedDate(info.getValue()),
            header: t("common.created_at")
        }),
        columnHelper.accessor('created_by', {
            cell: info => info.getValue(),
            header: t("common.created_by")
        }),
        columnHelper.accessor('updated_at', {
            cell: info => getFormattedDate(info.getValue()),
            header: t("common.updated_at")
        }),
        columnHelper.accessor('updated_by', {
            cell: info => info.getValue(),
            header: t("common.updated_by")
        }),
        columnHelper.accessor('action', {
            cell: info => {
                return (
                    <Link
                        className="font-medium text-primary-600 hover:text-primary-800"
                        to={`/admin/admin-user-edit/${info.row.original.id}`}
                       >
                    {t("common.edit")}
                    </Link>
                )
            },
            header: t("common.action"),
            enableHiding: false
        }),
    ]

    const table = useReactTable({
        data: adminUsers,
        columns,
        getCoreRowModel: getCoreRowModel(),
        initialState: {
            columnVisibility: {
                created_at: false,
                created_by: false,
                is_super_admin: false
            }
        }
    })

    const getRoleNames = ((roles: Array<IRoleModel>) => {
        return roles.map((role) => {
            return (
                <>
                    <span className="bg-gray-300 p-1 rounded mr-1">{role.name}</span>
                </>
            )
        })
    })

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
                <div className="w-full block overflow-hidden">
                    <div className="overflow-x-scroll">
                        <AvoRedTable table={table} />
                    </div>
                </div>
            </div>
        </div>
    );
}

export default AdminUserTable