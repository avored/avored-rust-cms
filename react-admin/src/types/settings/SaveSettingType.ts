import SettingType from "./SettingType"

type SaveSettingType = {
    settings: Array<SettingType>;
}

type UpdatableSettingType = {
    identifier: string;
    value: string;
}

export default SaveSettingType;
