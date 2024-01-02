import {useEffect, useState} from "react";
import {Link, redirect, useNavigate, useParams} from "react-router-dom";
import {isEmpty} from "lodash";
import axios from "axios";

function RoleEdit() {
    const [name, setName] = useState('Contact US update');
    const [identifier, setIdentifier] = useState('contact-us-update');
    const navigate = useNavigate()
    const params = useParams();


    useEffect(() => {
        const mounted = (async () => {
            const response = await axios({
                url: 'http://localhost:8080/api/role/' + params.role_id,
                method: 'get',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
                }
            })

            if (!response.status) {
                return
            }

            return response.data
        })

        mounted().then((res) => {
            if (isEmpty(res)) {
                localStorage. removeItem("AUTH_TOKEN")
                return navigate("/admin/login")
            }

            setName(res.role_model.name)
            setIdentifier(res.role_model.identifier)
        })

    }, [])

    const handleSubmit = (async (e) => {
        e.preventDefault()
        const updated_role_response = await axios({
            url: 'http://localhost:8080/api/role/' + params.role_id, 
            method: 'put',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
            },
            data: JSON.stringify({name: name, identifier: identifier, permissions: []})
        })
        if (updated_role_response.status) {
            return navigate("/admin/role");
        }
    })

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900 dark:text-gray-100">
                            Role Information
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
                            <div className="flex items-center">
                                <button type="submit"
                                        className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    Save
                                </button>
                                <Link to="/admin/role"
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

export default RoleEdit