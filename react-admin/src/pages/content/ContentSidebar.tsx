import {Link, useSearchParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {useCollectionAll} from "./hooks/useCollectionAll";
import {CollectionType} from "../../types/collection/CollectionType";
import _ from 'lodash';

export const ContentSidebar = (() => {
    const [t] = useTranslation("global");
    const [searchParams] = useSearchParams()
    const collections_api_response = useCollectionAll()
    const collections: Array<CollectionType> = _.get(collections_api_response, 'data.data.data', [])
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