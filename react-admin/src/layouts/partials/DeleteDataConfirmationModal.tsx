import { Button, Dialog, DialogPanel, DialogTitle } from "@headlessui/react";
import {useTranslation} from "react-i18next";
import {useDeleteDemoData} from "../../hooks/useDeleteDemoData";

type DeleteDataConfirmationModalProp = {
    isOpen: any,
    close: any
}
export const DeleteDataConfirmationModal = (({
    isOpen,
    close
}: DeleteDataConfirmationModalProp) => {
    const [t] = useTranslation("global")
    const { mutate } = useDeleteDemoData()
    const onConfirmButtonOnClick = (() => {
        mutate()
        close()
    })
    return (
        <Dialog open={isOpen} as="div" className="relative z-30 focus:outline-none" onClose={close}>
            <div className="fixed inset-0 z-30 w-screen overflow-y-auto">
                <div className="flex min-h-full items-center justify-center p-4">
                    <DialogPanel
                        className="w-full max-w-md rounded-xl bg-gray-800 p-6 duration-300 ease-out data-[closed]:transform-[scale(95%)] data-[closed]:opacity-0"
                    >
                        <DialogTitle as="h3" className="text-base/7 font-medium text-white">
                            {t("are_you_sure")}
                        </DialogTitle>
                        <p className="mt-2 text-sm/6 text-white/50">
                            {t("delete_demo_data_description")}
                        </p>
                        <div className="mt-4">
                            <Button
                                className="inline-flex items-center gap-2 rounded-md bg-gray-700 py-1.5 px-3 text-sm/6 font-semibold text-white shadow-inner shadow-white/10 focus:outline-none data-[hover]:bg-gray-600 data-[focus]:outline-1 data-[focus]:outline-white data-[open]:bg-gray-700"
                                onClick={onConfirmButtonOnClick}
                            >
                                {t("confirm")}
                            </Button>
                            <Button
                                className="ml-3 inline-flex items-center gap-2 rounded-md bg-gray-700 py-1.5 px-3 text-sm/6 font-semibold text-white shadow-inner shadow-white/10 focus:outline-none data-[hover]:bg-gray-600 data-[focus]:outline-1 data-[focus]:outline-white data-[open]:bg-gray-700"
                                onClick={close}
                            >
                                {t("no")}
                            </Button>
                        </div>
                    </DialogPanel>
                </div>
            </div>
        </Dialog>
    )
})