import _ from "lodash";
import {ErrorMessageType, GrpcErrorCode} from "../types/common/ErrorType";
import {Switch} from "@headlessui/react";
import AvoredModal from "./AvoredModal";

type ErrorMessageProps = {
    backendErrors: any;
    frontendErrors: any;
    identifier: string;
}

function ErrorMessage(props: ErrorMessageProps) {
    const getErrorIndex = (key: string): number => {

        let message = _.get(props.frontendErrors, key + '.message');
        if (message) {
            return 1;
        }

        let backendErrorMessages = JSON.parse('{"errors": []}');
        switch (props.backendErrors?.code) {
            case GrpcErrorCode.InvalidArgument:
                backendErrorMessages = JSON.parse(props.backendErrors?.message ?? '{"errors": []}');
                break;
            default:
                // do nothing as it should be handled by global query client
                //
                break;
        }

        return _.findIndex(_.get(backendErrorMessages, 'errors', []), ((err: ErrorMessageType) => err.key === key))
    }

    const getErrorMessage = (key: string) => {
        let message = _.get(props.frontendErrors, key + '.message')

        if (message) {
            return message;
        }
        let backendErrorMessages = JSON.parse('{"errors": []}');
        switch (props.backendErrors?.code) {
            case GrpcErrorCode.InvalidArgument:
                backendErrorMessages = JSON.parse(props.backendErrors?.message ?? '{"errors": []}');
                break;
            default:
                // do nothing as it should be handled by global query client
                //
                break;
        }

        return _.get(backendErrorMessages, "errors." + getErrorIndex(props.identifier) + ".message")
    }

    return (
        <>
            {getErrorIndex(props.identifier) >= 0 ? (
                <p className="text-red-500 mt-1 text-xs font-semibold">
                    {getErrorMessage(props.identifier)}
                </p>
            ) : ''}

        </>
    )

}

export default ErrorMessage;
