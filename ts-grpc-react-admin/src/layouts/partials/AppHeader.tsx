import { Menu, MenuButton, MenuItem, MenuItems } from "@headlessui/react";
import _ from "lodash";
import { Link } from "react-router-dom";
import {useContext, useState} from "react";
import logo from "../../assets/logo_only.svg"
import enLangLogo from "../../assets/images/locales/en.svg";
import { useTranslation } from "react-i18next";
import {InstallDataConfirmationModal} from "./InstallDataConfirmationModal";
import {DeleteDataConfirmationModal} from "./DeleteDataConfirmationModal";
import {Bars4Icon} from "@heroicons/react/24/solid";
import {ThemeContext} from "../../context/ThemeContext";

function AppHeader() {
  // const auth_user_model = localStorage.getItem("AUTH_ADMIN_USER") ?? "";
  // const adminUser: IAdminUserModel = JSON.parse(auth_user_model);
  // const redirect = useNavigate();
  // const client = useAxios();
  // const { data } = useQuery({
  //   queryKey: ["logged-in-user"],
  //   queryFn: async () => {
  //     try {
  //       const assetUrl: string = "/logged-in-user";
  //       return await client.get(assetUrl);
  //     } catch (error) {
  //       redirect("/admin/login");
  //     }
  //   },
  // });

  const adminUser = {
    name: "Admin",
    email: "admin@gmail.com"
  }
  const install_demo_data = true;
  const theme = useContext(ThemeContext);


  // const install_demo_data = _.get(data, "data.data.demo_data_status", false);

  // const navigate = useNavigate();
  const [t, i18n] = useTranslation("global");
  let [isInstallDemoDataVisible, setIsInstallDemoDataVisible] = useState(false);
  let [isDeleteDemoDataVisible, setIsDeleteDemoDataVisible] = useState(false);

  const openInstallDemoDataVisible = () => {
    setIsInstallDemoDataVisible(true);
  };

  const closeInstallDemoDataVisible = () => {
    setIsInstallDemoDataVisible(false);
  };

  const openDeleteDemoDataVisible = () => {
    setIsDeleteDemoDataVisible(true);
  };

  const closeDeleteDemoDataVisible = () => {
    setIsDeleteDemoDataVisible(false);
  };

  const changeLocale = ((test: any, test1: any) => {

  })


  // useEffect(() => {
    // if (!_.get(adminUser, "id")) {
    //   localStorage.clear();
      // navigate("/admin/login");
    // }
  // });

  return (
    <header className="h-16 py-2 flex shadow-lg px-4 fixed inset-y-0 md:sticky bg-gray-800 z-40">
      <InstallDataConfirmationModal close={closeInstallDemoDataVisible} isOpen={isInstallDemoDataVisible} />
      <DeleteDataConfirmationModal close={closeDeleteDemoDataVisible} isOpen={isDeleteDemoDataVisible} />
      <div className="flex w-full items-center">
        <a
          href="/admin"
          className={`${theme.isSidebarOpen ? "w-16" : "w-64"} text-white  flex items-center space-x-2 group hover:text-white `}
        >
          <div>
            <img src={logo} alt="AvoRed Rust Cms Logo" className="h-12" />
          </div>

          <div className={`${theme.isSidebarOpen ? "hidden" : ""} `}>
            <span className="text-2xl font-semibold">{t("avored")}</span>
            <span className="text-xs block">{t("rust_cms")}</span>
          </div>
        </a>
        <div className="text-white">
            <div className="cursor-pointer" onClick={() => theme.toggleSidebar()}>
                <Bars4Icon className="w-6 h-6" />
            </div>
        </div>
        <div className="ml-auto flex items-center">
          <Menu as="div" className="ml-3 inline-block relative">
            <MenuButton className="flex items-center">
              <div className="flex-shrink-0 w-10 h-10 relative">
                <img
                  className="w-10 h-10 ring-1 ring-white rounded-full"
                  src={enLangLogo}
                  alt={t(`locales.${i18n.language}_label`)}
                />
              </div>
              <div className="p-2 text-white text-left">
                {t(`locales.${i18n.language}_label`)}
              </div>
            </MenuButton>
            <MenuItems
              as="div"
              className="absolute shadow-md  z-30 py-1.5 rounded-md bg-white border border-gray-100 w-full"
            >
              <MenuItem
                as="div"
                className="cursor-pointer"
                onClick={(e) => changeLocale(i18n, "en")}
              >
                <span className="flex items-center text-sm py-1.5 px-4 text-gray-600 hover:text-primary-500 hover:bg-gray-50">
                  {t("english")}
                </span>
              </MenuItem>
              <MenuItem
                as="div"
                className="cursor-pointer"
                onClick={(e) => changeLocale(i18n, "fr")}
              >
                <span className="flex items-center text-sm py-1.5 px-4 text-gray-600 hover:text-primary-500 hover:bg-gray-50">
                  {t("french")}
                </span>
              </MenuItem>
            </MenuItems>
          </Menu>

          <Menu as="div" className="ml-3 inline-block relative">
            <MenuButton className="flex items-center">
              <div className="flex-shrink-0 w-10 h-10 relative">
                <img
                  className="w-10 h-10 ring ring-white rounded-full"
                  src={_.get(adminUser, "profile_image")}
                  alt={_.get(adminUser, "full_name")}
                />
              </div>
              <div className="p-2 text-left">
                <h2 className="text-sm font-semibold text-white">
                  {_.get(adminUser, "full_name")}
                </h2>
                <p className="text-xs text-gray-400">
                  {_.get(adminUser, "email")}
                </p>
              </div>
            </MenuButton>
            <MenuItems
              as="div"
              className="absolute shadow-md  z-30 py-1.5 rounded-md bg-white border border-gray-100 w-full"
            >
              <MenuItem as="div">
                <Link
                  to={`/admin/admin-user-edit/${_.get(adminUser, "id")}`}
                  className="flex items-center text-sm py-1.5 px-4 text-gray-600 hover:text-primary-500 hover:bg-gray-50"
                >
                  {t("profile")}
                </Link>
              </MenuItem>
              <MenuItem as="div">
                <Link
                  to={`/admin/admin-user-change-password`}
                  className="flex items-center text-sm py-1.5 px-4 text-gray-600 hover:text-primary-500 hover:bg-gray-50"
                >
                  {t("change_password")}
                </Link>
              </MenuItem>
              {install_demo_data ? (
                <>
                  <MenuItem as="div">
                    <div
                      onClick={openDeleteDemoDataVisible}
                      className="flex cursor-pointer items-center text-sm py-1.5 px-4 text-gray-600 hover:text-primary-500 hover:bg-gray-50"
                    >
                      {t("delete_demo_data")}
                    </div>
                  </MenuItem>
                </>
              ) : (
                <>
                  <MenuItem as="div">
                    <div
                      onClick={openInstallDemoDataVisible}
                      className="flex cursor-pointer items-center text-sm py-1.5 px-4 text-gray-600 hover:text-primary-500 hover:bg-gray-50"
                    >
                      {t("install_demo_data")}
                    </div>
                  </MenuItem>
                </>
              )}

              <MenuItem as="div">
                <Link
                  to={`/admin/logout`}
                  className="flex items-center text-sm py-1.5 px-4 text-gray-600 hover:text-primary-500 hover:bg-gray-50"
                >
                  {t("logout")}
                </Link>
              </MenuItem>
            </MenuItems>
          </Menu>
        </div>
      </div>
    </header>
  );
}

export default AppHeader;
