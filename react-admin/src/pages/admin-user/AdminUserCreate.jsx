import {useState} from "react";
import {Link, useNavigate} from "react-router-dom";
import InputField from "../../components/InputField";
import axios from 'axios';

function AdminUserCreate() {
    const [full_name, setFullName] = useState('Admin 3')
    const [email, setEmail] = useState('admin' + new Date().getMinutes() + new Date().getSeconds() +  '@admin.com')
    const [image, setImage] = useState()
    const navigate = useNavigate()

    const handleProfileImageChange = ((e) => {
        // console.log(e.target.files[0])
        const file = e.target.files[0];
        // console.log(file)
        setImage(file)
        // console.log(image)
    });

    const handleSubmit = (async (e) => {
        e.preventDefault()
        var formData = new FormData()

        formData.append("full_name", full_name);
        formData.append("email", email);
        formData.append('image', image)

        const created_admin_user_response = (await axios({
            url: 'http://localhost:8080/api/admin-user',
            method: 'POST',
            headers: {
                'Content-Type': 'multipart/form-data; boundary=----',
                'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
            },
            data: formData
        }))

        if (created_admin_user_response.data.status) {
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
                            {JSON.stringify(image)}
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
                                <InputField
                                    label="Email Address"
                                    type="email"
                                    name="email"
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                    autoFocus
                                />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    accept="image/*"
                                    label="Profile Photo"
                                    type="file"
                                    name="image"
                                    onChange={handleProfileImageChange}
                                    autoFocus
                                />
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

export default AdminUserCreate