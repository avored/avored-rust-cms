import InputField from "../../components/InputField";
import AvoredModal from "../../components/AvoredModal";
import React, {useEffect, useState} from "react";
import {useTranslation} from "react-i18next";
import IAssetModel from "../../types/asset/IAssetModel";
import {DisplayAsset} from "../asset/DisplayAsset";
import _ from "lodash";
import {useAssetTable} from "../asset/hooks/useAssetTable";
import {Link, useParams} from "react-router-dom";
import {RenameAssetModal} from "../asset/RenameAssetModal";
import {Menu, MenuButton, MenuItem, MenuItems} from "@headlessui/react";
import {EllipsisHorizontalCircleIcon, FolderPlusIcon} from "@heroicons/react/24/outline";
import {useAxios} from "../../hooks/useAxios";
import AvoRedButton from "../../components/AvoRedButton";

type SingleImageModalProps = {
    isOpen: any,
    onCloseModal: any,
    selectedAsset: any
}

export const SingleImageModal = (({
    isOpen,
    onCloseModal,
    selectedAsset
}: SingleImageModalProps) => {
    const [t] = useTranslation("global");
    const client = useAxios();
    const [assets, setAssets]= useState<Array<IAssetModel>>([]);

    const backend_url = import.meta.env.VITE_AVORED_BACKEND_BASE_URL;
    const openFolder = async (e: React.MouseEvent<HTMLElement>, asset_id: string) => {
        e.preventDefault();
        await fetchAssets(asset_id)
    };

    const selectAssetButtonOnClick = (e: React.MouseEvent<HTMLButtonElement>, asset: IAssetModel) => {
        selectedAsset(asset)
    }

    const fetchAssets = async (asset_id: string) => {
        const assetUrl: string = _.isEmpty(asset_id) ? '/asset' : '/asset?parent_id=' + asset_id;

        const asset_api_table_response = await client.get(assetUrl)
        const test: Array<IAssetModel> = _.get(
            asset_api_table_response,
            "data.data",
            [],
        );
        setAssets(test)
    }

    useEffect(() => {
        const asset_id: string = ''
        const fetchData = async () => {
            await fetchAssets(asset_id);
        }
        fetchData()
            .catch(console.error)

    }, [])

    return (
        <>
            <div className="max-w-7xl">
                <AvoredModal
                    isOpen={isOpen}
                    closeModal={onCloseModal}
                    modal_header={t("select_asset")}
                    modal_body={
                        <div className="text-sm text-gray-500">
                            <div className="text-sm text-gray-500 rounded">
                                <div className="mt-5">
                                    <div className="px-4 mx-auto">
                                        <div className="flex flex-col mt-6">
                                            <div className="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                                                <div className="inline-block min-w-full p-2">
                                                    <div className="grid grid-cols-6  gap-4 mx-5">
                                                        {assets.map((asset: IAssetModel) => {
                                                            return (
                                                                <div key={asset.id} className="border rounded p-3">

                                                                    <div className="flex justify-center h-40 mb-3">
                                                                        {asset.asset_type === "FOLDER" ? (
                                                                            <>
                                                                                <FolderPlusIcon
                                                                                    className="h-32 w-32 text-gray-300"/>
                                                                            </>
                                                                        ) : (
                                                                            <>
                                                                                <img
                                                                                    src={`${backend_url}${asset.path}`}
                                                                                    className="h-40"
                                                                                    alt={asset.name}
                                                                                />
                                                                            </>
                                                                        )}
                                                                    </div>
                                                                    <div
                                                                        className="flex justify-center  text-xs text-slate-900">
                                                                        <div className="w-full items-center">
                                                                            {asset.asset_type === "FOLDER" ? (
                                                                                <>
                                                                                    <button
                                                                                        onClick={(e) => openFolder(e, asset.id)}
                                                                                        className="bg-gray-100 py-2 px-1 rounded w-full hover:bg-gray-200"
                                                                                        type="button"
                                                                                    >

                                                                                            {asset.name}

                                                                                    </button>
                                                                                </>
                                                                            ) : (
                                                                                <>
                                                                                    <div
                                                                                        className="text-ellipsis w-full overflow-hidden bg-gray-100 py-2 px-1 rounded">
                                                                                        {/** ADD COPY ICON AND Allow them to copy the file name **/}
                                                                                        {asset.name}
                                                                                    </div>
                                                                                    <div>
                                                                                        <AvoRedButton
                                                                                            onClick={(e: React.MouseEvent<HTMLButtonElement>) => selectAssetButtonOnClick(e, asset)}
                                                                                            className="bg-primary-600 hover:bg-primary-500  focus:ring-primary-500 mt-3"
                                                                                            label={t!("select_asset")} />
                                                                                    </div>
                                                                                </>
                                                                            )}
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            );
                                                        })}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                />
            </div>

        </>
    )
})