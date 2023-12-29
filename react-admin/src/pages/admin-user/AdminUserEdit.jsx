import {useEffect, useState} from "react";
import {Link, redirect, useNavigate, useParams} from "react-router-dom";
import {isEmpty} from "lodash";
import InputField from "../../components/InputField";
import {Switch} from "@headlessui/react";

function AdminUserEdit() {
    const [full_name, setFullName] = useState();
    const [is_super_admin, setIsSuperAdmin] = useState(false)
    const navigate = useNavigate()
    const params = useParams();

    useEffect(() => {
        const mounted = (async () => {
            const response = await fetch('http://localhost:8080/api/admin-user/' + params.admin_user_id, {
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
                return navigate("/admin/login")
            }

            setFullName(res.admin_user_model.full_name)
            setIsSuperAdmin(res.admin_user_model.is_super_admin)
        })

    }, [])

    const handleSubmit = (async (e) => {
        e.preventDefault()
        const response = (await fetch('http://localhost:8080/api/admin-user/' + params.admin_user_id, {
            method: 'put',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
            },
            body: JSON.stringify({full_name: full_name, is_super_admin: is_super_admin})
        }))
        const updated_page_response = await response.json()
        if (updated_page_response.status) {
            return navigate("/admin/admin-user");
        }
    })

    return (
        <div className="flex-1 pl-64 bg-white">
            <div className="px-5">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900 dark:text-gray-100">
                            Admin User Information
                        </h1>
                        {/*<p className="text-gray-600 dark:text-gray-300 mb-6">Use a permanent address where you can*/}
                        {/*    receive mail.</p>*/}
                        <form onSubmit={handleSubmit}>
                            <div className="mb-4">
                                <InputField
                                    label="Full Name"
                                    type="text"
                                    name="full_name"
                                    value={full_name}
                                    onChange={(e) => setFullName(e.target.value)}
                                    autoFocus
                                />
                            </div>
                            <div className="mb-4">
                                <Switch
                                    checked={is_super_admin}
                                    onChange={setIsSuperAdmin}
                                    className={`${
                                        is_super_admin ? 'bg-primary-500' : 'bg-gray-200'
                                    } relative inline-flex h-6 w-11 items-center rounded-full`}
                                >
                                    <span className="sr-only">Enable notifications</span>
                                    <span
                                        className={`${
                                            is_super_admin ? 'translate-x-6' : 'translate-x-1'
                                        } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                                    />
                                </Switch>
                            </div>
                            <div className="flex items-center">
                                <button type="submit"
                                        className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    Save
                                </button>
                                <Link to="/admin/admin-user"
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

export default AdminUserEdit