import {ContentSidebar} from "./ContentSidebar";
import {Link, useSearchParams} from "react-router-dom";
import AvoRedTable from "../../components/AvoRedTable";
import {useTranslation} from "react-i18next";
import {useState} from "react";
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table";
import {ContentType} from "../../types/content/ContentType";
import {GrpcTimeStamp} from "../../types/common/common";
import {DateTime} from "luxon";
import {ContentPaginateRequest} from "../../grpc_generated/content_pb";
import {UseContentPaginateHook} from "../../hooks/content/UseContentPaginateHook";
import {AdminUserType} from "../../types/admin_user/AdminUserType";
import _ from "lodash";

export const ContentTablePage = () => {
    const [t] = useTranslation("global");
    const [searchParams] = useSearchParams()
    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });
    const [sorting, setSorting] = useState<SortingState>([]);

    const contentType = searchParams.get("type") ?? ''

    var contents = [] as ContentType[];
    var total_no_of_record = 0;

    if (!_.isEmpty(contentType)) {
        const request = new ContentPaginateRequest()
        request.setContentType(contentType)

        const content_paginate_response = UseContentPaginateHook(request)
        const data_list = content_paginate_response.data?.data?.dataList ?? [];
        contents = data_list as Array<unknown> as ContentType[];
        total_no_of_record = content_paginate_response.data?.data?.pagination?.total ?? 0;
    }




    const customSorting = (sorting: any) => {
        setSorting(sorting);
    };

    const customPagination = (async (pagination: any) => {
        setPagination(pagination)
    })

    const getFormattedDate = (date: GrpcTimeStamp) => {
        const date_object = DateTime.fromSeconds(date.seconds);
        return date_object.toFormat("dd-MM-yyyy HH:mm:ss")
    }



    const columnHelper = createColumnHelper<ContentType>();
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
        columnHelper.accessor("createdAt", {
            id: "createdAt",
            cell: (info) => getFormattedDate(info.getValue()),
            header: t("created_at"),
        }),
        columnHelper.accessor("createdBy", {
            cell: (info) => info.getValue(),
            header: t("created_by"),
        }),
        columnHelper.accessor("updatedAt", {
            cell: (info) => getFormattedDate(info.getValue()),
            header: t("updated_at"),
        }),
        columnHelper.accessor("updatedBy", {
            cell: (info) => info.getValue(),
            header: t("updated_by"),
        }),
        columnHelper.accessor("action", {
            cell: (info) => {
                return (
                    <>
                        <Link
                            className="font-medium text-primary-600 hover:text-primary-800"
                            to={`/admin/content-edit/${info.row.original.id}?type=${contentType}`}
                        >
                            {t("edit")}
                        </Link>
                    </>
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
        rowCount: total_no_of_record,
        onPaginationChange: customPagination,
        manualPagination: true,
        initialState: {
            columnVisibility: {
                createdAt: false,
                createdBy: false,
                updatedBy: false,
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

    return(
        <>
            <div className="p-5">
                <div className="flex w-full">
                    <div className="w-48 bg-gray-50">
                        <ContentSidebar />
                    </div>
                    <div className="flex-1">
                        {contentType ?
                            <>
                                <div className="flex items-center">
                                    <div className="p-5 text-2xl font-semibold text-primary-500">
                                        {t("collection type table title")}
                                    </div>
                                    <div className="ml-auto">
                                        {/*<HasPermission displayDenied={false} identifier="collection_create">*/}
                                        <Link
                                            className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                            to={encodeURI(`/admin/content-create?type=${contentType}`)}
                                        >
                                            {t("create")}
                                        </Link>
                                        {/*</HasPermission>*/}
                                    </div>
                                </div>
                                <div className="block overflow-x-auto">
                                    {/*<HasPermission identifier="content_table">*/}
                                    <AvoRedTable table={table} />
                                    {/*</HasPermission>*/}
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

            </div>
        </>
    )

}