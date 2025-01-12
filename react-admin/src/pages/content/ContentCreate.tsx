import {useTranslation} from "react-i18next";
import {ContentSidebar} from "./ContentSidebar";
import {useSearchParams} from "react-router-dom";

export const ContentCreate = (() => {
    const [t] = useTranslation("global");
    const [searchParams] = useSearchParams()
    // const collectionType = searchParams.get("type")

    return (
        <div className="flex w-full">
            <div className="p-5 w-64 bg-gray-50 min-h-screen">
                <ContentSidebar />
            </div>
            <div className="p-5 flex-1">
                Create form goes here
            </div>
        </div>

    )
})