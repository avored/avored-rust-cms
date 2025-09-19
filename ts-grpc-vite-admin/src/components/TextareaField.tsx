import _ from "lodash";
import React from "react";

type AvoRedTextareaFieldProps = {
    name?: string;
    id?: string;
    label?: string;
    placeholder?: string;
    value?: string | number;
    register: any;
}

export const TextareaField = (props: AvoRedTextareaFieldProps) => {
    return (
        <>
            <div>
                {(!_.isEmpty(props.label)) ? (
                    <label htmlFor={props.name} className="text-sm text-gray-600">
                        {props.label}
                    </label>
                ) : (
                    ""
                )}

                <div className="mt-1">
                    <textarea
                        id={props.id ?? props.name}
                        name={props.name}
                        className="appearance-none rounded-md ring-1 ring-gray-400
                            relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900
                            active::ring-primary-500
                            focus:ring-primary-500 focus:outline-none focus:z-10
                            disabled:bg-gray-200 disabled:opacity-70
                            sm:text-sm "
                        placeholder={props.placeholder ?? props.label}
                        {...props.register}
                    >{props.value}</textarea>
                </div>
            </div>
        </>
    )
}
