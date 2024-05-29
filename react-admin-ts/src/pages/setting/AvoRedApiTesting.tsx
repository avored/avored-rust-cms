import SwaggerUI from "swagger-ui-react";
import "swagger-ui-react/swagger-ui.css";
import { useMemo, useState, FC } from "react";
import { useOpenapi } from "./hooks/useOpenapi";
import _ from "lodash";

interface OpenApiData {
  data: any;
}

const AvoRedApiTesting: FC = () => {
  const [openapi, setOpenapi] = useState<string>("");
  const { data } = useOpenapi() as { data: OpenApiData };

  useMemo(() => {
    setOpenapi(JSON.stringify(_.get(data, "data")));
  }, [data]);

  return (
    <div className="flex-1 bg-white">
      <div className="px-5 ml-64">
        <SwaggerUI spec={JSON.parse(openapi)} />
      </div>
    </div>
  );
};

export default AvoRedApiTesting;
