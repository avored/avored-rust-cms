import {ContentSidebar} from "./ContentSidebar";

export const ContentTablePage = () => {
    return(
        <>
            <div className="flex w-full">
                <div className="p-5 w-48 bg-gray-50 min-h-screen">
                    <ContentSidebar />
                </div>
                <div className="p-5 flex-1">

                </div>
            </div>
        </>
    )

}