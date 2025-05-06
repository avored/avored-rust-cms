import {ContactSection} from "./ContactSection";
import FeaturesSection from "./FeaturesSection";
import MainHeroSection from "./MainHeroSection";
import RepositoryInformation from "./RepositoryInformation";
import {useHomeCmsPage} from "./hooks/useHomeCmsPage";
import mainHeroImage from "../../assets/main-hero.svg";
import _ from "lodash";
import {CmsContentType, ContentFieldDataType, ContentFieldFieldType, ContentFieldType} from "../../types/CmsPageType";
import {GetCmsContentRequest} from "../../grpc_generated/cms_pb";

const HomePage = () => {
    const request = new GetCmsContentRequest();
    request.setContentId("wvb4100904eaf3ykz64c");
    request.setContentType("pages");
    const get_cms_content_api_response = useHomeCmsPage(request)
    const get_content_model = get_cms_content_api_response?.data?.data ?? [];

    const values: CmsContentType = get_content_model as unknown as CmsContentType;

    const content_content_field_list: Array<any> = get_cms_content_api_response?.data?.data?.contentFieldsList ?? [];

    if (values) {
        values.content_fields = [];
        content_content_field_list.map(content_field => {
            const grpc_content_field: ContentFieldType = {
                name: content_field.name,
                identifier: content_field.identifier,
                data_type: content_field.dataType as ContentFieldDataType,
                field_type: content_field.fieldType as ContentFieldFieldType,
                field_content: {
                    text_value: content_field.fieldContent?.textValue ?? '',
                }
            }

            values.content_fields.push(grpc_content_field)

            return grpc_content_field
        })
    }

    const GetPageFields = ((): Array<ContentFieldType> => {
        return _.get(values, 'content_fields', [])
    })
    return (
        <>
            <div className="relative bg-white overflow-hidden">
                <MainHeroSection content_fields={GetPageFields()}/>

                <div className="lg:absolute lg:inset-y-0 lg:right-0 lg:w-1/2">
                    <img
                        className="h-96 pt-20 mt-5 hidden lg:block object-cover"
                        src={mainHeroImage}
                        alt="avored rust main hero"
                    />
                </div>
            </div>

            <RepositoryInformation content_fields={GetPageFields()}/>
            <FeaturesSection content_fields={GetPageFields()}/>
            <ContactSection content_fields={GetPageFields()}/>
        </>
    );
};
export default HomePage;
