import {useTranslation} from "react-i18next";
import {ContentSidebar} from "./ContentSidebar";
import {Link, useSearchParams} from "react-router-dom";
import HasPermission from "../../components/HasPermission";

export const ContentTable = (() => {
    const [t] = useTranslation("global");
    const [searchParams] = useSearchParams()
    const collectionType = searchParams.get("type")

    return (
        <div className="flex w-full">
            <div className="p-5 w-64 bg-gray-50 min-h-screen">
                <ContentSidebar />
            </div>
            <div className="p-5 flex-1">
                {collectionType ?
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
                    :
                    <>
                        {t('please select a collection from sidebar to see the collections entries.')}
                    </>
                }
            </div>
        </div>

    )
})