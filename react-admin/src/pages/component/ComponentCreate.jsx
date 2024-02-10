import {useState} from "react"
import {Link, useNavigate} from "react-router-dom"
import {PlusIcon} from "@heroicons/react/24/solid"
import apiClient from "../../ApiClient"
import {TrashIcon} from "@heroicons/react/16/solid"
import InputField from "../../components/InputField";

function ComponentCreate() {
    const [fields, setFields] = useState([])
    const [name, setName] = useState('Component 2')
    const [identifier, setIdentifier] = useState('component-2')
    const navigate = useNavigate()

    const addFieldOnClick = (() => {

        var field = {id: Math.random(), field_type: 'text', name: '', identifier: ''};

        setFields(fields => [...fields, field])
    })

    const deleteFieldOnClick = ((fieldIndex) => {
        var array = [...fields]; // make a separate copy of the array

        array = array.filter((ele) => ele.id !== fieldIndex)
        setFields(array);
    })

    const fieldTypeButtonOnClick = ((fieldIndex, fieldTypeValue) => {
        setFields(prevFields => {
            return prevFields.map(field => {
                if (field.id === fieldIndex) {
                    field.field_type = fieldTypeValue
                    return field
                }
                return field
            })
        })
    })

    const fieldNameChange = ((fieldIndex, fieldValue) => {
        setFields(prevFields => {
            return prevFields.map(field => {
                if (field.id === fieldIndex) {
                    field.name = fieldValue.target.value
                    return field
                }
                return field
            })
        })
    })

    const fieldIdentifierChange = ((fieldIndex, fieldValue) => {
        setFields(prevFields => {
            return prevFields.map(field => {
                if (field.id === fieldIndex) {
                    field.identifier = fieldValue.target.value
                    return field
                }
                return field
            })
        })
    })

    const handleSubmit = (async (e) => {
        e.preventDefault()

        const created_page_response = await apiClient({
            url: '/component',
            method: 'POST',
            headers: {
                'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
            },
            data: JSON.stringify({name: name, identifier: identifier, fields: fields})
        })

        if (created_page_response.status) {
            return navigate("/admin/component");
        }
    })

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64 ">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900 dark:text-gray-100">
                            Component Information
                            {JSON.stringify(fields)}
                        </h1>
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

                            {fields.map((field) => {
                                return (
                                    <div key={field.id} className="block ring-1 ring-gray-400 mb-4 rounded p-5">
                                        <div className="flex w-full">
                                            <button type="button"
                                                    onClick={() => {deleteFieldOnClick(field.id)}}
                                                    className="ml-auto">
                                                <TrashIcon className="w-4 h-4" />
                                            </button>
                                        </div>

                                        <div className="flex w-full">
                                            <div className="border-r p-5 w-1/3">
                                                <div
                                                    onClick={e => fieldTypeButtonOnClick(field.id ,'text')}
                                                    className={`${field.field_type === 'text' ? 'bg-primary-300' : 'bg-gray-300'} ring-1 p-3 mt-3 rounded`}
                                                >
                                                    Text
                                                </div>
                                                <div
                                                    onClick={e => fieldTypeButtonOnClick(field.id ,'textarea')}
                                                    className={`${field.field_type === 'textarea' ? 'bg-primary-300' : 'bg-gray-300'} ring-1 p-3 mt-3 rounded`}
                                                >
                                                    Textarea
                                                </div>
                                            </div>

                                            <div className="p-3 w-2/3">
                                                <div>
                                                    Field Type: {field.field_type}
                                                </div>
                                                <div className="mt-3">
                                                    <InputField
                                                        label="Field Name"
                                                        type="text"
                                                        onChange={e => fieldNameChange(field.id, e)}
                                                    />
                                                </div>

                                                <div className="mt-3">
                                                    <InputField
                                                        label="Field Identifier"
                                                        type="text"
                                                        onChange={e => fieldIdentifierChange(field.id, e)}
                                                    />
                                                </div>
                                            </div>
                                        </div>

                                    </div>
                                )
                            })}

                            <div className="mb-4 flex items-center justify-center ring-1 ring-gray-400 rounded p-5">
                                <button type="button" className="flex" onClick={addFieldOnClick}>
                                    <PlusIcon className="text-primary-500 h-6 w-6" />
                                    <span className="text-sm ml-1 text-primary-500">
                                        Add Field
                                    </span>
                                </button>

                            </div>
                                <div className="flex items-center">
                                    <button type="submit"
                                            className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                    >
                                        Save
                                    </button>
                                    <Link to="/admin/component"
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

export default ComponentCreate