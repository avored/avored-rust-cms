import {Link} from "react-router-dom";
import {useRoleTable} from "./hooks/useRoleTable";
import _ from 'lodash';
import {useTranslation} from "react-i18next";
import {createColumnHelper, getCoreRowModel, useReactTable} from "@tanstack/react-table";
import {getFormattedDate} from "../../lib/common";
import IRoleModel from "../../types/admin-user/IRoleModel";
import AvoRedTable from "../../components/AvoRedTable";
import HasPermission from "../../components/HasPermission";

export default function RoleTable() {
    const [t] = useTranslation("global")
    const role_api_table_response = useRoleTable();
    const roles: Array<IRoleModel> = _.get(role_api_table_response, 'data.data.data', [])

    const columnHelper = createColumnHelper<IRoleModel>()
    const columns = [
        columnHelper.accessor('id', {
            cell: info => info.getValue(),
            header: t("common.id")
        }),
        columnHelper.accessor('name', {
            cell: info => info.getValue(),
            header: t("common.name")
        }),
        columnHelper.accessor('identifier', {
            cell: info => info.getValue(),
            header: t("common.identifier")
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
                    <HasPermission displayDenied={false} identifier="role_edit">
                        <Link
                            className="font-medium text-primary-600 hover:text-primary-800"
                            to={`/admin/role-edit/${info.row.original.id}`}
                        >
                            {t("common.edit")}
                        </Link>
                    </HasPermission>
                )
            },
            header: t("common.action"),
            enableHiding: false
        }),
    ]

    const table = useReactTable({
        data: roles,
        columns,
        getCoreRowModel: getCoreRowModel(),
        initialState: {
            columnVisibility: {
                created_at: false,
                created_by: false
            }
        }
    })

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 ml-64">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        {t("roles.roles")}
                    </div>
                    <HasPermission displayDenied={false} identifier="role_create">
                    <Link
                        className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                        to="/admin/role-create"
                    >
                        {t("common.create")}
                    </Link>
                    </HasPermission>
                </div>

                <div className="w-full block overflow-hidden">
                    <div className="overflow-x-scroll">
                        <HasPermission identifier="role_table">
                            <AvoRedTable table={table}/>
                        </HasPermission>
                    </div>
                </div>
            </div>
        </div>
    );
}
