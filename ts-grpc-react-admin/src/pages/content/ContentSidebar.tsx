import {Link, useSearchParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {
    CollectionType,
    StoreCollectionType,
    UpdateCollectionType
} from "../../types/content/ContentType";
import {UseCollectionAllHook} from "../../hooks/content/UseCollectionAllHook";
import {CollectionAllRequest, StoreCollectionRequest, UpdateCollectionRequest} from "../../grpc_generated/content_pb";
import AvoRedIconButton from "../../components/AvoRedIconButton";
import {EllipsisVerticalIcon, PlusIcon} from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";
import {useState} from "react";
import InputField from "../../components/InputField";
import ErrorMessage from "../../components/ErrorMessage";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import AvoRedButton, {ButtonType} from "../../components/AvoRedButton";
import {UseStoreCollectionHook} from "../../hooks/content/UseStoreCollectionHook";
import {UseUpdateCollectionHook} from "../../hooks/content/UseUpdateCollectionHook";
import {UseCollectionCreateSchema} from "../../schemas/content/UseCollectionCreateSchema";
import {UseCollectionUpdateSchema} from "../../schemas/content/UseCollectionUpdateSchema";
import {RefetchOptions} from "@tanstack/react-query";

export const ContentSidebar = (() => {
    const [t] = useTranslation("global");
    const [isCreateCollectionModalOpen, setIsCreateCollectionModalOpen] = useState<boolean>(false);
    const [isEditCollectionModalOpen, setIsEditCollectionModalOpen] = useState<boolean>(false);
    const [searchParams] = useSearchParams()
    const request = new CollectionAllRequest()
    const collections_api_response = UseCollectionAllHook(request)

    const refetch= collections_api_response?.refetch;
    const collection_all_data_list = collections_api_response?.data?.dataList ?? [];
    const collections: Array<CollectionType> = collection_all_data_list as Array<unknown> as CollectionType[];

    const {mutate: storeCollectionMutate, error: storeCollectionError} = UseStoreCollectionHook(refetch)
    const {mutate: updateCollectionMutate, error: updateCollectionError} = UseUpdateCollectionHook(refetch)


    const {
        register: createRegister,
        handleSubmit: createHandleSubmit,
        formState: { errors: createErrors }
    } = useForm<StoreCollectionType>({
        resolver: joiResolver(UseCollectionCreateSchema()),
    })

    const {
        register: updateRegister,
        handleSubmit: updateHandleSubmit,
        formState: { errors: updateErrors },
        setValue: updateSetValue,
    } = useForm<UpdateCollectionType>({
        resolver: joiResolver(UseCollectionUpdateSchema(), {allowUnknown: true}),
    })

    const storeCollectionHandler = ((data: StoreCollectionType) => {
        const request = new StoreCollectionRequest();
        request.setName(data.name)
        request.setIdentifier(data.identifier)
        storeCollectionMutate(request)
        setIsCreateCollectionModalOpen(false)
    })

    const updateCollectionHandler = (async (data: UpdateCollectionType) => {
        const request = new UpdateCollectionRequest();
        request.setId(data.id)
        request.setName(data.name)
        request.setIdentifier(data.identifier)
        updateCollectionMutate(request)
        setIsEditCollectionModalOpen(false)
        // await collections_api_response?.refetch()
    })

    const editCollectionOnClick = (e:any, collection: CollectionType) => {
        e.preventDefault();
        updateSetValue('id' ,collection.id)
        updateSetValue('name' ,collection.name)
        updateSetValue('identifier' ,collection.identifier)
        setIsEditCollectionModalOpen(true)
    }

    return (
        <>
            {/* Create Collection modal */}
            <AvoredModal
                modal_header={t("create_collection")}
                modal_body={(
                    <div className="w-full">
                        <div className="w-full mt-5">
                            <InputField
                                label={t("name")}
                                type="text"
                                name="name"
                                register={createRegister("name")}
                            />
                            <ErrorMessage frontendErrors={createErrors} backendErrors={storeCollectionError} identifier="name" />
                        </div>
                        <div className="w-full mt-5">
                            <InputField
                                label={t("identifier")}
                                type="text"
                                name="identifier"
                                register={createRegister("identifier")}
                            />
                            <ErrorMessage frontendErrors={createErrors} backendErrors={storeCollectionError} identifier="identifier" />
                        </div>
                        <div className="mt-5">
                            <AvoRedButton
                                label={"Save"}
                                className={"bg-primary-600 hover:bg-primary-500 focus:ring-primary-500"}
                                type={ButtonType.button}
                                onClick={createHandleSubmit(storeCollectionHandler)}
                            />
                        </div>
                    </div>
                )}
                isOpen={isCreateCollectionModalOpen}
                closeModal={setIsCreateCollectionModalOpen}
            />

            {/* Edit Collection modal */}
            <AvoredModal
                modal_header={t("edit_collection")}
                modal_body={(
                    <div className="w-full">
                        <div className="w-full mt-5">
                            <InputField
                                label={t("name")}
                                type="text"
                                name="name"
                                register={updateRegister("name")}
                            />
                            <ErrorMessage frontendErrors={updateErrors} backendErrors={updateCollectionError} identifier="name" />
                        </div>
                        <div className="w-full mt-5">
                            <InputField
                                label={t("identifier")}
                                type="text"
                                name="identifier"
                                register={updateRegister("identifier")}
                            />
                            <ErrorMessage frontendErrors={updateErrors} backendErrors={updateCollectionError} identifier="identifier" />
                        </div>
                        <div className="mt-5">
                            <AvoRedButton
                                label={"Save"}
                                className={"bg-primary-600 hover:bg-primary-500 focus:ring-primary-500"}
                                type={ButtonType.button}
                                onClick={updateHandleSubmit(updateCollectionHandler)}
                            />
                        </div>
                    </div>
                )}
                isOpen={isEditCollectionModalOpen}
                closeModal={setIsEditCollectionModalOpen}
            />
            <div className="mt-5 flex w-full justify-center text-primary-500 font-semibold">

                <div>
                    {t("collections")}
                </div>
                <div className="text-sm ml-auto pr-3">
                    <AvoRedIconButton icon={(
                        <PlusIcon onClick={() => setIsCreateCollectionModalOpen(true)}  className="h-4 w-4"/>
                    )} />
                </div>
            </div>

            <ul className="mt-5">
                {collections.map((collection: CollectionType) => {
                    return (
                        <li key={collection.id}>
                            <div className={`rounded p-3 mt-3 flex w-full justify-between text-sm ${searchParams.get("type") === collection.identifier ? 'text-primary-600 font-semibold bg-gray-300' : ''}`}>

                                <div className="flex-1">
                                    <Link
                                        to={`/admin/content?type=${encodeURI(collection.identifier)}`}
                                        key={collection.identifier}
                                        className={`text-sm cursor-pointer overflow-x-hidden`}
                                    >
                                        {collection.name}
                                    </Link>
                                </div>
                                <div className="ml-auto cursor-pointer ">
                                    <EllipsisVerticalIcon onClick={(e) => editCollectionOnClick(e, collection)} className="h-4 w-4" />
                                </div>
                            </div>

                        </li>
                    )
                })}
            </ul>
        </>
    )
})
