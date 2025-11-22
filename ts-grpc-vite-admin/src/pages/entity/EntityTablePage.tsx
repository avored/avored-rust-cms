import {useState} from "react";
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table";
import {useTranslation} from "react-i18next";
import AvoRedTable from "../../components/AvoRedTable";
import {GrpcTimeStamp} from "../../types/common/common";
import { DateTime } from 'luxon'
import {Link} from "react-router-dom";
import HasPermission from "../../components/HasPermission";
import { EntityType } from "../../types/entity/EntityType";
import { EntityPaginateRequest } from "grpc-avored/entity_pb";
import { UseEntityPaginateHook } from "../../hooks/entity/UseEntityPaginateHook";

export const EntityTablePage = (() => {
    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });
    const [sorting, setSorting] = useState<SortingState>([]);


    const request = new EntityPaginateRequest()
    const entity_paginate_response = UseEntityPaginateHook(request, {
        order: sorting.map((s) => `${s.id}:${s.desc ? 'DESC' : 'ASC'}`).join(','),
        page: pagination.pageIndex
    });

    const admin_error = entity_paginate_response.error;

    console.log(admin_error)

    const data_list = entity_paginate_response.data?.data?.dataList ?? [];
    const entities: Array<EntityType> = data_list as Array<unknown> as EntityType[];

    const customSorting = (async (sorting: any) => {
        setSorting(sorting)
    })
    const customPagination = (async (page: any) => {
        setPagination(page)
    })

    const [t] = useTranslation("global");

    const getFormattedDate = (date: GrpcTimeStamp) => {
        const date_object = DateTime.fromSeconds(date.seconds);
        return date_object.toFormat("dd-MM-yyyy HH:mm:ss")
    }

    const columnHelper = createColumnHelper<EntityType>()

    const columns = [
        columnHelper.accessor('id', {
            cell: info =>  info.getValue(),
            header: t("id")
        }),
        columnHelper.accessor('name', {
            cell: info => info.getValue(),
            header: t("name")
        }),
        columnHelper.accessor('identifier', {
            cell: info => info.getValue(),
            header: t("identifier"),
        }),
        columnHelper.accessor('createdAt', {
            id: "createdAt",
            cell: info => getFormattedDate(info.getValue()),
            header: t("created_at")
        }),
        columnHelper.accessor('createdBy', {
            cell: info => info.getValue(),
            header: t("created_by")
        }),
        columnHelper.accessor('updatedAt', {
            cell: info => getFormattedDate(info.getValue()),
            header: t("updated_at")
        }),
        columnHelper.accessor('updatedBy', {
            cell: info => info.getValue(),
            header: t("updated_by")
        }),
        columnHelper.accessor('action', {
            cell: info => {
                return (
                    <Link
                        className="font-medium text-primary-600 hover:text-primary-800"
                        to={`/admin/entity-edit/${info.row.original.id}`}
                    >
                        {t("edit")}
                    </Link>
                )
            },
            header: t("action"),
            enableHiding: false,
            enableSorting: false
        }),
    ];

    // const adminUserTableResponse = { data : {}};

    const table = useReactTable({
        data: entities,
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
        rowCount: entity_paginate_response.data?.data?.pagination?.total ?? 0,
        initialState: {
            columnVisibility: {
                createdAt: false,
                createdBy: false,
                updatedAt: false,
                updatedBy: false,
            },
            pagination
        }
    })

    // const getRoleNames = ((roles: any) => {
    //     if (roles.length === 0) {
    //         return (<></>)
    //     }
    //     return roles.map((role) => {
    //         return (
    //             <span key={role.id} className="bg-gray-300 p-1 rounded mr-1">
    //                 {role.name}
    //             </span>
    //         )
    //     })
    // })


    return (
        <>
            <div className="p-5">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        {t("entities")}
                    </div>
                    <HasPermission displayDenied={false} identifier="entity_create">
                        <Link
                            className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                            to="/admin/entity-create"
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
    )
});
