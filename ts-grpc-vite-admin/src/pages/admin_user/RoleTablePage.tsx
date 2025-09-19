import {useState} from "react";
import {createColumnHelper, getCoreRowModel, SortingState, useReactTable} from "@tanstack/react-table";
import {RolePaginateRequest} from "grpc-avored/admin_user_pb";
import {RoleType} from "../../types/admin_user/AdminUserType";
import {useTranslation} from "react-i18next";
import {GrpcTimeStamp} from "../../types/common/common";
import {DateTime} from "luxon";
import {Link} from "react-router-dom";
import HasPermission from "../../components/HasPermission";
import AvoRedTable from "../../components/AvoRedTable";
import {UseRolePaginateHook} from "../../hooks/admin_user/UseRolePaginateHook";

export const RoleTablePage = () => {
    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });
    const [sorting, setSorting] = useState<SortingState>([]);

    let request = new RolePaginateRequest();

    const role_res = UseRolePaginateHook(request, {
        order: sorting.map((s) => `${s.id}:${s.desc ? 'DESC' : 'ASC'}`).join(','),
        page: pagination.pageIndex
    });
    const data_list = role_res.data?.data?.dataList ?? [];
    const roles: Array<RoleType> = data_list as Array<unknown> as RoleType[];

    const customSorting = (async (sorting: any) => {
        setSorting(sorting)
    })
    const customPagination = (async (pagination: any) => {
        setPagination(pagination)
    })

    const [t] = useTranslation("global");

    const getFormattedDate = (date: GrpcTimeStamp) => {
        const date_object = DateTime.fromSeconds(date.seconds);
        return date_object.toFormat("dd-MM-yyyy HH:mm:ss")
    }

    const columnHelper = createColumnHelper<RoleType>()

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
                        to={`/admin/role-edit/${info.row.original.id}`}
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
        data: roles,
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
        rowCount: role_res.data?.data?.pagination?.total ?? 1,
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
    return (
        <>
            <div className="p-5">
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
        </>
    )
}
