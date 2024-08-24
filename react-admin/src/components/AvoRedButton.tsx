enum ButtonType {
    submit = "submit",
    button = "button",
}

type ButtonPropsType = {
    isPending?: boolean;
    label: string,
    type?: ButtonType,
    className?: string
}

const AvoRedButton = (({
    label,
    isPending,
    type = ButtonType.submit,
    className = ""
}: ButtonPropsType) => {
    return (
        <>
            <button
                type={type}
                className={`bg-gray-300 w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white focus:outline-none focus:ring-2 focus:ring-offset-2 ${className}`}
            >
                {isPending ? "Loading..." : label}
            </button>
        </>
    )
})

export default AvoRedButton
