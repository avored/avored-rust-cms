import InputField from "../../components/InputField";
import { useMemo, useState } from "react";
import { useSetting } from "./hooks/useSetting";
import _ from "lodash";
import { useStoreSetting } from "./hooks/useStoreSetting";
import { useTranslation } from "react-i18next";

interface Setting {
  identifier: string;
  value: string;
}

interface SettingData {
  settings: any;
  email: string;
  password: string;
}

const Setting = () => {
  const [settings, setSettings] = useState<Setting[]>([]);
  const { data } = useSetting();
  const { mutate } = useStoreSetting();
  const [t] = useTranslation("global");

  useMemo(() => {
    setSettings(_.get(data, "data", []));
  }, [data]);

  const getSettingValue = (identifier: string): string => {
    let setting = settings.find(
      (setting: any) => setting.identifier === identifier
    );

    return _.get(setting, "value", "");
  };

  const settingChange = (identifier: string, value: string): void => {
    setSettings((prevSettings: Setting[]) => {
      return prevSettings.map((setting: Setting) => {
        if (setting.identifier === identifier) {
          return { ...setting, value: value };
        }
        return setting;
      });
    });
  };

  const email = ""; // Replace with the actual email value
  const password = ""; // Replace with the actual password value
  const handleSubmit = async (e: { preventDefault: () => void }) => {
    e.preventDefault();

    const settingsObject: SettingData = {
      email: email,
      password: password, // replace with the actual password value
      settings: settings.reduce((obj: Record<string, string>, setting) => {
        obj[setting.identifier] = setting.value;
        return obj;
      }, {}),
    };

    mutate(settingsObject);
  };

  return (
    <div className="flex-1 bg-white">
      <div className="pl-64">
        <div className="p-5">
          <div className="flex items-center">
            <div className="p-5 text-2xl font-semibold text-primary-500">
              {t("common.settings")}
            </div>
          </div>

          <div className="grid grid-cols-12 pt-3">
            <div className="col-span-2 sm:border-0 border-r px-3">
              <ul>
                <li
                  className="mt-5 bg-gray-200 rounded cursor-pointer 
                                        border-l-2 border-transparent px-2 py-2 font-semibold
                                        transition hover:border-l-primary-600"
                >
                  {t("common.general")}
                </li>
              </ul>
            </div>

            <div className="col-span-10 px-5 rounded py-5">
              <form onSubmit={handleSubmit}>
                <div className="sm:items-center sm:justify-between">
                  <InputField
                    label={t("settings.site_name")}
                    name="site_name"
                    value={getSettingValue("general_site_name")}
                    onChange={(e) =>
                      settingChange("general_site_name", e.target.value)
                    }
                    autoFocus
                    errorMessages={[]}
                  />
                </div>

                <div className="flex">
                  <button
                    type="submit"
                    className="mt-5 rounded-lg bg-primary-600 px-4 py-2 text-white"
                  >
                    {t("common.save")}
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Setting;
