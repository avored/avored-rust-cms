import { Link } from "react-router-dom";
import _ from "lodash";
import { usePageTable } from "./hooks/usePageTable";
import { useTranslation } from "react-i18next";
import {
    createColumnHelper,
    getCoreRowModel,
    SortingState,
    useReactTable,
} from "@tanstack/react-table";
import { getFormattedDate } from "../../lib/common";
import IPageModel from "../../types/page/IPageModel";
import AvoRedTable from "../../components/AvoRedTable";
import HasPermission from "../../components/HasPermission";
import {useEffect, useState} from "react";
import AvoredModal from "../../components/AvoredModal";
import {usePageRemoveModal} from "./hooks/usePageRemoveModal";
import {useDeletePage} from "./hooks/useDeletePage";

function PageTable() {
    const { isOpen, updateOpen, setValToRemove, valToRemove } = usePageRemoveModal()
    const [pagination, setPagination] = useState({
        pageIndex: 0, //initial page index
        pageSize: 10, //default page size
    });
    const [sorting, setSorting] = useState<SortingState>([]);
    const [t] = useTranslation("global");
    const page_api_table_response = usePageTable({
        order: sorting.map((s) => `${s.id}:${s.desc ? "DESC" : "ASC"}`).join(","),
        page: pagination.pageIndex
    });
    const { mutate, isPending, isSuccess, data } = useDeletePage();
    const customPagination = (async (pagination: any) => {
        setPagination(pagination)
    })
    const pages: Array<IPageModel> = _.get(
        page_api_table_response,
        "data.data.data",
        [],
    );

    useEffect(() => {
        if (isSuccess) {
            updateOpen(false);
            page_api_table_response.refetch().then();
        }
    }, [isSuccess]);

    const customSorting = (sorting: any) => {
        setSorting(sorting);
    };

    const onDeleteSelect = (e: React.MouseEvent<HTMLAnchorElement>, value: IPageModel) => {
        e.preventDefault()
        updateOpen(true)
        setValToRemove(value);
    }

    const onDeleteItem = async () => {
        if (valToRemove?.id) {
            mutate(valToRemove?.id)
        }
    }

    const columnHelper = createColumnHelper<IPageModel>();
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
                    <HasPermission displayDenied={false} identifier="page_edit">
                        <Link
                            className="font-medium text-primary-600 hover:text-primary-800"
                            to={`/admin/page-edit/${info.row.original.id}`}
                        >
                            {t("edit")}
                        </Link>
                        <div>
                            <Link
                                className="font-medium text-primary-600 hover:text-primary-800"
                                to='#'
                                onClick={(e) => onDeleteSelect(e, info.row.original)}
                            >
                                {t("delete")}
                            </Link>
                        </div>
                    </HasPermission>
                );
            },
            enableSorting: false,
            header: t("action"),
            enableHiding: false,
        }),
    ];

    const table = useReactTable({
        data: pages,
        columns,
        getCoreRowModel: getCoreRowModel(),
        rowCount: page_api_table_response.data?.data.pagination.total,
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
        <>
            <div className="px-5">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        {t("page")}
                    </div>
                    <div className="ml-auto">
                        <HasPermission displayDenied={false} identifier="page_create">
                            <Link
                                className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                to="/admin/page-create"
                            >
                                {t("create")}
                            </Link>
                        </HasPermission>
                    </div>
                </div>

                <div className="overflow-x-auto">
                    <HasPermission identifier="page_table">
                        <AvoRedTable table={table} />
                    </HasPermission>
                </div>
            </div>
            <AvoredModal
                closeModal={() => updateOpen(false)}
                modal_header={t('intention_to_remove_page', {page_var: valToRemove?.name})}
                modal_body={
                    <div>
                    <button onClick={() => updateOpen(false)} className="mr-2 ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">Cancel</button>
                        <button onClick={onDeleteItem} className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">{isPending ? 'Loading...' : 'Remove'}</button>
                    </div>
            }
                isOpen={isOpen}
            ></AvoredModal>
       </>
    );
}

export default PageTable;
