import InputField from "../../components/InputField";
import {useMemo, useState} from "react";
import {useSetting} from "./hooks/useSetting"
import _ from 'lodash'
import {useStoreSetting} from "./hooks/useStoreSetting";
import { useTranslation } from "react-i18next";

function Setting() {
    const [settings, setSettings] = useState([])
    const {data} = useSetting()
    const {mutate} = useStoreSetting()
    const [t] = useTranslation("global")

    useMemo(() => {
        setSettings(_.get(data, 'data', []))
    }, [data])

    const getSettingValue = ((identifier) => {
        let setting  = settings.find((setting) => setting.identifier === identifier);

        return _.get(setting, 'value', '');
    })

    const settingChange = ((identifier, value) => {
        setSettings(prevSettings => {
            return prevSettings.map(setting => {
                if (setting.identifier === identifier) {
                    setting.value = value
                    return setting
                }
                return setting
            })
        })
    })

    const handleSubmit = (async (e) => {
        e.preventDefault()
        mutate({"settings": settings})
    })

    return (
      <div className="flex-1 bg-white">
        <div className="pl-64">
          <div className="p-5">
            <div className="flex items-center">
              <div className="p-5 text-2xl font-semibold text-primary-500">
                {t("common.settings")}
              </div>
            </div>

            <div class="grid grid-cols-12 pt-3">
              <div class="col-span-2 sm:border-0 border-r px-3">
                <ul>
                  <li
                    class="mt-5 bg-gray-200 rounded cursor-pointer 
                                        border-l-2 border-transparent px-2 py-2 font-semibold
                                        transition hover:border-l-primary-600"
                  >
                    {t("common.general")}
                  </li>
                </ul>
              </div>

              <div class="col-span-10 px-5 rounded py-5">
                <form onSubmit={handleSubmit}>
                  <div class="sm:items-center sm:justify-between">
                    <InputField
                      label={t("settings.site_name")}
                      name="site_name"
                      value={getSettingValue("general_site_name")}
                      onChange={(e) =>
                        settingChange("general_site_name", e.target.value)
                      }
                      autoFocus
                    />
                  </div>

                  <div class="flex">
                    <button
                      type="submit"
                      class="mt-5 rounded-lg bg-primary-600 px-4 py-2 text-white"
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
}

export default Setting