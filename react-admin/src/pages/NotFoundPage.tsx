import {useTranslation} from "react-i18next";

function NotFoundPage() {
    const [t] = useTranslation('global');
    return (
        <div className="h-screen flex w-full justify-center items-center">
            <div className="font-semibold text-primary-500 text-2xl">
                {t('not_found_404')}
            </div>
        </div>
    )
}

export default NotFoundPage