import {Link} from "react-router-dom"
import _ from "lodash"
import {useAdminUserTable} from "./hooks/useAdminUserTable"
import {useTranslation} from "react-i18next"
import IAdminUserModel from "../../types/admin-user/IAdminUserModel"
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table"
import {getFormattedDate} from "../../lib/common";
import AvoRedTable from "../../components/AvoRedTable"
import IRoleModel from "../../types/admin-user/IRoleModel";
import HasPermission from "../../components/HasPermission"
import {useState} from "react";
import {useQueryClient} from "@tanstack/react-query";

function AdminUserTable() {
    const queryClient = useQueryClient()
    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });
    const [sorting, setSorting] = useState<SortingState>([]);
    const adminUserTableResponse = useAdminUserTable({
        order: sorting.map((s) => `${s.id}:${s.desc ? 'DESC' : 'ASC'}`).join(','),
        page: pagination.pageIndex
    })

    const customSorting = (async (sorting: any) => {
        setSorting(sorting)
    })
    const customPagination = (async (pagination: any) => {
        setPagination(pagination)
    })
    const adminUsers: Array<IAdminUserModel> = _.get(adminUserTableResponse, 'data.data.data', [])
    const [t] = useTranslation("global");

    const columnHelper = createColumnHelper<IAdminUserModel>()
    const columns = [
        columnHelper.accessor('id', {
            cell: info => info.getValue(),
            header: t("id")
        }),
        columnHelper.accessor('full_name', {
            cell: info => info.getValue(),
            header: t("full_name")
        }),
        columnHelper.accessor('email', {
            cell: info => info.getValue(),
            header: t("email"),
        }),
        columnHelper.accessor('is_super_admin', {
            cell: info => info.getValue(),
            header: t("is_super_admin"),
            enableSorting: false
        }),
        columnHelper.accessor('roles', {
            cell: info => getRoleNames(info.getValue() ?? []),
            header: t("role"),
            enableSorting: false,
        }),
        columnHelper.accessor('created_at', {
            id: "created_at",
            cell: info => getFormattedDate(info.getValue()),
            header: t("created_at")
        }),
        columnHelper.accessor('created_by', {
            cell: info => info.getValue(),
            header: t("created_by")
        }),
        columnHelper.accessor('updated_at', {
            cell: info => getFormattedDate(info.getValue()),
            header: t("updated_at")
        }),
        columnHelper.accessor('updated_by', {
            cell: info => info.getValue(),
            header: t("updated_by")
        }),
        columnHelper.accessor('action', {
            cell: info => {
                return (
                    <Link
                        className="font-medium text-primary-600 hover:text-primary-800"
                        to={`/admin/admin-user-edit/${info.row.original.id}`}
                    >
                        {t("edit")}
                    </Link>
                )
            },
            header: t("action"),
            enableHiding: false,
            enableSorting: false
        }),
    ]

    const table = useReactTable({
        data: adminUsers,
        columns,
        getCoreRowModel: getCoreRowModel(),
        manualSorting: true,
        onSortingChange: customSorting,
        onPaginationChange: customPagination,
        manualPagination: true,
        state: {
            sorting,
            pagination
        },
        rowCount: adminUserTableResponse.data?.data.pagination.total,
        initialState: {
            columnVisibility: {
                created_at: false,
                created_by: false,
                is_super_admin: false
            },
            pagination
        }
    })

    const getRoleNames = ((roles: Array<IRoleModel>) => {
        if (roles.length === 0) {
            return (<></>)
        }
        return roles.map((role) => {
            return (
                <span key={role.id} className="bg-gray-300 p-1 rounded mr-1">
                    {role.name}
                </span>
            )
        })
    })

    return (
        <>
            <div className="p-5">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        {t("admin_users")}
                    </div>
                    <HasPermission displayDenied={false} identifier="admin_user_create">
                        <Link
                            className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                            to="/admin/admin-user-create"
                        >
                            {t("create")}
                        </Link>
                    </HasPermission>
                </div>
                <div className="w-full block overflow-hidden">
                    <div className="overflow-x-scroll">
                        <HasPermission identifier="admin_user_table">
                            <AvoRedTable table={table}/>
                        </HasPermission>
                    </div>
                </div>
            </div>
        </>

    );
}

export default AdminUserTable
