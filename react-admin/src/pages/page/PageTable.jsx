import {useEffect, useState} from "react";
import {Link, redirect} from "react-router-dom";
import {isEmpty} from "lodash";

function PageTable() {
    const [pages, setPages] = useState([]);

    const getFormattedDate = ((date) => {
        var date = new Date(date);

        return `${date.getFullYear()}-${date.getMonth()}-${date.getDate()}`;
    })

    useEffect(() => {
        const mounted = (async () => {
            const response = await fetch('http://localhost:8080/api/page', {
                method: 'get',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
                }
            })
            console.log(response.ok)
            if (!response.ok) {

                return
            }
            return await response.json()
        })

        mounted().then((res) => {
            if (isEmpty(res)) {
                localStorage.removeItem("AUTH_TOKEN")
                alert("please go to login page manually till we fix the issue.")
                return redirect("/admin/login")
            }
            setPages(res.data)
        })

    }, [])

    return (
        <div className="flex-1 pl-64 bg-white">
            <div className="px-5">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        Pages
                    </div>
                    <Link className="ml-auto bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                        to="/admin/page/create">
                        Create
                    </Link>
                </div>


                <div className="overflow-x-auto">
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
                        {pages.map((page) => {
                            return (
                                <tr className="border-b">
                                    <td className="py-3 px-4">{page.id}</td>
                                    <td className="py-3 px-4">{page.name}</td>
                                    <td className="py-3 px-4">{page.identifier}</td>
                                    <td className="py-3 px-4">
                                        {getFormattedDate(page.created_at)}
                                    </td>
                                    <td className="py-3 px-4">
                                        {getFormattedDate(page.updated_at)}
                                    </td>
                                    <td className="py-3 px-4">{page.created_by}</td>
                                    <td className="py-3 px-4">{page.updated_by}</td>
                                    <td className="py-3 px-4">
                                        <Link className="font-medium text-primary-600 hover:text-primary-800"
                                              to={`/admin/page/${page.id}`}>
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

export default PageTable