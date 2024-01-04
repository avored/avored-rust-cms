import AvoredModal from "../../components/AvoredModal";
import React, {useState} from "react";

function AssetTable() {
    const [isOpen, setIsOpen] = useState(false)

    const onClose = (() => {
        setIsOpen(false)
    });
    function openModal() {
        setIsOpen(true)
    }

    return (
        <div className="flex-1 bg-white">
            <div className="pl-64">
                <div className="p-5">
                    Asset table
                    <button
                        type="button"
                        onClick={openModal}
                        className="rounded-md bg-black/20 px-4 py-2 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
                    >
                        Create
                    </button>
                    <AvoredModal isOpen={isOpen} closeModal={onClose} body={
                        <p className="text-sm text-gray-500">
                            parent Your payment has been successfully submitted. We’ve sent
                            you an email with all of the details of your order.
                        </p>
                    }/>
                </div>

            </div>
        </div>
    )
}

export default AssetTable