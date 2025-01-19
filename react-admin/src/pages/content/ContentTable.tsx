import {useTranslation} from "react-i18next";
import {ContentSidebar} from "./ContentSidebar";
import {Link, useSearchParams} from "react-router-dom";
import HasPermission from "../../components/HasPermission";
import _ from 'lodash';
import {ContentModel} from "../../types/content/ContentType";
import {useContentTable} from "./hooks/useContentTable";
import {useState} from "react";
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table";
import {getFormattedDate} from "../../lib/common";
import AvoRedTable from "../../components/AvoRedTable";

export const ContentTable = (() => {
    const [t] = useTranslation("global");
    const [searchParams] = useSearchParams()
    const collectionType = searchParams.get("type")

    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });
    const [sorting, setSorting] = useState<SortingState>([]);
    const content_api_table_response = useContentTable({
        order: sorting.map((s) => `${s.id}:${s.desc ? "DESC" : "ASC"}`).join(","),
        page: pagination.pageIndex
    }, collectionType);

    const contents: Array<ContentModel> = _.get(
        content_api_table_response,
        "data.data.data.data",
        [],
    );


    const customSorting = (sorting: any) => {
        setSorting(sorting);
    };

    const customPagination = (async (pagination: any) => {
        setPagination(pagination)
    })

    const columnHelper = createColumnHelper<ContentModel>();
    const columns = [
        columnHelper.accessor("id", {
            cell: (info) => info.getValue(),
            header: t("id"),
        }),
        columnHelper.accessor("name", {
            cell: (info) => info.getValue(),
            header: t("name"),
        }),
        columnHelper.accessor("identifier", {
            cell: (info) => info.getValue(),
            header: t("identifier"),
        }),
        columnHelper.accessor("created_at", {
            id: "created_at",
            cell: (info) => getFormattedDate(info.getValue()),
            header: t("created_at"),
        }),
        columnHelper.accessor("created_by", {
            cell: (info) => info.getValue(),
            header: t("created_by"),
        }),
        columnHelper.accessor("updated_at", {
            cell: (info) => getFormattedDate(info.getValue()),
            header: t("updated_at"),
        }),
        columnHelper.accessor("updated_by", {
            cell: (info) => info.getValue(),
            header: t("updated_by"),
        }),
        columnHelper.accessor("action", {
            cell: (info) => {
                return (
                    <HasPermission displayDenied={false} identifier="content_edit">
                        <Link
                            className="font-medium text-primary-600 hover:text-primary-800"
                            to={`/admin/content-edit/${info.row.original.id}`}
                        >
                            {t("edit")}
                        </Link>
                        {/*<div>*/}
                        {/*    <Link*/}
                        {/*        className="font-medium text-primary-600 hover:text-primary-800"*/}
                        {/*        to='#'*/}
                        {/*        onClick={(e) => onDeleteSelect(e, info.row.original)}*/}
                        {/*    >*/}
                        {/*        {t("delete")}*/}
                        {/*    </Link>*/}
                        {/*</div>*/}
                    </HasPermission>
                );
            },
            enableSorting: false,
            header: t("action"),
            enableHiding: false,
        }),
    ];


    const table = useReactTable({
        data: contents,
        columns,
        getCoreRowModel: getCoreRowModel(),
        rowCount: _.get(content_api_table_response, '.data.data.data.pagination.total', 10),
        onPaginationChange: customPagination,
        manualPagination: true,
        initialState: {
            columnVisibility: {
                created_at: false,
                created_by: false,
            },
            pagination
        },
        manualSorting: true,
        onSortingChange: customSorting,
        state: {
            sorting,
            pagination
        },
    });


    return (
        <div className="flex w-full">
            <div className="p-5 w-48 bg-gray-50 min-h-screen">
                <ContentSidebar />
            </div>
            <div className="p-5 flex-1">
                {collectionType ?
                    <>
                        <div className="flex items-center w-full">
                            <div className="p-5 text-2xl font-semibold text-primary-500">
                                {t("collection type table title")}
                            </div>
                            <div className="ml-auto">
                                <HasPermission displayDenied={false} identifier="collection_create">
                                    <Link
                                        className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                        to={encodeURI(`/admin/content-create?type=${collectionType}`)}
                                    >
                                        {t("create")}
                                    </Link>
                                </HasPermission>
                            </div>
                        </div>
                        <div className="overflow-x-auto">
                            <HasPermission identifier="content_table">
                                <AvoRedTable table={table} />
                            </HasPermission>
                        </div>
                    </>

                    :
                    <>
                        <div>
                            {t("please select the type on left hand side")}
                        </div>
                    </>
                }
            </div>
        </div>

    )
})