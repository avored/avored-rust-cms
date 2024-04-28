import {useMemo, useState} from "react"
import {Link, useParams} from "react-router-dom"
import InputField from "../../components/InputField"
import {Switch} from "@headlessui/react"
import AvoRedMultiSelectField from "../../components/AvoRedMultiSelectField"
import _ from "lodash"
import {useGetAdminUser} from "./hooks/useGetAdminUser"
import {useGetRoleOptions} from "./hooks/useGetRoleOptions"
import {useUpdateAdminUser} from "./hooks/useUpdateAdminUser"
function AdminUserEdit() {

    const [selectedOption, setSelectedOption] = useState([])
    const [full_name, setFullName] = useState()
    const [current_profile_image, setCurrentProfileImage] = useState()
    const [is_super_admin, setIsSuperAdmin] = useState(false)
    const [image, setImage] = useState()
    const params = useParams();

    const {data} = useGetAdminUser(params.admin_user_id)
    useMemo(() => {
        setFullName(_.get(data, 'data.admin_user_model.full_name'))

        setIsSuperAdmin(_.get(data, 'data.admin_user_model.is_super_admin'))
        setCurrentProfileImage(_.get(data, 'data.admin_user_model.profile_image'))
        var role_ids = []
        _.get(data, 'data.admin_user_model.roles', []).forEach((role) => {
            role_ids.push(role.id)
        })
        setSelectedOption(role_ids)

    }, [data])

    const roleOptionResult = useGetRoleOptions()
    const {mutate} = useUpdateAdminUser(params.admin_user_id)

    const roles = _.get(roleOptionResult, 'data.data.options', [])


    const handleProfileImageChange = ((e) => {
        const file = e.target.files[0];
        setImage(file)
    })

    const handleSubmit = (async (e) => {
        e.preventDefault()

        var formData = new FormData()

        formData.append("full_name", full_name)
        formData.append("is_super_admin", is_super_admin)

        selectedOption.map((option) => {
            formData.append("role_ids[]", option)
        })

        if (image) {
            formData.append('image', image)
        }

        mutate(formData)

    })

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64">
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
                                <div className="relative z-10">
                                    <AvoRedMultiSelectField
                                        label="Roles"
                                        options={roles}
                                        selectedOption={selectedOption}
                                        onChangeSelectedOption={setSelectedOption}
                                    >
                                    </AvoRedMultiSelectField>

                                </div>
                            </div>
                            <div className="mb-4 flex items-center">
                                <label htmlFor="is_super_admin_switch" className="text-sm text-gray-600">Is Super
                                    Admin</label>
                                <Switch
                                    checked={is_super_admin}
                                    onChange={setIsSuperAdmin}
                                    id="is_super_admin_switch"
                                    className={`${
                                        is_super_admin ? 'bg-primary-500' : 'bg-gray-200'
                                    } relative ml-5 inline-flex h-6 w-11 items-center rounded-full`}
                                >
                                    <span className="sr-only">Enable notifications</span>
                                    <span
                                        className={`${
                                            is_super_admin ? 'translate-x-6' : 'translate-x-1'
                                        } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                                    />
                                </Switch>
                            </div>
                            <div className="flex items-center mt-3">
                                <div className="ring-1 ring-gray-300 rounded">
                                    <div className="p-3">
                                        <img className="h-48 w-48 rounded"
                                             alt={full_name}
                                             src={`${current_profile_image}`}/>
                                    </div>
                                </div>
                                <div className="ml-5">
                                    <div className="mb-4">
                                        <InputField
                                            accept="image/*"
                                            label="New Profile Photo"
                                            type="file"
                                            name="image"
                                            onChange={handleProfileImageChange}
                                        />
                                    </div>
                                </div>
                            </div>
                            <div className="flex items-center mt-5">
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