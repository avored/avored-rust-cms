import {useState} from "react";
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table";
import {useTranslation} from "react-i18next";
import AvoRedTable from "../../components/AvoRedTable";
import {UseAdminUserPaginateHook} from "../../hooks/admin_user/UseAdminUserPaginateHook";
import {AdminUserType} from "../../types/admin_user/AdminUserType";
import {GrpcTimeStamp} from "../../types/common/common";
import { DateTime } from 'luxon'
import {Link} from "react-router-dom";
import HasPermission from "../../components/HasPermission";
import { AdminUserPaginateRequest } from "grpc-avored/admin_user_pb";

export const AdminUserTablePage = (() => {
    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });
    const [sorting, setSorting] = useState<SortingState>([]);


    const request = new AdminUserPaginateRequest()
    const admin_user_res = UseAdminUserPaginateHook(request, {
        order: sorting.map((s) => `${s.id}:${s.desc ? 'DESC' : 'ASC'}`).join(','),
        page: pagination.pageIndex
    });

    const admin_error = admin_user_res.error;

    console.log(admin_error)

    const data_list = admin_user_res.data?.data?.dataList ?? [];
    const admin_users: Array<AdminUserType> = data_list as Array<unknown> as AdminUserType[];

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

    const columnHelper = createColumnHelper<AdminUserType>()

    const columns = [
        columnHelper.accessor('id', {
            cell: info =>  info.getValue(),
            header: t("id")
        }),
        columnHelper.accessor('fullName', {
            cell: info => info.getValue(),
            header: t("full_name")
        }),
        columnHelper.accessor('email', {
            cell: info => info.getValue(),
            header: t("email"),
        }),
        columnHelper.accessor('isSuperAdmin', {
            cell: info => info.getValue() ? t("yes") : t("no"),
            header: t("is_super_admin"),
            enableSorting: false
        }),
        // columnHelper.accessor('roles', {
        //     cell: info => getRoleNames(info.getValue() ?? []),
        //     header: t("role"),
        //     enableSorting: false,
        // }),
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
    ];

    // const adminUserTableResponse = { data : {}};

    const table = useReactTable({
        data: admin_users,
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
        rowCount: admin_user_res.data?.data?.pagination?.total ?? 0,
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
    )
});
