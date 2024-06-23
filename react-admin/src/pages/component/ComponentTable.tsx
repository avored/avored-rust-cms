import {Link} from "react-router-dom";
import {useComponentTable} from "./hooks/useComponentTable";
import _ from "lodash";
import {useTranslation} from "react-i18next";
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table";
import IPageModel from "../../types/page/IPageModel";
import HasPermission from "../../components/HasPermission";
import AvoRedTable from "../../components/AvoRedTable";
import {useQueryClient} from "@tanstack/react-query";
import {useState} from "react";

function ComponentTable() {
    const queryClient = useQueryClient()
    const [sorting, setSorting] = useState<SortingState>([]);
    const comoonent_api_table_response = useComponentTable({
        order: sorting.map((s) => `${s.id}:${s.desc ? 'DESC' : 'ASC'}`).join(','),
    })
    const components = _.get(comoonent_api_table_response, 'data.data.data', [])
    const [t] = useTranslation("global")

    const customSorting = ((sorting: any) => {
        queryClient.invalidateQueries( {queryKey: ['component-table']});
        setSorting(sorting)
    })

    const getFormattedDate = ((date: string) => {
        var date_obj = new Date(date);

        return `${date_obj.getFullYear()}-${date_obj.getMonth() + 1}-${date_obj.getDate()}`;
    })

    const columnHelper = createColumnHelper<IPageModel>()
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
                    <HasPermission displayDenied={false} identifier="component_edit">
                        <Link
                            className="font-medium text-primary-600 hover:text-primary-800"
                            to={`/admin/component-edit/${info.row.original.id}`}
                        >
                            {t("common.edit")}
                        </Link>
                    </HasPermission>
                )
            },
            header: t("common.action"),
            enableSorting: false,
            enableHiding: false
        }),
    ]

    const table = useReactTable({
        data: components,
        columns,
        getCoreRowModel: getCoreRowModel(),
        initialState: {
            columnVisibility: {
                created_at: false,
                created_by: false
            }
        },
        manualSorting: true,
        onSortingChange: customSorting,
        state: {
            sorting
        },
    })

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 ml-64">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        {t("component.components")}
                    </div>
                    <HasPermission displayDenied={false} identifier="component_create">
                        <Link
                            className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                            to="/admin/component-create"
                        >
                            {t("common.create")}
                        </Link>
                    </HasPermission>
                </div>

                <div className="overflow-x-hidden">
                    <HasPermission identifier="component_table">
                        <AvoRedTable table={table}/>
                    </HasPermission>
                </div>
            </div>
        </div>
    );
}

export default ComponentTable