import React from "react";

const InputField = (props) => {
  return (
    <div>
      <label htmlFor={props.name} className="text-sm text-gray-600">
        {props.label ?? ''}
      </label>
      <div className="mt-1">
        <input
          type={props.type}
          name={props.name}
          required={props.required}
          value={props.value}
          onChange={props.onChange}
          autoFocus={props.autoFocus}
          className="appearance-none rounded-md ring-1 ring-gray-400 active::ring-primary-500 focus:ring-primary-500 relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900 focus:outline-none sm:text-sm focus:z-10"
          placeholder={props.placeholder ?? props.label}
        />
      </div>
    </div>
  );
};

export default InputField;
