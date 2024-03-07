import {useEffect, useState} from "react";
import {Link, redirect, useNavigate} from "react-router-dom";
import {PlusIcon} from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";
import apiClient from "../../ApiClient";
import InputField from "../../components/InputField";

function PageCreate() {
    const [name, setName] = useState('Contact US')
    const [isComponentTableModalOpen, setIsComponentTableModalOpen] = useState(false)
    const [identifier, setIdentifier] = useState('contact-us');
    const navigate = useNavigate()
    const [components, setComponents] = useState([])
    const [pageComponents, setPageComponents] = useState([])


    const getFormattedDate = ((date) => {
        var date_obj = new Date(date);

        return `${date_obj.getFullYear()}-${date_obj.getMonth() + 1}-${date_obj.getDate()}`;
    })

    const renderComponentFieldType = ((componentField) => {
        switch (componentField.field_type) {
            case 'text':
                return (
                    <div>

                        <InputField
                            label={componentField.name}
                            type="text"
                            name={componentField.identifier}
                        />
                    </div>
                )
            case 'textarea':
                return (
                    <div>
                        Textarea
                    </div>
                )
        }
    })

    const renderComponentField = ((componentField) => {
        return (
            <div className="ring-1 my-2 ring-gray-200" key={componentField.id}>
                {/*<div>*/}
                {/*    component field name: {componentField.name}*/}
                {/*</div>*/}
                {/*<div>*/}
                {/*    component field identifier: {componentField.identifier}*/}
                {/*</div>*/}
                {/*<div>*/}
                {/*    component field type: {componentField.field_type}*/}
                {/*</div>*/}
                {renderComponentFieldType(componentField)}
            </div>
        )
    })

    const componentSelected = ((e, componentId) => {
        e.preventDefault()
        const selectedComponent = components.find((component) => component.id === componentId)

        setIsComponentTableModalOpen(false)

        setPageComponents(pageComponents => [...pageComponents, selectedComponent])

    })


    const addComponentOnClick = (() => {
        setIsComponentTableModalOpen(true)
    })

    const pageModelOnClose = (() => {
        setIsComponentTableModalOpen(false)
    })

    const renderComponent = ((pageComponent) => {
        return (
            <div key={pageComponent.id} className="my-5 ring-1 ring-gray-200 rounded p-3">
                <div>
                    component name: {pageComponent.name}
                </div>
                <div>
                    component identifier: {pageComponent.identifier}
                </div>

                Component Fields
                {pageComponent.fields.map((componentField) => {
                    return renderComponentField(componentField)
                })}
                <div>

                </div>

            </div>
        )
    })


    useEffect(() => {
        const mounted = (async () => {

            return await apiClient({
                url: '/component-all',
                method: 'get',
                headers: {
                    'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
                }
            })
        })

        mounted().then(({data}) => {
            setComponents(data)
        }).catch((errors) => {
            if (errors.response.status === 401) {
                localStorage.removeItem("AUTH_TOKEN")
                return navigate("/admin/login")
            }
        })

    }, [navigate])

    const handleSubmit = (async (e) => {
        e.preventDefault()
        // const response = (await fetch('http://localhost:8080/api/page', {
        //     method: 'post',
        //     headers: {
        //         'Content-Type': 'application/json',
        //         'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
        //     },
        //     body: JSON.stringify({name: name, identifier: identifier, component_content: pageComponents})
        // }))
        const created_page_response = await apiClient({
            url: '/page',
            method: 'POST',
            headers: {
                'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
            },
            data: JSON.stringify({name: name, identifier: identifier, component_content: pageComponents})
        })

        if (created_page_response.status) {
            return navigate("/admin/page");
        }
    })

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64 ">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900 dark:text-gray-100">
                            Page Information
                        </h1>
                        {/*<p className="text-gray-600 dark:text-gray-300 mb-6">Use a permanent address where you can*/}
                        {/*    receive mail.</p>*/}
                        <form onSubmit={handleSubmit}>
                            <div className="mb-4">
                                <input type="text" placeholder="Name"
                                       value={name}
                                       onChange={e => setName(e.target.value)}
                                       className="border p-2 rounded w-full"/>
                            </div>
                            <div className="mb-4">
                                <input type="text"
                                       placeholder="Identifier"
                                       value={identifier}
                                       onChange={e => setIdentifier(e.target.value)}
                                       className="border p-2 rounded w-full"/>
                            </div>

                            <div>
                                {pageComponents.map((pageComponent) => {
                                    return renderComponent(pageComponent)
                                })}
                            </div>

                            <div className="mb-4 flex items-center justify-center ring-1 ring-gray-400 rounded p-5">
                                <button type="button" className="flex" onClick={addComponentOnClick}>
                                    <PlusIcon className="text-primary-500 h-6 w-6"/>
                                    <span className="text-sm ml-1 text-primary-500">
                                        Add Component
                                    </span>
                                </button>
                            </div>

                            <AvoredModal
                                closeModal={pageModelOnClose}
                                modal_header="Select Component"
                                modal_body={(
                                    <div className="text-primary-500">
                                        <table className="min-w-full bg-white shadow-md rounded">
                                            <thead>
                                            <tr className="bg-gray-700 text-white">
                                                <th className="py-3 px-4 rounded-l font-semibold text-left">ID</th>
                                                <th className="py-3 px-4 font-semibol text-left">Name</th>
                                                <th className="py-3 px-4 font-semibol text-left">Identifier</th>
                                                <th className="py-3 px-4 font-semibol text-left">Created at</th>
                                                <th className="py-3 px-4 font-semibol text-left">Updated at</th>
                                                <th className="py-3 px-4 font-semibol text-left">Created by</th>
                                                <th className="py-3 px-4 font-semibol text-left">Updated by</th>
                                                <th className="py-3 px-4 rounded-r font-semibol text-left">Action</th>
                                            </tr>
                                            </thead>
                                            <tbody className="">
                                            {components.map((component) => {
                                                return (
                                                    <tr key={component.id} className="border-b">
                                                        <td className="py-3 px-4">{component.id}</td>
                                                        <td className="py-3 px-4">{component.name}</td>
                                                        <td className="py-3 px-4">{component.identifier}</td>
                                                        <td className="py-3 px-4">
                                                            {getFormattedDate(component.created_at)}
                                                        </td>
                                                        <td className="py-3 px-4">
                                                            {getFormattedDate(component.updated_at)}
                                                        </td>
                                                        <td className="py-3 px-4">{component.created_by}</td>
                                                        <td className="py-3 px-4">{component.updated_by}</td>
                                                        <td className="py-3 px-4">
                                                            <button type="button"
                                                                className="font-medium text-primary-600 hover:text-primary-800"
                                                                onClick={e => componentSelected(e, component.id)}
                                                            >
                                                                Select
                                                            </button>

                                                        </td>
                                                    </tr>
                                                )
                                            })}
                                            </tbody>
                                        </table>
                                    </div>
                                )}
                                isOpen={isComponentTableModalOpen}
                            ></AvoredModal>


                            <div className="flex items-center">
                                <button type="submit"
                                        className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    Save
                                </button>
                                <Link to="/admin/page"
                                      className="ml-auto font-medium text-gray-600 hover:text-gray-500">
                                    Cancel
                                </Link>
                            </div>

                        </form>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default PageCreate