import _ from "lodash";
import {ErrorMessageType} from "../types/common/ErrorType";

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

        console.log(props.backendErrors?.message, "compo");
        const backendErrorMessages = JSON.parse(props.backendErrors?.message ?? '{"errors": []}');

        return _.findIndex(_.get(backendErrorMessages, 'errors', []), ((err: ErrorMessageType) => err.key === key))
    }

    const getErrorMessage = (key: string) => {
        let message = _.get(props.frontendErrors, key + '.message')

        if (message) {
            return message;
        }
        const backendErrorMessages = JSON.parse(props.backendErrors?.message ?? '{"errors": []}');

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