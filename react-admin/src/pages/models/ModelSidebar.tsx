import {useTranslation} from "react-i18next";
import {useModelAll} from "./hooks/useModelAll";
import {ModelType} from "../../types/model/ModelType";
import _ from 'lodash';

export const ModelSidebar = (() => {
    const [t] = useTranslation("global")
    const models_api_response = useModelAll()
    console.log(models_api_response)
    const models: Array<ModelType> = _.get(models_api_response, 'data.data.data', [])
    return (
        <>
            <div className="py-5 text-xl font-semibold text-primary-500">
                {t("models")}
            </div>
            <div className="mt-5">
                {models.map((model: ModelType) => {
                    return (
                        <div className="mt-3 text-sm cursor-pointer text-primary-600 overflow-x-hidden font-semibold bg-gray-300 p-3">
                            {model.name}
                        </div>
                    )
                })}
            </div>
        </>
    )
})