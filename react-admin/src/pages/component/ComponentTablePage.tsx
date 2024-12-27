import {Link} from "react-router-dom";
import {useComponentTable} from "./hooks/useComponentTable";
import _ from "lodash";
import {useTranslation} from "react-i18next";
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table";
import IPageModel from "../../types/page/IPageModel";
import HasPermission from "../../components/HasPermission";
import AvoRedTable from "../../components/AvoRedTable";
import {useState} from "react";
import { getFormattedDate } from "../../lib/common";

export const ComponentTablePage = (() => {
    const [sorting, setSorting] = useState<SortingState>([]);
    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });
    const component_api_table_response = useComponentTable({
        order: sorting.map((s) => `${s.id}:${s.desc ? 'DESC' : 'ASC'}`).join(','),
        page: pagination.pageIndex
    })
    const customPagination = (async (pagination: any) => {
        setPagination(pagination)
    })

    const components = _.get(component_api_table_response, 'data.data.data', [])
    const [t] = useTranslation("global")

    const customSorting = ((sorting: any) => {
        setSorting(sorting)
    })

    const columnHelper = createColumnHelper<IPageModel>()
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
                    <HasPermission displayDenied={false} identifier="component_edit">
                        <Link
                            className="font-medium text-primary-600 hover:text-primary-800"
                            to={`/admin/component-edit/${info.row.original.id}`}
                        >
                            {t("edit")}
                        </Link>
                    </HasPermission>
                )
            },
            header: t("action"),
            enableSorting: false,
            enableHiding: false
        }),
    ]

    const table = useReactTable({
        data: components,
        columns,
        getCoreRowModel: getCoreRowModel(),
        rowCount: component_api_table_response.data?.data.pagination.total,
        onPaginationChange: customPagination,
        manualPagination: true,

        initialState: {
            columnVisibility: {
                created_at: false,
                created_by: false
            },
            pagination
        },
        manualSorting: true,
        onSortingChange: customSorting,
        state: {
            sorting,
            pagination
        },
    })

    return (
        <div className="px-5">
            <div className="flex items-center">
                <div className="p-5 text-2xl font-semibold text-primary-500">
                    {t("components")}
                </div>
                <HasPermission displayDenied={false} identifier="component_create">
                    <Link
                        className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                        to="/admin/component-create"
                    >
                        {t("create")}
                    </Link>
                </HasPermission>
            </div>

            <div className="overflow-x-hidden">
                <HasPermission identifier="component_table">
                    <AvoRedTable table={table}/>
                </HasPermission>
            </div>
        </div>
    );
})
