import { useTranslation } from "react-i18next";
import { ResponsiveBar } from '@nivo/bar';
import { UseGetVisitByYearHook } from "../hooks/dashboard/UseGetVisitByYearHook";
import { VisitByYearType, VisitByContentTypeType } from "../types/dashboard/dashboard";
import { UseGetVisitByContentTypeHook } from "../hooks/dashboard/UseGetVisitByContentTypeHook";
import { VisitByContentTypeRequest, VisitByYearRequest } from "grpc-avored/dashboard_pb";

export const DashboardPage = (() => {
    const [t] = useTranslation("global");

    const visitByYearRequest = new VisitByYearRequest()
    visitByYearRequest.setYear(2025)

    const request = new VisitByContentTypeRequest()
    request.setYear(2025)
    request.setContentType("pages")


    const getVisitByYearResponse = UseGetVisitByYearHook(visitByYearRequest);
    const getVisitByContentTypeResponse = UseGetVisitByContentTypeHook(request);
    
    const data_list = getVisitByYearResponse?.data?.dataList ?? [];
    const visit_by_year_lists: Array<VisitByYearType> = data_list as Array<unknown> as VisitByYearType[];

    const visit_list_by_content_type = getVisitByContentTypeResponse?.data?.dataList ?? [];
    const visit_by_content_type_lists: Array<VisitByContentTypeType> = visit_list_by_content_type as Array<unknown> as VisitByContentTypeType[];

   

    return (
        <div className="p-5">
            <div className="block md:flex my-5">
                <div className="w-full md:w-1/2">
                    {t("Visitor by month")}
                    <ResponsiveBar
                        data={visit_by_year_lists}
                        indexBy="month"
                        margin={{
                            bottom: 60,
                            left: 30,
                            right: 110,
                            top: 60
                        }}
                        keys={[
                            'visits'
                        ]}
                        padding={0.2}
                    />
                </div>
                <div className="w-full md:w-1/2 h-96 ml-3">
                    {t('Visitor by Content type')}
                    <ResponsiveBar
                        data={visit_by_content_type_lists}
                        indexBy="month"
                        margin={{
                            bottom: 60,
                            left: 30,
                            right: 110,
                            top: 60
                        }}
                        keys={[
                            'visits'
                        ]}
                        padding={0.2}
                    />
                </div>
            </div>
            
        </div>
    )
})