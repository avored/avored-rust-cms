import {useTranslation} from "react-i18next";
import {ModelSidebar} from "./ModelSidebar";
import {useSearchParams} from "react-router-dom";

export const ModelNew = (() => {

    const [t] = useTranslation("global")


    return (
            <div className="overflow-x-auto">
                <div className="flex w-full">
                    <div className="w-64 px-5 bg-gray-100 min-h-screen bor border-gray-200">
                        <ModelSidebar />
                    </div>
                    <div className="px-3 mt-5">
                        please select a model from sidebar to see the models entries.
                    </div>
                </div>
            </div>
    )
})