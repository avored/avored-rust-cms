import {useTranslation} from "react-i18next";

function Dashboard() {
    const [t] = useTranslation("global");
    return (
        <div className="flex-1 bg-white">
            <div className="pl-64">
                <div className="p-5">
                    {t('dashboard_demo_content')}
                </div>
            </div>
        </div>
    )
}

export default Dashboard