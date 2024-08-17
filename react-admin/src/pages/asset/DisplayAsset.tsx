import IAssetModel from "../../types/asset/IAssetModel";
import React from "react";
import {EllipsisHorizontalCircleIcon, FolderPlusIcon} from "@heroicons/react/24/outline";

type DisplayAssetProp = {
  asset: IAssetModel;
};
export const DisplayAsset = ({ asset }: DisplayAssetProp) => {
  const backend_url = import.meta.env.VITE_AVORED_BACKEND_BASE_URL;


  return (
    <>
      <div key={asset.id} className="border rounded p-3">
          <div className="mb-2 flex">
              <div className="hs-dropdown relative ml-auto inline-flex">
                  <button id={`hs-dropdown-folder-options-${asset.id}`} type="button"
                          className="hs-dropdown-toggle hover:bg-gray-50"
                          aria-haspopup="menu" aria-expanded="false" aria-label="Dropdown">
                      <EllipsisHorizontalCircleIcon className="text-gray-400 w-6 h-6"/>
                  </button>

                  <div
                      className="hs-dropdown-menu transition-[opacity,margin] duration hs-dropdown-open:opacity-100 opacity-0 hidden min-w-[38px] bg-white shadow-md rounded-lg p-1 space-y-0.5 mt-2  after:h-4 after:absolute after:-bottom-4 after:start-0 after:w-full before:h-4 before:absolute before:-top-4 before:start-0 before:w-full"
                      role="menu" aria-orientation="vertical" aria-labelledby="hs-dropdown-default">
                      <a className="flex items-center gap-x-3.5 py-2 px-3 rounded-lg text-sm text-gray-800 hover:bg-gray-100 focus:outline-none focus:bg-gray-100"
                         href="#">
                          Remove
                      </a>
                      <a className="flex items-center gap-x-3.5 py-2 px-3 rounded-lg text-sm text-gray-800 hover:bg-gray-100 focus:outline-none focus:bg-gray-100"
                         href="#">
                          Rename
                      </a>

                  </div>
              </div>

          </div>
          <div className="flex justify-center h-40 mb-3">
              {asset.asset_type === "FOLDER" ? (
                  <>
                      <FolderPlusIcon className="h-32 w-32 text-gray-300"/>
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
          <div className="flex justify-center bg-gray-100 text-xs text-slate-900 py-2 px-1 rounded">
              <div className="">
                  {asset.name}

                  {/** ADD COPY ICON AND Allow them to copy the file name **/}
              </div>
          </div>
      </div>
    </>
  );
};
