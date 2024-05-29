// import React from "react";
// import _ from "lodash";

type InputFieldProps = {
  label?: string;
  id?: string;
  type?: string;
  name: string;
  required?: boolean;
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  autoFocus?: boolean;
  placeholder?: string;
  register?: string;
  errorMessages: string[];
  accept?: string;
};

const InputField: React.FC<InputFieldProps> = ({
  label,
  type,
  name,
  onChange,
  accept,
  value,
  autoFocus,
  errorMessages,
}) => {
  return (
    <div>
      <label
        htmlFor={name}
        className="block text-sm font-medium text-gray-700 dark:text-gray-200"
      >
        {label}
      </label>
      <input
        type={type}
        name={name}
        id={name}
        className="mt-1 block w-full px-3 py-2 border border-gray-300 dark:border-gray-700 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
        onChange={onChange}
        value={value}
        autoFocus={autoFocus}
        accept={accept}
      />
      {errorMessages.map((error, index) => (
        <p key={index} className="text-red-500 text-xs italic">
          {error}
        </p>
      ))}
    </div>
  );
};

export default InputField;
