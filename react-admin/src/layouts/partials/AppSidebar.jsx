import FeatherIcon from "feather-icons-react";
import {Link, Outlet} from "react-router-dom";
import {Menu} from "@headlessui/react";

function AppSidebar() {
    return (
        <div className="flex">
            <div
                className="w-64 max-h-screen top-16 pt-5 h-screen bg-gray-800 text-blue-100 fixed inset-y-0 left-0 transform transition duration-200 ease-in-out">
                <nav
                    className="px-4 pt-4 scroller overflow-y-scroll max-h-[calc(100vh-64px)]"
                >
                    <ul className="flex flex-col space-y-2">
                        <li className="text-sm text-gray-500 ">
                            <Link to={`/admin`}
                               className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700 ">
                                <div className="pr-2">
                                    <FeatherIcon className="h-4 w-4" icon="anchor"/>
                                </div>
                                <div>Dashboard</div>
                            </Link>
                        </li>

                        <div
                            className="section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3">
                            Content Manager
                        </div>


                        <li className="text-sm text-gray-500 ">
                            <Link
                                className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700"
                                to={`/admin/page`}>
                                <div className="pr-2">
                                    <FeatherIcon className="h-4 w-4" icon="anchor"/>
                                </div>
                                <div>Page</div>
                            </Link>

                            <Link to={`admin/component`}
                               className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700 ">
                                <div className="pr-2">
                                    <FeatherIcon className="h-4 w-4" icon="compass"/></div>
                                <div>Components</div>
                            </Link>
                            <Link to={`/admin/asset`}
                               className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700">
                                <div className="pr-2">
                                    <FeatherIcon className="h-4 w-4" icon="image"/>
                                </div>
                                <div>
                                    Assets Manager
                                </div>
                            </Link>
                        </li>


                        <div
                            className="section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3">
                            Managment
                        </div>

                        <Menu as="li" className="text-sm text-gray-500">
                            <Menu.Button
                                className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700">
                                <div className="pr-2">
                                    <FeatherIcon className="h-4 w-4" icon="anchor"/>
                                </div>
                                <div>Team</div>
                                <div
                                    className="absolute right-1.5 transition-transform duration-300"
                                >
                                    <FeatherIcon className="h-4 w-4" icon="chevron-down"/>
                                </div>
                            </Menu.Button>

                            <Menu.Items className="flex flex-col mt-2 pl-2 ml-3 border-l border-gray-700 space-y-1">
                                <Menu.Item as="li">
                                    {({active}) => (
                                        <Link to={`/admin/admin-user`}
                                           className={`flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700`}
                                        >
                                            <div>
                                                Admin User
                                            </div>
                                        </Link>
                                    )}
                                </Menu.Item>
                                <Menu.Item as="li">
                                    {({active}) => (
                                        <Link to={`/admin/role`}
                                           className={`flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700`}
                                        >
                                            <div>
                                                Role
                                            </div>
                                        </Link>
                                    )}
                                </Menu.Item>
                                <Menu.Item>
                                    {({ active }) => (
                                        <Link to={`/admin/setting`}
                                            className={`flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700`}
                                        >
                                            Setting
                                        </Link>
                                    )}
                                </Menu.Item>

                            </Menu.Items>
                        </Menu>


                    </ul>
                </nav>
            </div>
            <Outlet/>
        </div>
);
}

export default AppSidebar;
