import avoredLogo from '/avored.svg'
import {useState} from "react";

const Dashboard = (() => {
    const [count, setCount] = useState(0)
    return(
        <div className="flex w-full justify-center items-center h-screen">
            <div>
                <a href="https://avored.com" target="_blank">
                    <img src={avoredLogo} className="w-24 h-24" alt="AvoRed CMS"/>
                </a>

                <h1 className="mt-5 text-3xl font-bold underline">
                    AvoRed Cms deploy 1
                </h1>
                <div className="mt-5">
                    <button
                        className="bg-purple-500 text-white px-4 py-3 rounded"
                        onClick={() => setCount((count) => count + 1)}
                    >
                        count is {count}
                    </button>

                </div>
            </div>
        </div>
    )
})

export default Dashboard