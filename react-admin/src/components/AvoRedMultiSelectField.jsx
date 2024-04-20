import React, { Fragment } from "react";
import {Listbox, Transition} from "@headlessui/react";
import {CheckIcon, ChevronUpDownIcon} from "@heroicons/react/20/solid";

const AvoRedMultiSelectField = ({
                        label,
                        options = [],
                        selectedOption,
                        onChangeSelectedOption
                    }) => {

    const getOption = ((optionId) => {
        return options.find(option => option.value === optionId)
    })

    return (
        <Listbox value={selectedOption} onChange={onChangeSelectedOption} multiple>
            <div className="relative mt-1">
                <Listbox.Label className="text-gray-600 text-sm">
                    {label}
                </Listbox.Label>
                <Listbox.Button
                    className="w-full cursor-default h-8 rounded py-2 pl-3 pr-10 text-left text-sm shadow ring-1 ring-gray-400 focus:ring-primary-500 active:ring-primary-500 focus:outline-none"
                >
                    <span className="block min-h:[5]">
                        {selectedOption.map((optionId) => getOption(optionId).label ?? '').join(", ")}
                    </span>
                    <span className="pointer-events-none absolute inset-y-10  right-0 flex items-center pr-2">
                        <ChevronUpDownIcon
                              className="h-5 w-5 text-gray-400"
                              aria-hidden="true"
                          />
                    </span>
                </Listbox.Button>

                <Transition
                    as={Fragment}
                    leave="transition ease-in duration-100"
                    leaveFrom="opacity-100"
                    leaveTo="opacity-0"
                >
                    <Listbox.Options className="absolute mt-1 max-h-60 w-full overflow-auto rounded bg-white py-1 text-base shadow ring-1 ring-black/5 focus:outline-none sm:text-sm">
                        {options.map((option) => (
                            <Listbox.Option
                                key={option.value}
                                className={(({active}) => {
                                    return `relative cursor-default select-none py-2 pl-10 pr-4 ${active ? 'bg-primary-100 text-primary-900' : 'text-gray-900'}`
                                })}
                                value={option.value}
                            >
                                {({ selected, active }) => (
                                    <>
                                      <span
                                          className={`block truncate ${
                                              selected ? 'font-medium' : 'font-normal'
                                          }`}
                                      >
                                        {option.label}
                                      </span>
                                        {(selected) ? (
                                            <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-primary-600">
                                                <CheckIcon className="h-5 w-5" aria-hidden="true" />
                                            </span>
                                        ) : null}
                                    </>
                                )}
                            </Listbox.Option>
                        ))}
                    </Listbox.Options>
                </Transition>
            </div>
        </Listbox>
);
};

export default AvoRedMultiSelectField;
