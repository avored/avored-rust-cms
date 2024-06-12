import {Link} from "react-router-dom"
import _ from 'lodash'
import {usePageTable} from "./hooks/usePageTable"
import {useTranslation} from "react-i18next"
import {createColumnHelper, getCoreRowModel, useReactTable} from "@tanstack/react-table";
import {getFormattedDate} from "../../lib/common";
import IPageModel from "../../types/page/IPageModel";
import AvoRedTable from "../../components/AvoRedTable";

function PageTable() {
    const [t] = useTranslation("global")
    const page_api_table_response = usePageTable();
    const pages: Array<IPageModel> = _.get(page_api_table_response, 'data.data.data', [])

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
                    <Link
                        className="font-medium text-primary-600 hover:text-primary-800"
                        to={`/admin/page-edit/${info.row.original.id}`}
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
        data: pages,
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
                        {t("pages.pages")}
                    </div>
                    <Link
                        className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                        to="/admin/page-create"
                    >
                        {t("common.create")}
                    </Link>
                </div>

                <div className="overflow-x-auto">
                    <AvoRedTable table={table}/>
                </div>
            </div>
        </div>
    );
}

export default PageTable