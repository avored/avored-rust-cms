import SwaggerUI from "swagger-ui-react"
import "swagger-ui-react/swagger-ui.css"
import {useMemo, useState} from "react";
import {useOpenapi} from "./hooks/useOpenapi";
import _ from 'lodash'

function AvoRedApiTesting() {
    const [openapi, setOpenapi] = useState({})

    const {data} = useOpenapi()


    useMemo(() => {

        setOpenapi(JSON.stringify(_.get(data, 'data')))
    }, [data])


    return (
        <div className="flex-1 bg-white">
            <div className="px-5 ml-64">
                <SwaggerUI spec={openapi} />
            </div>
        </div>
    )
}

export default AvoRedApiTesting