import {useTranslation} from "react-i18next";
import {ModelSidebar} from "./ModelSidebar";
import {Link, useSearchParams} from "react-router-dom";
import HasPermission from "../../components/HasPermission";

export const ModelNew = (() => {

    const [t] = useTranslation("global")
    const [searchParams] = useSearchParams()
    const modelType = searchParams.get("type")


    return (
            <div className="overflow-x-auto">
                <div className="flex w-full">
                    <div className="w-64 px-5 bg-gray-100 min-h-screen bor border-gray-200">
                        <ModelSidebar />
                    </div>
                    <div className="px-3 mt-5">
                        {modelType ?
                            <div className="flex items-center w-full">
                                <div className="p-5 text-2xl font-semibold text-primary-500">
                                    {t("model type table title")}
                                </div>
                                <div className="ml-auto">
                                    <HasPermission displayDenied={false} identifier="model_create">
                                        <Link
                                            className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                            to={encodeURI(`/admin/model-create?type=${modelType}`)}
                                        >
                                            {t("create")}
                                        </Link>
                                    </HasPermission>
                                </div>
                            </div>
                            :
                            <>
                                {t('please select a model from sidebar to see the models entries.')}
                            </>
                        }
                    </div>
                </div>
            </div>
    )
})