import InputField from "../../components/InputField";
import AvoredModal from "../../components/AvoredModal";
import React from "react";
import {useTranslation} from "react-i18next";

type AssetUploadModalProps = {
    isOpen: any,
    onCloseModal: any,
    handleSubmit: any,
    submitHandler: any,
    register: any
}

export const AssetUploadModal = (({
    isOpen,
    onCloseModal,
    handleSubmit,
    submitHandler,
    register
}: AssetUploadModalProps) => {
    const [t] = useTranslation("global");
    return (
        <>
            <div className="max-w-7xl">
                <AvoredModal
                    isOpen={isOpen}
                    closeModal={onCloseModal}
                    modal_header={t("upload_asset")}
                    modal_body={
                        <div className="text-sm text-gray-500">
                            <div className="text-sm text-gray-500 rounded">
                                <form onSubmit={handleSubmit(submitHandler)}>
                                    <div className="">
                                        <div className="flex">
                                            <div className="mt-3">
                                                <div className="mt-1">
                                                    <InputField
                                                        label={t("asset_file")}
                                                        type="file"
                                                        name="file_list"
                                                        register={register('file_list')}
                                                    />
                                                </div>
                                            </div>
                                        </div>

                                        <div className="flex pt-5 mt-6 space-x-2">
                                            <button
                                                type="submit"
                                                className="px-4 py-2 text-sm font-medium text-center text-white transition
                                                    duration-150 ease-linear bg-primary-600 border border-primary-600 rounded-lg
                                                    hover:bg-primary-500"
                                            >
                                                {t("upload")}
                                            </button>
                                            <button
                                                type="button"
                                                onClick={onCloseModal}
                                                className="py-2 px-4 text-sm text-center text-gray-600 transition duration-150
                                                    ease-linear bg-white border border-gray-400 rounded-lg hover:bg-gray-200"
                                            >
                                                {t("cancel")}
                                            </button>
                                        </div>
                                    </div>
                                </form>
                            </div>
                        </div>
                    }
                />
            </div>

        </>
    )
})