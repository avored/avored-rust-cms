import AvoredModal from "../../components/AvoredModal";
import React, { useState, ChangeEvent, FormEvent } from "react";
import InputField from "../../components/InputField";
import { useAssetTable } from "./hooks/useAssetTable";
import _ from "lodash";
import { useStoreAsset } from "./hooks/useStoreAsset";
import { useTranslation } from "react-i18next";

const AssetTable: React.FC = () => {
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [file, setFile] = useState<File | null>(null);
  const asset_api_table_response = useAssetTable();
  const assets = _.get(asset_api_table_response, "data.data.data", []);
  const { mutate } = useStoreAsset();
  const [t] = useTranslation("global");

  const onCloseModal = () => {
    setIsOpen(false);
  };

  const openModal = () => {
    setIsOpen(true);
  };

  const saveAsset = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!file) return;

    // Assuming mutate accepts a FormData object
    const formData = new FormData();
    formData.append("file", file);

    mutate({ file }); // Pass the file object to mutate
    onCloseModal();

    // todo invalid asset table here
  };

  const handleAssetChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      setFile(file);
    }
  };

  return (
    <div className="flex-1 bg-white">
      <div className="pl-64">
        <div className="p-5 flex w-full items-center">
          <div className="text-primary-500 text-2xl font-semibold">
            {t("asset.asset_table")}
          </div>

          <button
            type="button"
            onClick={openModal}
            className="ml-auto bg-primary-500 rounded-md bg-black/20 px-4 py-2 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
          >
            {t("common.create")}
          </button>
          <AvoredModal
            isOpen={isOpen}
            closeModal={onCloseModal}
            modal_header={t("asset.upload_asset")}
            modal_body={
              <div className="text-sm text-gray-500">
                <div className="text-sm text-gray-500 rounded">
                  <div className="py-5">
                    <div className="flex">
                      <div className="mt-3">
                        {t("common.file")}
                        <div className="mt-1">
                          <InputField
                            label={t("asset.asset_file")}
                            type="file"
                            name="file"
                            onChange={handleAssetChange}
                            value={""}
                            errorMessages={[]}
                          />
                        </div>
                      </div>
                    </div>
                    <form onSubmit={saveAsset}>
                      <div className="flex flex-row mt-6 space-x-2 justify-evenly">
                        <button
                          type="submit"
                          className="w-full py-3 text-sm font-medium text-center text-white transition
                            duration-150 ease-linear bg-red-600 border border-red-600 rounded-lg
                            hover:bg-red-500"
                        >
                          {t("common.upload")}
                        </button>
                        <button
                          type="button"
                          onClick={onCloseModal}
                          className="w-full py-3 text-sm text-center text-gray-500 transition duration-150
                            ease-linear bg-white border border-gray-200 rounded-lg hover:bg-gray-100"
                        >
                          {t("common.cancel")}
                        </button>
                      </div>
                    </form>
                  </div>
                </div>
              </div>
            }
          />
        </div>

        <div className="mt-5">
          <div className="px-4 mx-auto">
            <div className="flex flex-col mt-6">
              <div className="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                <div className="inline-block min-w-full p-2">
                  <div className="grid grid-cols-6 gap-4 mx-5">
                    {assets.map(
                      (asset: {
                        id: React.Key | null | undefined;
                        file_path: any;
                        file_name: any;
                      }) => {
                        return (
                          <div key={asset.id} className="border rounded p-3">
                            <div className="h-32 mb-3">
                              <img
                                src={`http://localhost:8080/${asset.file_path}`}
                                className="h-32"
                                alt={asset.file_name}
                              />
                            </div>
                            <h6 className="text-sm font-semibold">
                              {asset.file_name} {/* Added file name to h6 */}
                            </h6>
                          </div>
                        );
                      }
                    )}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AssetTable;
