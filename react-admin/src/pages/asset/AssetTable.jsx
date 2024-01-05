import AvoredModal from "../../components/AvoredModal";
import React, {useEffect, useState} from "react";
import axios from "axios";
import {isEmpty} from "lodash";
import {useNavigate} from "react-router-dom";
import InputField from "../../components/InputField";

function AssetTable() {
    const [isOpen, setIsOpen] = useState(false)
    const [assets, setAssets] = useState([])
    const navigate = useNavigate()
    const [file, setFile] = useState()

    const onCloseModal = (() => {
        setIsOpen(false)
    })

    const openModal = (() => {
        setIsOpen(true)
    })

    const saveAsset = (async (e) => {
        e.preventDefault()
        var formData = new FormData()

        formData.append('file', file)

        const created_asset_response = (await axios({
            url: 'http://localhost:8080/api/asset',
            method: 'POST',
            headers: {
                'Content-Type': 'multipart/form-data; boundary=----',
                'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
            },
            data: formData
        }))
        console.log(window.x = created_asset_response, created_asset_response.data.data)
        if (!created_asset_response.data.success) {
            alert("asset did not uploaded. Please try again")
        }
        onCloseModal()
        fetchAssets()
    })
    const handleAssetChange = ((e) => {
        const file = e.target.files[0];
        setFile(file)
    });

    useEffect(() => {
        fetchAssets()
    }, [])

    const fetchAssets = (() => {
        const mounted = (async () => {

            const response = await axios({
                url: 'http://localhost:8080/api/asset',
                method: 'get',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
                }
            })
            return response
        })

        mounted().then((res) => {
            setAssets(res.data.data)
        }).catch((error) => {
            localStorage.removeItem("AUTH_TOKEN")
            return navigate("/admin/login")
        })
    })

    return (
        <div className="flex-1 bg-white">
            <div className="pl-64">
                <div className="p-5 flex w-full items-center">
                    <div className="text-primary-500 text-2xl font-semibold">
                        Asset table
                    </div>

                    <button
                        type="button"
                        onClick={openModal}
                        className="ml-auto bg-primary-500 rounded-md bg-black/20 px-4 py-2 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
                    >
                        Create
                    </button>
                    <AvoredModal
                        isOpen={isOpen}
                        closeModal={onCloseModal}
                        modal_header="Upload Asset"
                        modal_body={
                            <div className="text-sm text-gray-500">
                                <div className="text-sm text-gray-500 rounded">
                                    <div className="py-5">

                                        <div className="flex">
                                            <div className="mt-3">
                                                File
                                                <div className="mt-1">
                                                    <InputField
                                                        label="Asset File"
                                                        type="file"
                                                        name="file"
                                                        onChange={handleAssetChange}
                                                    />
                                                </div>
                                            </div>

                                        </div>

                                        <div className="flex flex-row mt-6 space-x-2 justify-evenly">
                                            <a href="#"
                                                onClick={saveAsset}
                                                className="w-full py-3 text-sm font-medium text-center text-white transition
                                                duration-150 ease-linear bg-red-600 border border-red-600 rounded-lg
                                                hover:bg-red-500">
                                                Upload
                                            </a>
                                            <a href="#" onClick={onCloseModal}
                                                className="w-full py-3 text-sm text-center text-gray-500 transition duration-150
                                                ease-linear bg-white border border-gray-200 rounded-lg hover:bg-gray-100">
                                                Cancel
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }/>
                </div>

                <div className="mt-5">
                    <div className="px-4 mx-auto">
                        <div className="flex flex-col mt-6">
                            <div className="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                                <div className="inline-block min-w-full p-2">
                                    <div className="grid grid-cols-4 gap-4 mx-5">
                                        {assets.map((asset) => {
                                            return (
                                                <div key={asset.id} className="border rounded p-3">
                                                    <div className="h-32 mb-3">
                                                        <img src={`http://localhost:8080/${asset.file_path}`}
                                                             className="h-32" alt={asset.file_name}/>
                                                    </div>
                                                    <h6 className="text-sm font-semibold">

                                                    </h6>
                                                </div>
                                            )
                                        })}


                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default AssetTable