import {useTranslation} from "react-i18next";
import {useModelAll} from "./hooks/useModelAll";
import {ModelType} from "../../types/model/ModelType";
import _ from 'lodash';
import {Link, useSearchParams} from "react-router-dom";

export const ModelSidebar = (() => {
    const [t] = useTranslation("global")
    const [searchParams] = useSearchParams()
    const models_api_response = useModelAll()

    const models: Array<ModelType> = _.get(models_api_response, 'data.data.data', [])
    return (
        <>
            <div className="py-5 text-xl font-semibold text-primary-500">
                {t("models")}
            </div>
            <div className="mt-5">
                {models.map((model: ModelType) => {
                    return (
                        <Link
                            to={`/admin/models?type=${encodeURI(model.identifier)}`}
                            key={model.identifier}
                            className={`block mt-3 p-3 text-sm cursor-pointer ${searchParams.get("type") === model.identifier ? 'text-primary-600 font-semibold bg-gray-300' : ''}  overflow-x-hidden`}
                        >
                            {model.name}
                        </Link>
                    )
                })}
            </div>
        </>
    )
})