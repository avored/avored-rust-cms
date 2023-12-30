import React from "react";

const InputField = ({
    label,
    type = "text",
    name,
    value,
    onChange,
    autoFocus,
    placeholder,
    required
}) => {
  return (
    <div>
      <label htmlFor={name} className="text-sm text-gray-600">
        {label}
      </label>
      <div className="mt-1">
        <input
          type={type}
          name={name}
          required={required}
          value={value}
          onChange={onChange}
          autoFocus={autoFocus}
          className="appearance-none rounded-md ring-1 ring-primary-300 relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-primary-500 sm:text-sm focus:z-10"
          placeholder={placeholder ?? label}
        />
      </div>
    </div>
  );
};

export default InputField;
