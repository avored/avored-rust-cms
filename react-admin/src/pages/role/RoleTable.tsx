import {Link} from "react-router-dom";
import {useRoleTable} from "./hooks/useRoleTable";
import _ from 'lodash';
import {useTranslation} from "react-i18next";
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table";
import {getFormattedDate} from "../../lib/common";
import IRoleModel from "../../types/admin-user/IRoleModel";
import AvoRedTable from "../../components/AvoRedTable";
import HasPermission from "../../components/HasPermission";
import {useQueryClient} from "@tanstack/react-query";
import {useState} from "react";

export default function RoleTable() {
    const queryClient = useQueryClient()
    const [sorting, setSorting] = useState<SortingState>([]);
    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });
    const [t] = useTranslation("global")
    const role_api_table_response = useRoleTable({
        order: sorting.map((s) => `${s.id}:${s.desc ? 'DESC' : 'ASC'}`).join(','),
        page: pagination.pageIndex
    });
    const customPagination = (async (pagination: any) => {
        setPagination(pagination)
    })
    const roles: Array<IRoleModel> = _.get(role_api_table_response, 'data.data.data', [])

    const customSorting = ((sorting: any) => {
        setSorting(sorting)
    })

    const columnHelper = createColumnHelper<IRoleModel>()
    const columns = [
        columnHelper.accessor('id', {
            cell: info => info.getValue(),
            header: t("id")
        }),
        columnHelper.accessor('name', {
            cell: info => info.getValue(),
            header: t("name")
        }),
        columnHelper.accessor('identifier', {
            cell: info => info.getValue(),
            header: t("identifier")
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
                    <HasPermission displayDenied={false} identifier="role_edit">
                        <Link
                            className="font-medium text-primary-600 hover:text-primary-800"
                            to={`/admin/role-edit/${info.row.original.id}`}
                        >
                            {t("edit")}
                        </Link>
                    </HasPermission>
                )
            },
            enableSorting: false,
            header: t("action"),
            enableHiding: false
        }),
    ]

    const table = useReactTable({
        data: roles,
        columns,
        getCoreRowModel: getCoreRowModel(),
        rowCount: role_api_table_response.data?.data.pagination.total,
        onPaginationChange: customPagination,
        manualPagination: true,
        initialState: {
            pagination,
            columnVisibility: {
                created_at: false,
                created_by: false
            }
        },
        manualSorting: true,
        onSortingChange: customSorting,
        state: {
            sorting,
            pagination
        },
    })

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 ml-64">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        {t("roles")}
                    </div>
                    <HasPermission displayDenied={false} identifier="role_create">
                    <Link
                        className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                        to="/admin/role-create"
                    >
                        {t("create")}
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
