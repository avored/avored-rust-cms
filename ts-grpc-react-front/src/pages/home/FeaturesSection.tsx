import {GetElementValue} from "../../lib/page";
import {PageFieldType} from "../../types/CmsPageType";

type KeyFeaturesComponentProps = {
    page_fields: PageFieldType[]
}
const FeaturesSection = (props: KeyFeaturesComponentProps) => {
    return (
        <>
            <div className="text-4xl text-primary-700 pl-5 font-semibold heading-font mt-10">
                KEY FEATURES
            </div>
            <div className="my-8">
                <div className="md:flex flex-row">
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            {GetElementValue(props.page_fields, 'content-modeling-title')}
                        </div>
                        <div className="text-left text-sm mt-3">
                            {GetElementValue(props.page_fields, 'content-modeling-description')}
                        </div>
                    </div>
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            {GetElementValue(props.page_fields, 'api-first-approach-title')}
                        </div>
                        <div className="text-left text-sm mt-3">
                            {GetElementValue(props.page_fields, 'api-first-approach-description')}
                        </div>
                    </div>
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            {GetElementValue(props.page_fields, 'content-versioning-and-history-title')}
                        </div>
                        <div className="text-left text-sm mt-3">
                            {GetElementValue(props.page_fields, 'content-versioning-and-history-description')}
                        </div>
                    </div>
                </div>
            </div>
            <div className="mt-8 flex">
                <div className="md:flex flex-row">
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            {GetElementValue(props.page_fields, 'scalability-and-performance-title')}
                        </div>
                        <div className="text-left text-sm mt-3">
                            {GetElementValue(props.page_fields, 'scalability-and-performance-description')}
                        </div>
                    </div>
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font heading-font text-xl">
                            {GetElementValue(props.page_fields, 'integration-capabilities-title')}
                        </div>
                        <div className="text-left text-sm mt-3">
                            {GetElementValue(props.page_fields, 'integration-capabilities-description')}
                        </div>
                    </div>
                    <div className="md:w-1/3 w-full hover:bg-primary-600 rounded-lg hover:text-white p-6 mt-3">
                        <div className="font-semibold heading-font text-xl">
                            {GetElementValue(props.page_fields, 'content-localization-and-internationalization-title')}
                        </div>
                        <div className="text-left text-sm mt-3">
                            {GetElementValue(props.page_fields, 'content-localization-and-internationalization-description')}
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
};
export default FeaturesSection;
