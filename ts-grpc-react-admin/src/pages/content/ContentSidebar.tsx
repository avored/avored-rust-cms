import {Link, useSearchParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {CollectionType} from "../../types/content/ContentType";
import {UseCollectionAllHook} from "../../hooks/content/UseCollectionAllHook";
import {CollectionAllRequest} from "../../grpc_generated/content_pb";

export const ContentSidebar = (() => {
    const [t] = useTranslation("global");
    const [searchParams] = useSearchParams()
    const request = new CollectionAllRequest()
    const collections_api_response = UseCollectionAllHook(request)

    const collection_all_data_list = collections_api_response?.data?.dataList ?? [];
    const collections: Array<CollectionType> = collection_all_data_list as Array<unknown> as CollectionType[];

    return (
        <>
            <div className="text-primary-500 font-semibold">
                {t("collections")}
            </div>

            <ul className="mt-5">
                {collections.map((collection: CollectionType) => {
                    return (
                        <li key={collection.id}>
                            <Link
                                to={`/admin/content?type=${encodeURI(collection.identifier)}`}
                                key={collection.identifier}
                                className={`rounded block mt-3 p-3 text-sm cursor-pointer ${searchParams.get("type") === collection.identifier ? 'text-primary-600 font-semibold bg-gray-300' : ''}  overflow-x-hidden`}
                            >
                                {collection.name}
                            </Link>
                        </li>
                    )
                })}


            </ul>
        </>
    )
})