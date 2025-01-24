import HasPermission from "../../components/HasPermission"
import {Link} from "react-router-dom"
import {useTranslation} from "react-i18next"
import {useState} from "react"
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table"
import AvoRedTable from "../../components/AvoRedTable"
import {getFormattedDate} from "../../lib/common"
import _ from "lodash"
import {useCollectionTable} from "./hooks/useCollectionTable";
import {CollectionType} from "../../types/collection/CollectionType";

export const CollectionTable = (() => {
    const [t] = useTranslation("global")
    const [sorting, setSorting] = useState<SortingState>([]);
    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });

    const customSorting = ((sorting: any) => {
        setSorting(sorting)
    })

    const collection_api_table_response = useCollectionTable({
        order: sorting.map((s) => `${s.id}:${s.desc ? 'DESC' : 'ASC'}`).join(','),
        page: pagination.pageIndex
    });
    const customPagination = (async (pagination: any) => {
        setPagination(pagination)
    })
    const collections: Array<CollectionType> = _.get(collection_api_table_response, 'data.data.data', [])

    const columnHelper = createColumnHelper<CollectionType>()
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
                    <HasPermission displayDenied={false} identifier="collection_edit">
                        <Link
                            className="font-medium text-primary-600 hover:text-primary-800"
                            to={`/admin/collection-edit/${info.row.original.id}`}
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
        data: collections,
        columns,
        getCoreRowModel: getCoreRowModel(),
        rowCount: collection_api_table_response.data?.data.pagination.total,
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
        <>
            <div className="px-5">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        {t("collection")}
                    </div>
                    <div className="ml-auto">
                        <HasPermission displayDenied={false} identifier="collection_create">
                            <Link
                                className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                to="/admin/collection-create"
                            >
                                {t("create")}
                            </Link>
                        </HasPermission>
                    </div>
                </div>

                <div className="overflow-x-auto">
                    <HasPermission identifier="collection_table">
                        <AvoRedTable table={table}/>
                    </HasPermission>
                </div>
            </div>
        </>
    )
})