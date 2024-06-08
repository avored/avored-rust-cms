import {Link, Outlet} from "react-router-dom";
import {Menu} from "@headlessui/react";
import {useTranslation} from "react-i18next";
import {ChevronDownIcon, FilmIcon, RocketLaunchIcon, CpuChipIcon} from "@heroicons/react/24/solid";

function AppSidebar() {
    const [t] = useTranslation("global");
    return (
        <div className="flex">
            <div
                className="w-64 max-h-screen top-0 pt-16 h-screen bg-gray-800 text-blue-100 fixed inset-y-0 left-0 transform transition duration-200 ease-in-out">
                <nav className="px-4 pt-4 scroller overflow-y-scroll max-h-[calc(100vh-64px)]">
                    <ul className="flex flex-col space-y-2">
                        <li className="text-sm text-gray-500 ">
                            <Link
                                to={`/admin`}
                                className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700 "
                            >
                                <div className="pr-2">
                                    <CpuChipIcon className="h-4 w-4"/>
                                </div>
                                <div>{t("sidebar.dashboard")}</div>
                            </Link>
                        </li>

                        <div className="section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3">
                            {t("sidebar.content_manager")}
                        </div>

                        <li className="text-sm text-gray-500 ">
                            <Link
                                className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700"
                                to={`/admin/page`}
                            >
                                <div className="pr-2">
                                    <RocketLaunchIcon className="h-4 w-4" />
                                </div>
                                <div>{t("sidebar.page")}</div>
                            </Link>

                            <Link
                                to={`admin/component`}
                                className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700 "
                            >
                                <div className="pr-2">
                                    <CpuChipIcon className="h-4 w-4"/>
                                </div>
                                <div>{t("sidebar.components")}</div>
                            </Link>
                            <Link
                                to={`/admin/asset`}
                                className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700"
                            >
                                <div className="pr-2">
                                    <FilmIcon className="h-4 w-4" />
                                </div>
                                <div>{t("sidebar.assets_manager")}</div>
                            </Link>
                        </li>

                        <div className="section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3">
                            {t("sidebar.management")}
                        </div>

                        <Menu as="li" className="text-sm text-gray-500">
                            <Menu.Button
                                className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700">
                                <div className="pr-2">
                                    <RocketLaunchIcon className="h-4 w-4"/>
                                </div>
                                <div>{t("sidebar.team")}</div>
                                <div className="absolute right-1.5 transition-transform duration-300">
                                    <ChevronDownIcon className="h-4 w-4" />
                                </div>
                            </Menu.Button>

                            <Menu.Items className="flex flex-col mt-2 pl-2 ml-3 border-l border-gray-700 space-y-1">
                                <Menu.Item as="li">
                                    {({active}) => (
                                        <Link
                                            to={`/admin/admin-user`}
                                            className={`flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700`}
                                        >
                                            <div>{t("sidebar.admin_user")}</div>
                                        </Link>
                                    )}
                                </Menu.Item>
                                <Menu.Item as="li">
                                    {({active}) => (
                                        <Link
                                            to={`/admin/role`}
                                            className={`flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700`}
                                        >
                                            <div>{t("sidebar.role")}</div>
                                        </Link>
                                    )}
                                </Menu.Item>
                                <Menu.Item>
                                    {({active}) => (
                                        <Link
                                            to={`/admin/setting`}
                                            className={`flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700`}
                                        >
                                            {t("sidebar.setting")}
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
