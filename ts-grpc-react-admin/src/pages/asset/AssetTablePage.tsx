import {useState} from "react";
import {useParams} from "react-router-dom";
import {UseAssetTableHook} from "../../hooks/asset/UseAssetTableHook";
import {AssetSaveType, AssetType} from "../../types/asset/AssetType";
import {useTranslation} from "react-i18next";
import {UseStoreAssetHook} from "../../hooks/asset/UseStoreAssetHook";
import {SubmitHandler, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import { AssetSaveSchema } from "../../schemas/asset/AssetSaveSchema";
import {AssetUploadModal} from "./AssetUploadModal";
import {CreateFolderModal} from "./CreateFolderModal";
import {DisplayAsset} from "./DisplayAsset";
import {AssetPaginateRequest} from "../../grpc_generated/asset_pb";

export const AssetTablePage = () => {
    const [isUploadAssetModalOpen, setIsUploadAssetModalOpen] = useState(false);
    const [isCreateFolderModalOpen, setIsCreateFolderModalOpen] = useState(false);

    const params = useParams()
    const asset_id: string = params.asset_id ?? ''
    const request = new AssetPaginateRequest();

    const asset_api_table_response = UseAssetTableHook(request);

    const data_list = asset_api_table_response.data?.data?.dataList ?? [];
    const assets = data_list as Array<unknown> as AssetType[];


    const {mutate} = UseStoreAssetHook(asset_id);
    const [t] = useTranslation("global");

    const {
        register,
        handleSubmit,
        // formState: {errors},
    } = useForm({
        resolver: joiResolver(AssetSaveSchema, {allowUnknown: true}),
    });

    const openFolder = (e: React.MouseEvent<HTMLElement>, asset_id: string) => {
        e.preventDefault();
        console.log(asset_id);
    };

    const onCloseCreateFolderModal = () => {
        setIsCreateFolderModalOpen(false);
    };

    const openCreateFolderModal = () => {
        setIsCreateFolderModalOpen(true);
    };

    const onCloseUploadModal = () => {
        setIsUploadAssetModalOpen(false);
    };

    const openUploadAssetModal = () => {
        setIsUploadAssetModalOpen(true);
    };

    const submitHandler: SubmitHandler<AssetSaveType> = (data: AssetSaveType) => {
        const upload_file: AssetSaveType = {
            file: data.file_list ? data.file_list[0] : undefined,
        }
        // data.file = data.file_list ? data.file_list[0] : undefined;
        onCloseUploadModal();
        mutate(upload_file);
    };

    return(
        <>
            <div className="p-5 flex w-full items-center">
                <div className="text-primary-500 text-2xl font-semibold">
                    {t("asset_table")}
                </div>
                <div className="ml-auto">
                    <button
                        type="button"
                        onClick={openUploadAssetModal}
                        className="bg-primary-500 rounded-md bg-black/20 px-4 py-2 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
                    >
                        {t("upload_asset")}
                    </button>
                    <button
                        type="button"
                        onClick={openCreateFolderModal}
                        className="ml-3 bg-gray-400 rounded-md bg-black/20 px-4 py-2 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
                    >
                        {t("create_folder")}
                    </button>
                </div>

                <div className="">
                    <AssetUploadModal
                        onCloseModal={onCloseUploadModal}
                        isOpen={isUploadAssetModalOpen}
                        submitHandler={submitHandler}
                        handleSubmit={handleSubmit}
                        register={register}
                    />
                    <CreateFolderModal
                        parent_id={asset_id}
                        onCloseModal={onCloseCreateFolderModal}
                        isOpen={isCreateFolderModalOpen}
                    />
                </div>
            </div>

            <div className="mt-5">
                <div className="px-4 mx-auto">
                    <div className="flex flex-col mt-6">
                        <div className="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                            <div className="inline-block min-w-full p-2">
                                <div className="grid grid-cols-6  gap-4 mx-5">
                                    {assets.map((asset: AssetType) => {
                                        return (
                                            <DisplayAsset
                                                openFolder={openFolder}
                                                key={asset.id}
                                                asset={asset}
                                            />
                                        );
                                    })}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    )
}
