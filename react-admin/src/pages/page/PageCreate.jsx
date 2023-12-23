import {useState} from "react";
import {Link, redirect} from "react-router-dom";

function PageCreate() {
    const [name, setName] = useState('Contact US');
    const [identifier, setIdentifier] = useState('contact-us');

    const handleSubmit = (async (e) => {
        e.preventDefault()
        const response = (await fetch('http://localhost:8080/api/page', {
            method: 'post',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
            },
            body: JSON.stringify({name: name, identifier: identifier})
        }))
        const created_page_response = await response.json()
        if (created_page_response.status) {
            return redirect("/admin/page");
        }
    })

    return (
        <div className="flex-1 pl-64 bg-white">
            <div className="px-5">
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