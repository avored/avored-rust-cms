import SwaggerUI from "swagger-ui-react"
import "swagger-ui-react/swagger-ui.css"
import {useMemo, useState} from "react";
import {useOpenapi} from "./hooks/useOpenapi";
import _ from 'lodash'

function AvoRedApiTesting() {
    const [openapi, setOpenapi] = useState({})

    const {data} = useOpenapi()


    useMemo(() => {
        console.log(JSON.stringify(_.get(data, 'data')))
        setOpenapi(JSON.stringify(_.get(data, 'data')))
    }, [data])


    return (
        <div className="flex-1 bg-white">
            <div className="px-5 ml-64">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        Swagger UI
                    </div>
                </div>

                <div className="overflow-x-auto">
                    <SwaggerUI spec={openapi} />
                </div>
            </div>
        </div>
    )
}

export default AvoRedApiTesting