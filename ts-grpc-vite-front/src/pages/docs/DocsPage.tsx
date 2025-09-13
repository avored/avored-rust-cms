import Markdown from "react-markdown";
// import { CmsContentType } from "../../types/CmsPageType";
import { useHomeCmsPage } from "../home/hooks/useHomeCmsPage";
import { GetElementValue } from "../../lib/page";
import { Link } from "react-router-dom";
import { GetCmsContentRequest } from "grpc-avored/cms_pb";

export const DocsPage = (() => {

    const request = new GetCmsContentRequest();
    request.setContentIdentifier("docs-page");
    request.setContentType("pages");
    
    const get_cms_content_api_response = useHomeCmsPage(request)
    // const get_content_model = get_cms_content_api_response?.data?.data ?? [];

    // const values: CmsContentType = get_content_model as unknown as CmsContentType;

    const content_content_field_list: Array<any> = get_cms_content_api_response?.data?.data?.contentFieldsList ?? [];
    
    return(
        <>
        <div className="flex w-full">
            <div className="p-5 bg-gray-200 rounded-lg w-48">
                <Link to={`/docs`}>
                    Installation
                </Link>
            </div>
            <div className="p-5 flex-1 markdown-body">
                <Markdown>{GetElementValue(content_content_field_list, 'docs-page-content')}</Markdown>
            </div>
            <div className="p-5 w-64">
                {/* right side */}
            </div>
        </div>
        </>
    )
})