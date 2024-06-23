import _ from "lodash";
import IErrorMessage from "../types/common/IError";

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

        return _.findIndex(_.get(props.backendErrors, 'response.data.errors', []), ((err: IErrorMessage) => err.key === key))
    }

    const getErrorMessage = (key: string) => {
        let message = _.get(props.frontendErrors, key + '.message')

        if (message) {
            return message;
        }
        return _.get(props.backendErrors, "response.data.errors." + getErrorIndex(props.identifier) + ".message")
    }

    return (
        <>
            {getErrorIndex(props.identifier) >= 0 ? (
                <p className="text-red-500 mt-1 text-xs font-semibold">
                    {getErrorMessage(props.identifier)}
                </p>
            ) :  ''}

        </>
    )

}

export default ErrorMessage;