import IAssetModel from "../../types/asset/IAssetModel";
import React, { useState } from "react";
import {
  EllipsisHorizontalCircleIcon,
  FolderPlusIcon,
} from "@heroicons/react/24/outline";
import { useDeleteFolder } from "./hooks/useDeleteFolder";
import { RenameFolderModal } from "./RenameFolderModal";
import { useTranslation } from "react-i18next";
import {Link} from "react-router-dom";

type DisplayAssetProp = {
  asset: IAssetModel;
  openFolder: any
};
export const DisplayAsset = ({ asset, openFolder }: DisplayAssetProp) => {
  const [isRenameFolderModalOpen, setIsRenameFolderModalOpen] = useState(false);
  const [t] = useTranslation("global");

  const onCloseRenameFolderModal = () => {
    setIsRenameFolderModalOpen(false);
  };

  const openRenameFolderModal = () => {
    setIsRenameFolderModalOpen(true);
  };
  const backend_url = import.meta.env.VITE_AVORED_BACKEND_BASE_URL;
  const { mutate: deleteFolderMutate } = useDeleteFolder();

  const onRemoveAssetOnClick = (
    e: React.MouseEvent<HTMLAnchorElement>,
    type: string,
    asset_id: string,
  ) => {
    e.preventDefault();
    if (type === "FOLDER") {
      deleteFolderMutate({ asset_id });
    }
  };

  return (
    <>
      <div key={asset.id} className="border rounded p-3">
        <div className="mb-2 flex">
          <RenameFolderModal
            key={asset.id}
            asset={asset}
            onCloseModal={onCloseRenameFolderModal}
            isOpen={isRenameFolderModalOpen}
          />
          <div className="hs-dropdown relative ml-auto inline-flex">
            <button
              id={`hs-dropdown-folder-options-${asset.id}`}
              type="button"
              className="hs-dropdown-toggle hover:bg-gray-50"
              aria-haspopup="menu"
              aria-expanded="false"
              aria-label="Dropdown"
            >
              <EllipsisHorizontalCircleIcon className="text-gray-400 w-6 h-6" />
            </button>

            <div
              className="hs-dropdown-menu transition-[opacity,margin] duration hs-dropdown-open:opacity-100 opacity-0 hidden min-w-[38px] bg-white shadow-md rounded-lg p-1 space-y-0.5 mt-2  after:h-4 after:absolute after:-bottom-4 after:start-0 after:w-full before:h-4 before:absolute before:-top-4 before:start-0 before:w-full"
              role="menu"
              aria-orientation="vertical"
              aria-labelledby="hs-dropdown-default"
            >
              <a
                className="flex items-center gap-x-3.5 py-2 px-3 rounded-lg text-sm text-gray-800 hover:bg-gray-100 focus:outline-none focus:bg-gray-100"
                onClick={(e) =>
                  onRemoveAssetOnClick(e, asset.asset_type, asset.id)
                }
                href="#"
              >
                {t("remove")}
              </a>
              <a
                onClick={openRenameFolderModal}
                className="flex items-center gap-x-3.5 py-2 px-3 rounded-lg text-sm text-gray-800 hover:bg-gray-100 focus:outline-none focus:bg-gray-100"
                href="#"
              >
                {t("rename")}
              </a>
            </div>
          </div>
        </div>
        <div className="flex justify-center h-40 mb-3">
          {asset.asset_type === "FOLDER" ? (
            <>
              <FolderPlusIcon className="h-32 w-32 text-gray-300" />
            </>
          ) : (
            <>
              <img
                src={`${backend_url}/${asset.path}`}
                className="h-40"
                alt={asset.name}
              />
            </>
          )}
        </div>
        <div className="flex justify-center  text-xs text-slate-900">
          <div className="w-full items-center">
            {asset.asset_type === "FOLDER" ? (
              <>
                <button
                    onClick={e => openFolder(e, asset.id)}
                    className="bg-gray-100 py-2 px-1 rounded w-full hover:bg-gray-200" type="button">
                  <Link to={`/admin/asset/${asset.id}`}>
                    {asset.name}
                  </Link>
                </button>
              </>
            ) : (
              <>
                <div className="text-ellipsis w-full overflow-hidden bg-gray-100 py-2 px-1 rounded">

                {/** ADD COPY ICON AND Allow them to copy the file name **/}
                {asset.name}
                </div>
              </>
            )}
          </div>
        </div>
      </div>
    </>
  );
};
