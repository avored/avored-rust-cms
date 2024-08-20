import InputField from "../../components/InputField";
import AvoredModal from "../../components/AvoredModal";
import React from "react";
import {useTranslation} from "react-i18next";
import {SubmitHandler, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {useCreateFolder} from "./hooks/useCreateFolder";
import {useRenameFolderSchema} from "./schemas/rename.folder.schema";
import IAssetModel from "../../types/asset/IAssetModel";
import {RenameFolderType} from "../../types/asset/RenameFolderType";
import {useRenameFolder} from "./hooks/useRenameFolder";

type RenameFolderModalProps = {
    isOpen: any,
    onCloseModal: any,
    asset: IAssetModel
}

export const RenameFolderModal = (({
                                       isOpen,
                                       onCloseModal,
                                       asset
                                  }: RenameFolderModalProps) => {
    const [t] = useTranslation("global");
    const { mutate } = useRenameFolder()


    const submitHandler: SubmitHandler<RenameFolderType>  = ((data: RenameFolderType) => {
        onCloseModal()
        mutate(data)
    })

    const {
        register,
        handleSubmit,
    } = useForm<RenameFolderType>({
        resolver: joiResolver(useRenameFolderSchema(), { allowUnknown: true }),
        values: asset
    });

    return (
        <>
            <div className="max-w-7xl">
                <AvoredModal
                    isOpen={isOpen}
                    closeModal={onCloseModal}
                    modal_header={t("rename_folder")}
                    modal_body={
                        <div className="text-sm text-gray-500">
                            <div className="text-sm text-gray-500 rounded">
                                <form onSubmit={handleSubmit(submitHandler)}>
                                    <div className="">
                                        <div className="flex">
                                            <div className="w-full mt-3">

                                                    <InputField
                                                        label={t("folder_name")}
                                                        type="text"
                                                        name="name"
                                                        register={register('name')}
                                                    />

                                            </div>
                                        </div>

                                        <div className="flex pt-5 mt-6 space-x-2">
                                            <button
                                                type="submit"
                                                className="px-4 py-2 text-sm font-medium text-center text-white transition
                                                    duration-150 ease-linear bg-primary-600 border border-primary-600 rounded-lg
                                                    hover:bg-red-500"
                                            >
                                                {t("create")}
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