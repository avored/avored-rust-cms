import logo from '../assets/logo_only.svg'
import FeatherIcon from 'feather-icons-react'

function Dashboard() {
    return (
        <div
            className="min-h-screen">
            <header
                className="h-16 py-2 shadow-lg px-4 md:sticky top-0 bg-gray-800 z-40">
                <a href="/admin"
                   className="text-white flex items-center space-x-2 group hover:text-white">
                    <div>
                        <img src={logo} className="h-12"/>
                    </div>

                    <div>
                        <span className="text-2xl font-semibold">AvoRed</span>
                        <span className="text-xs block">Rust CMS</span>
                    </div>
                </a>
            </header>
            <div className="flex">
                <div
                    className="w-64 max-h-screen top-16 pt-5 h-screen bg-gray-800 text-blue-100 fixed inset-y-0 left-0 transform transition duration-200 ease-in-out">
                    <nav
                        className="px-4 pt-4 scroller overflow-y-scroll max-h-[calc(100vh-64px)]"
                    >
                        <ul className="flex flex-col space-y-2">


                            <li className="text-sm text-gray-500 ">
                                <a href="/admin"
                                   className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700 ">
                                    <div className="pr-2">
                                        <FeatherIcon className="h-4 w-4" icon="anchor"/>
                                    </div>
                                    <div>Dashboard</div>
                                </a>
                            </li>

                            <div
                                className="section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3">
                                Content Manager
                            </div>


                            <li className="text-sm text-gray-500 ">
                                <a href="#"
                                   className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700 ">
                                    <div className="pr-2">
                                        <FeatherIcon className="h-4 w-4" icon="anchor"/>
                                    </div>
                                    <div>Page</div>
                                </a>
                                <a href="#"
                                   className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700 ">
                                    <div className="pr-2">
                                        <FeatherIcon className="h-4 w-4" icon="compass"/></div>
                                    <div>Components</div>
                                </a>
                                <a href="/admin/asset"
                                   className="flex items-center w-full py-1 px-2 mt-3 rounded relative hover:text-white hover:bg-gray-700 ">
                                    <div className="pr-2">
                                        <FeatherIcon className="h-4 w-4" icon="image"/>
                                    </div>
                                    <div>
                                        Assets Manager
                                    </div>
                                </a>
                            </li>


                            <div
                                className="section border-b pt-4 mb-4 text-xs text-gray-600 border-gray-700 pb-1 pl-3">
                                Managment
                            </div>


                            <li className="text-sm text-gray-500 ">
                                <a href="#"
                                   className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700">
                                    <div className="pr-2">
                                        <FeatherIcon className="h-4 w-4" icon="anchor"/>
                                    </div>
                                    <div>Team</div>
                                    <div
                                        className="absolute right-1.5 transition-transform duration-300"
                                    >
                                        <FeatherIcon className="h-4 w-4" icon="chevron-down"/>
                                    </div>
                                </a>

                                <div
                                    className="pl-4 pr-2 overflow-hidden transition-all transform translate duration-300 max-h-0 "
                                >
                                    <ul
                                        className="flex flex-col mt-2 pl-2 text-gray-500 border-l border-gray-700 space-y-1 text-xs">

                                        <li className="text-sm text-gray-500 ">
                                            <a href="#"
                                               className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700">
                                                <div>
                                                    Admin Users
                                                </div>
                                            </a>
                                        </li>

                                        <li className="text-sm text-gray-500 ">
                                            <a href="/admin/role"
                                               className="flex items-center w-full py-1 px-2 rounded relative hover:text-white hover:bg-gray-700">
                                                <div>
                                                    Roles
                                                </div>
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                            </li>

                        </ul>
                    </nav>
                </div>

                <div className="flex-1 pl-64 bg-white">
                    <div className="p-5">
                        Dashboard content goes here
                    </div>
                </div>
            </div>


        </div>
    )
}

export default Dashboard