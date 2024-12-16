import {Link, Outlet} from "react-router-dom";
import {Menu} from "@headlessui/react";
import {useTranslation} from "react-i18next";
import {ChevronDownIcon, FilmIcon, RocketLaunchIcon, CpuChipIcon, DeviceTabletIcon} from "@heroicons/react/24/solid";
import {useContext} from "react";
import {ThemeContext} from "../../context/ThemeContext";

function AppSidebar() {
    const [t] = useTranslation("global");
    const theme = useContext(ThemeContext);
    return (
        <div className="flex">
            <div
                className={` ${theme.isSidebarOpen ? 'w-16' : 'w-64'} overflow-x-hidden max-h-screen top-0 pt-16 h-screen bg-gray-800 text-blue-100 fixed inset-y-0 left-0 transform transition duration-200 ease-in-out`}>
                <nav className="px-4 pt-4 scroller max-h-[calc(100vh-64px)]">
                    <ul className="flex flex-col space-y-2">
                        <li className="text-sm text-gray-500 ">
                            <Link
                                to={`/admin`}
                                className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700 "
                            >
                                <div className="pr-2">
                                    <CpuChipIcon className="h-6 w-6"/>
                                </div>
                                <div className="ml-2">{t("sidebar.dashboard")}</div>
                            </Link>
                        </li>

                        <div
                            className={`${theme.isSidebarOpen ? 'hidden' : ''} section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3`}>
                            {t("sidebar.content_manager")}
                        </div>

                        <li className="text-sm text-gray-500 ">
                            <Link
                                className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700"
                                to={`/admin/page`}
                            >
                                <div className="pr-2">
                                    <RocketLaunchIcon className="h-6 w-6"/>
                                </div>
                                <div className="ml-2">{t("sidebar.page")}</div>
                            </Link>

                            <Link
                                to={`admin/component`}
                                className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700 "
                            >
                                <div className="pr-2">
                                    <CpuChipIcon className="h-6 w-6"/>
                                </div>
                                <div className="ml-2">{t("components")}</div>
                            </Link>
                            <Link
                                to={`/admin/asset`}
                                className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700"
                            >
                                <div className="pr-2">
                                    <FilmIcon className="h-6 w-6"/>
                                </div>
                                <div className="ml-2">{t("asset_manager")}</div>
                            </Link>

                            <Link
                                className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700"
                                to={`/admin/model`}
                            >
                                <div className="pr-2">
                                    <DeviceTabletIcon className="h-6 w-6"/>
                                </div>
                                <div className="ml-2">{t("model")}</div>
                            </Link>

                        </li>

                        <div
                            className={`${theme.isSidebarOpen ? 'hidden' : ''} section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3`}>
                            {t("sidebar.management")}
                        </div>

                        <Menu as="li" className="text-sm text-gray-500">
                            <Menu.Button
                                className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700">
                                <div className="pr-2">
                                    <RocketLaunchIcon className="h-6 w-6"/>
                                </div>
                                <div className="ml-2">{t("sidebar.team")}</div>
                                <div className="absolute right-1.5 transition-transform duration-300">
                                    <ChevronDownIcon className="h-6 w-6"/>
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
            <div className="flex-1 bg-white">
                <div className={`${theme.isSidebarOpen ? 'pl-16' : 'pl-64'}`}>
                    <Outlet/>
                </div>
            </div>
        </div>
    );
}

export default AppSidebar;
