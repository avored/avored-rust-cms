import React from "react";
import { ChevronDownIcon } from "@heroicons/react/24/solid";

type SelectFieldProps = {
    name: string;
    label: string;
    id?: string;
    required?: boolean;
    value?: string;
    onChange?: (e: React.ChangeEvent<HTMLSelectElement>, ...args: any[]) => void;
    disabled?: boolean;
    register: any;
    children: React.ReactNode;
}

const SelectField = (props: SelectFieldProps) => {
    return (
        <div>
            <label htmlFor={props.id ?? props.name} className="text-sm text-gray-600">
                {props.label}
            </label>

            <div className="mt-1 relative">
                <select
                    disabled={props.disabled ?? false}
                    id={props.id ?? props.name}
                    name={props.name}
                    required={props.required}
                    value={props.value}
                    onChange={props.onChange}
                    className="appearance-none rounded-md ring-1 ring-gray-400 relative border-0 block w-full px-3 py-2 pr-10 text-gray-900 focus:ring-primary-500 focus:outline-none focus:z-10 disabled:bg-gray-200 disabled:opacity-70 sm:text-sm"
                    {...props.register}
                >
                    {props.children}
                </select>
                <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
                    <ChevronDownIcon className="h-5 w-5 text-gray-400" />
                </div>
            </div>
        </div>
    );
};

export default SelectField;
