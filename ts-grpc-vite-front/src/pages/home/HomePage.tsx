import {ContactSection} from "./ContactSection";
import FeaturesSection from "./FeaturesSection";
import MainHeroSection from "./MainHeroSection";
import RepositoryInformation from "./RepositoryInformation";
import {useHomeCmsPage} from "./hooks/useHomeCmsPage";
import mainHeroImage from "../../assets/main-hero.svg";
import _ from "lodash";
import { GetCmsContentRequest } from "grpc-avored/cms_pb";
import { CmsContentType, ContentFieldDataType, ContentFieldFieldType, ContentFieldType } from "../../types/CmsPageType";

const HomePage = () => {
    const request = new GetCmsContentRequest();
    request.setContentIdentifier("home-page");
    request.setContentType("pages");
    const get_cms_content_api_response = useHomeCmsPage(request)
    const get_content_model = get_cms_content_api_response?.data?.data ?? [];

    const values: CmsContentType = get_content_model as unknown as CmsContentType;

    const content_content_field_list: Array<any> = get_cms_content_api_response?.data?.data?.contentFieldsList ?? [];

    if (values) {
        values.contentFields = [];
        content_content_field_list.map(content_field => {
            const grpc_content_field: ContentFieldType = {
                name: content_field.name,
                identifier: content_field.identifier,
                dataType: content_field.dataType as ContentFieldDataType,
                fieldType: content_field.fieldType as ContentFieldFieldType,
                fieldContent: {
                    textValue: content_field.fieldContent?.textValue ?? '',
                }
            }

            values.contentFields.push(grpc_content_field)

            return grpc_content_field
        })
    }

    const GetPageFields = ((): Array<ContentFieldType> => {
        return _.get(values, 'contentFields', [])
    })
    return (
        <>
            <div className="relative bg-white overflow-hidden">
                <MainHeroSection contentFields={GetPageFields()}/>

                <div className="lg:absolute lg:inset-y-0 lg:right-0 lg:w-1/2">
                    <img
                        className="h-80 pl-24 pt-20 mt-5 hidden lg:block object-cover"
                        src={mainHeroImage}
                        alt="avored rust main hero"
                    />
                </div>
            </div>

            <RepositoryInformation contentFields={GetPageFields()}/>
            <FeaturesSection contentFields={GetPageFields()}/>
            <ContactSection contentFields={GetPageFields()}/>
        </>
    );
};
export default HomePage;
