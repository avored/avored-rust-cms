import React from 'react';
import {useHealthCheckHook} from "./hooks/useHealthCheckHook";
import {HealthCheckRequest} from "./grpc_generated/misc_pb";

function App() {
    const request = new HealthCheckRequest();
    const {data} = useHealthCheckHook(request);

    console.log('status', data?.getStatus());

    return (
        <h1 className="flex items-center justify-center h-screen w-full">
            <div className="text-3xl font-semibold text-red-600">
                Avored rust grpc  cms
            </div>

        </h1>
    );
}

export default App;
