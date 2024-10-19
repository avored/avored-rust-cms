import {ReactNode} from "react";

type ButtonPropsType = {
  icon: ReactNode
}

const AvoRedIconButton = (props: ButtonPropsType) => {
  const { icon } = props;
  return (
      <button className='bg-primary-600 hover:bg-primary-500 focus:ring-primary-500 w-full flex justify-center py-0.1 px-1 border border-transparent text-sm font-medium rounded-md text-white focus:outline-none focus:ring-2 focus:ring-offset-2'>
        {icon}
      </button>
  )
}

export default AvoRedIconButton
