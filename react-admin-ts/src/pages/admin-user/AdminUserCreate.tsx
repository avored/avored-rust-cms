import { useState, ChangeEvent, FormEvent } from "react";
import { Link } from "react-router-dom";
import InputField from "../../components/InputField";
import { useStoreAdminUser } from "./hooks/useStoreAdminUser";

function AdminUserCreate() {
  const [full_name, setFullName] = useState<string>("Admin 3");
  const [email, setEmail] = useState<string>(
    "admin" + new Date().getMinutes() + new Date().getSeconds() + "@admin.com"
  );
  const [image, setImage] = useState<File | null>(null);
  const { mutate } = useStoreAdminUser();

  const handleProfileImageChange = (e: ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files ? e.target.files[0] : null;
    setImage(file);
  };

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData();

    const data = {
      email,
      password: "", // Add the missing properties with empty values
      name: "",
      role: "",
      avatar: "",
    };

    Object.entries(data).forEach(([key, value]) => {
      formData.append(key, value);
    });

    if (image) {
      formData.append("image", image);
    }

    mutate(formData);
  };

  return (
    <div className="flex-1 bg-white">
      <div className="px-5 pl-64">
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
                  errorMessages={[]}
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
                  errorMessages={[]}
                />
              </div>
              <div className="mb-4">
                <InputField
                  accept="image/*"
                  label="Profile Photo"
                  type="file"
                  name="image"
                  onChange={handleProfileImageChange}
                  value={""}
                  errorMessages={[]}
                />
              </div>

              <div className="flex items-center">
                <button
                  type="submit"
                  className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                >
                  Save
                </button>
                <Link
                  to="/admin/admin-user"
                  className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                >
                  Cancel
                </Link>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
}

export default AdminUserCreate;
