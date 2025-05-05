import avoredLogo from '../../assets/avored.svg'

const AppHeader = (() => {
    return (
        <div className="flex bg-white-500 px-5 py-3">
            <div className='flex justify-center align-middle'>
                <img src={avoredLogo} className="w-8 h-8" alt="AvoRed CMS"/>
                <span className='text-primary-500 font-semibold '>
                    Avored Rust CMS
                    </span>
            </div>
            <div className='ml-auto flex'>

            </div>
        </div>
    )
})

export default AppHeader
