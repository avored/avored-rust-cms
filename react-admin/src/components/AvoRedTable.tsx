import {Menu, MenuButton, MenuItem, MenuItems, Transition} from "@headlessui/react"
import {ChevronDownIcon, ChevronUpIcon} from "@heroicons/react/24/solid"
import {Column, flexRender, Table} from "@tanstack/react-table"


interface Props {
    table: Table<any>
}

const AvoRedTable = (props: Props) => {
    const getColumnName = ((column: Column<any>) => {
        return (typeof column.columnDef.header === 'string')  ? column.columnDef.header : column.id
    })
    return (
        <>
            <div className="px-1 border-b flex w-full">
                <Menu>
                    <MenuButton
                        className="inline-flex mb-3 ml-auto items-center gap-2 rounded-md bg-gray-400 py-2 px-3 text-sm font-semibold text-white hover:bg-gray-500">
                        Toggle
                        <ChevronDownIcon className="size-4 text-white"/>
                    </MenuButton>
                    <Transition
                        enter="transition ease-out duration-75"
                        enterFrom="opacity-0 scale-95"
                        enterTo="opacity-100 scale-100"
                        leave="transition ease-in duration-100"
                        leaveFrom="opacity-100 scale-100"
                        leaveTo="opacity-0 scale-95"
                    >
                        <MenuItems
                            as="div"
                            anchor="bottom end"
                            className="w-52 origin-top-right rounded-xl border border-white/5 bg-white/5 p-1 text-sm/6 text-white [--anchor-gap:var(--spacing-1)] focus:outline-none"
                        >
                            {props.table.getAllLeafColumns().map(column => {
                                return (
                                    <div key={column.id} className="bg-gray-400 px-3 py-1 text-white">
                                        <MenuItem as="div">
                                            <>
                                                <input
                                                    id={`column-checkbox-visible-${column.id}`}
                                                    className="rounded"
                                                    {...{
                                                        type: 'checkbox',
                                                        checked: column.getIsVisible(),
                                                        onChange: column.getToggleVisibilityHandler(),
                                                    }}
                                                />
                                                <label htmlFor={`column-checkbox-visible-${column.id}`} className="pl-3">
                                                    {getColumnName(column) }
                                                </label>
                                            </>
                                        </MenuItem>
                                    </div>
                                )
                            })}
                        </MenuItems>
                    </Transition>
                </Menu>
            </div>
            <table className="w-full bg-white shadow-md rounded">
                <thead>
                {props.table.getHeaderGroups().map(headerGroup => (
                    <tr key={headerGroup.id} className="bg-gray-700 text-white">
                        {headerGroup.headers.map(header => (
                            <th key={header.id} className="py-3 px-4 font-semibol text-left">
                                <div
                                    {...{
                                        className: header.column.getCanSort()
                                            ? 'select-none cursor-pointer flex items-center gap-1'
                                            : '',
                                        onClick: header.column.getToggleSortingHandler(),
                                    }}
                                >
                                    {header.isPlaceholder
                                        ? null
                                        : flexRender(
                                            header.column.columnDef.header,
                                            header.getContext()
                                        )}
                                    {{
                                        asc: <ChevronDownIcon className="h-4 w-4"/>,
                                        desc: <ChevronUpIcon className="h-4 w-4"/>,
                                    }[header.column.getIsSorted() as string] ?? null}
                                </div>
                            </th>
                            ))}
                    </tr>
                ))}
                </thead>
                <tbody>
                {props.table.getRowModel().rows.map(row => (
                    <tr key={row.id} className="border-b">
                        {row.getVisibleCells().map(cell => (
                            <td key={cell.id} className="py-3 px-4">
                                {flexRender(cell.column.columnDef.cell, cell.getContext())}
                            </td>
                        ))}
                    </tr>
                ))}
                </tbody>
            </table>
        </>
    )
}
export default AvoRedTable
