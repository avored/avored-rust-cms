import React, { useState } from "react"
import { useAssetTable } from "./hooks/useAssetTable"
import _ from "lodash"
import { useStoreAsset } from "./hooks/useStoreAsset"
import { useTranslation } from "react-i18next"
import { AssetSaveSchema } from "./schemas/asset.save.schema"
import { SubmitHandler, useForm } from "react-hook-form"
import { joiResolver } from "@hookform/resolvers/joi"
import IAssetSave from "../../types/asset/IAssetSave"
import IAssetModel from "../../types/asset/IAssetModel"
import { AssetUploadModal } from "./AssetUploadModal"

function AssetTable() {
  const [isOpen, setIsOpen] = useState(false)
  const asset_api_table_response = useAssetTable()
  const assets: Array<IAssetModel> = _.get(
    asset_api_table_response,
    "data.data.data",
    [],
  )
  const { mutate } = useStoreAsset()
  const [t] = useTranslation("global")
  const backend_url = import.meta.env.VITE_AVORED_BACKEND_BASE_URL

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: joiResolver(AssetSaveSchema, { allowUnknown: true }),
  });

  const onCloseModal = () => {
    setIsOpen(false);
  };

  const openModal = () => {
    setIsOpen(true);
  };

  const submitHandler: SubmitHandler<IAssetSave> = (data: IAssetSave) => {
    data.file = data.file_list ? data.file_list[0] : undefined;
    onCloseModal();
    mutate(data);
  };

  return (
    <div className="flex-1 bg-white">
      <div className="pl-64">
        <div className="p-5 flex w-full items-center">
          <div className="text-primary-500 text-2xl font-semibold">
            {t("asset_table")}
          </div>
          <div className="ml-auto">
            <button
              type="button"
              onClick={openModal}
              className="bg-primary-500 rounded-md bg-black/20 px-4 py-2 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
            >
              {t("upload_asset")}
            </button>
            <button
              type="button"
              onClick={openModal}
              className="ml-3 bg-gray-400 rounded-md bg-black/20 px-4 py-2 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
            >
              {t("create_folder")}
            </button>
          </div>

          <div className="">
            <AssetUploadModal
              onCloseModal={onCloseModal}
              isOpen={isOpen}
              submitHandler={submitHandler}
              handleSubmit={handleSubmit}
              register={register}
            />
          </div>
        </div>

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
                            <img
                              src={`${backend_url}/${asset.path}`}
                              className="h-40"
                              alt={asset.name}
                            />
                          </div>
                          <div className="flex justify-center text-xs text-gray-500">
                            {asset.name} {/* Added file name to h6 */}
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
  );
}

export default AssetTable;
