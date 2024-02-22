import {useEffect, useState} from "react";
import {Link, useNavigate} from "react-router-dom";
import apiClient from "../../ApiClient";

function ComponentTable() {
    const [components, setComponents] = useState([]);
    const navigate = useNavigate()

    const getFormattedDate = ((date) => {
        var date_obj = new Date(date);

        return `${date_obj.getFullYear()}-${date_obj.getMonth() + 1}-${date_obj.getDate()}`;
    })



    useEffect(() => {
        const mounted = (async () => {

            return await apiClient({
                url: '/component',
                method: 'get',
                headers: {
                    'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
                }
            })
        })

        mounted().then(({data}) => {
            setComponents(data.data)
        }).catch((errors) => {
            if (errors.response.status === 401) {
                localStorage.removeItem("AUTH_TOKEN")
                return navigate("/admin/login")
            }
        })

    }, [navigate])

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 ml-64">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        Components
                    </div>
                    <Link className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                        to="/admin/component-create">
                        Create
                    </Link>
                </div>


                <div className="overflow-x-hidden">
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
                                        <Link className="font-medium text-primary-600 hover:text-primary-800"
                                              to={`/admin/component-edit/${component.id}`}>
                                            Edit
                                        </Link>

                                    </td>
                                </tr>
                            )
                        })}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    )
}

export default ComponentTable