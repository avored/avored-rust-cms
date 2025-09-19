import InputField from "../../components/InputField";
import AvoredModal from "../../components/AvoredModal";
import {useTranslation} from "react-i18next";
import {SubmitHandler, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {UseRenameAssetHook} from "../../hooks/asset/UseRenameAssetHook";
import {AssetType, RenameFolderType} from "../../types/asset/AssetType";
import {UseRenameFolderSchema} from "../../schemas/asset/UseRenameFolderSchema";
import {RenameAssetRequest} from "grpc-avored/asset_pb";

type RenameAssetModalProps = {
    isOpen: any,
    onCloseModal: any,
    asset: AssetType,
}

export const RenameAssetModal = (({
                                       isOpen,
                                       onCloseModal,
                                       asset
                                  }: RenameAssetModalProps) => {
    const [t] = useTranslation("global");
    const { mutate } = UseRenameAssetHook()


    const submitHandler: SubmitHandler<RenameFolderType>  = ((data: RenameFolderType) => {
        onCloseModal()
        const request = new RenameAssetRequest();
        request.setAssetId(asset.id)
        request.setName(data.name)
        mutate(request)
    })

    const {
        register,
        handleSubmit,
    } = useForm<RenameFolderType>({
        resolver: joiResolver(UseRenameFolderSchema, { allowUnknown: true }),
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
                                                {t("rename")}
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
