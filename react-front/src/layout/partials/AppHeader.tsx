import avoredLogo from '/avored.svg'

const AppHeader = (() => {
    return (
        <div className="flex w-full bg-white-500 px-5 py-3">
            <div className='flex justify-center align-middle'>
                <img src={avoredLogo} className="w-8 h-8" alt="AvoRed CMS"/>
                <span className='text-primary-500 font-semibold '>
                    Avored Rust CMS
                    </span>
            </div>
            <div className='ml-auto flex'>
                {/* <Link className='mr-3' to="/">
                    Docs
                </Link>
                <div className='mr-3 cursor-pointer text-primary-500 font-semibold' data-hs-overlay="#avored-get-started-modal">
                    Get started
                </div> */}
                {/* <GetStartedModal /> */}
            </div>
        </div>
    )
})

export default AppHeader
