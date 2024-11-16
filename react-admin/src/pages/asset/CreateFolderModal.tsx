import InputField from "../../components/InputField";
import AvoredModal from "../../components/AvoredModal";
import React from "react";
import {useTranslation} from "react-i18next";
import {SubmitHandler, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {useCreateFolderSchema} from "./schemas/create.folder.schema";
import {CreateFolderType} from "../../types/asset/CreateFolderType";
import {useStoreAsset} from "./hooks/useStoreAsset";
import {useCreateFolder} from "./hooks/useCreateFolder";

type CreateFolderModalProps = {
    isOpen: any,
    onCloseModal: any,
    parent_id?: string,
}

export const CreateFolderModal = (({
                                       isOpen,
                                       onCloseModal,
                                       parent_id
                                  }: CreateFolderModalProps) => {
    const [t] = useTranslation("global");
    const { mutate } = useCreateFolder()


    const submitHandler: SubmitHandler<CreateFolderType>  = ((data: CreateFolderType) => {
        onCloseModal()
        data.parent_id = parent_id
        mutate(data)
    })

    const {
        register,
        handleSubmit,
    } = useForm<CreateFolderType>({
        resolver: joiResolver(useCreateFolderSchema(), { allowUnknown: true }),
    });

    return (
        <>
            <div className="max-w-7xl">
                <AvoredModal
                    isOpen={isOpen}
                    closeModal={onCloseModal}
                    modal_header={t("create_folder")}
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
                                                    hover:bg-primary-500"
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