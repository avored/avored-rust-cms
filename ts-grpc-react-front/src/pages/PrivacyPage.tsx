import {GetCmsContentRequest} from "../grpc_generated/cms_pb";
import {useHomeCmsPage} from "./home/hooks/useHomeCmsPage";
import {CmsContentType, ContentFieldDataType, ContentFieldFieldType, ContentFieldType} from "../types/CmsPageType";
import _ from "lodash";
import {GetElementValue} from "../lib/page";
import Markdown from "react-markdown";

export const PrivacyPage = () => {
    const request = new GetCmsContentRequest();
    request.setContentIdentifier("privacy-page");
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
            <div className="mt-10 px-5">

                <div className="mb-10 block">
                    <div className="text-4xl heading-font text-primary-600 font-bold mt-10">
                        {GetElementValue(GetPageFields(), 'privacy-title')}
                    </div>

                    <div className="mt-10 mb-10">
                        <div className="markdown-body">
                            <Markdown>
                                {GetElementValue(GetPageFields(), 'privacy-content')}
                            </Markdown>
                        </div>
                    </div>

                </div>
            </div>
        </>
    )
}
