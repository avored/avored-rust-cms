import {useTranslation} from "react-i18next";

export const DashboardPage = (() => {
    const [t] = useTranslation("global");
    return (
        <div className="p-5">
            {t('dashboard_demo_content')}
        </div>
    )
})