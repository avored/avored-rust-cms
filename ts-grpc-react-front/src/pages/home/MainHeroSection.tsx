import {PageFieldType} from "../../types/CmsPageType";
import {GetElementValue} from "../../lib/page";

type PageFieldsProps = {
    page_fields: PageFieldType[]
}

const MainHeroSection = ((props: PageFieldsProps) => {
    return (
        <>
        <div className="mx-auto">
          <div className="relative z-10 pb-8 bg-white sm:pb-16 md:pb-20 lg:max-w-2xl lg:w-full lg:pb-28 xl:pb-32">
            <div className="mt-10 mx-auto px-4 sm:mt-12 sm:px-6 md:mt-16 lg:mt-20 lg:px-8 xl:mt-28">
              <div className="sm:text-center lg:text-left">
                <h2 className="text-3xl tracking-tight leading-10 heading-font font-extrabold text-gray-900 sm:text-4xl sm:leading-none md:text-5xl">
                    {GetElementValue(props.page_fields, 'hero-page-title')}
                </h2>

                <p className="mt-3 text-gray-500 sm:mt-5  sm:max-w-xl sm:mx-auto md:mt-5 lg:mx-0">
                    {GetElementValue(props.page_fields, 'hero-page-description')}
                </p>
                <div className="mt-5 sm:mt-8 sm:flex sm:justify-center lg:justify-start">
                  <div className="rounded-md shadow">
                    <a
                      href={GetElementValue(props.page_fields, 'get-started-button-url')}
                      target="_blank"
                      className="w-full flex items-center justify-center bg-gradient-to-b from-primary-700 via-primary-600 to-primary-700 hover:bg-gradient-to-t hover:from-primary-800 hover:via-primary-700 hober:to-primary-700 px-8 py-3 border border-transparent text-base leading-6 font-medium rounded-md text-white bg-primary-600 hover:bg-primary-500 hover:text-white focus:outline-none focus:shadow-outline transition duration-150 ease-in-out md:py-4 md:text-lg md:px-10"
                    >
                        {GetElementValue(props.page_fields, 'get-started-button-title')}
                    </a>
                  </div>
                  <div className="mt-3 sm:mt-0 sm:ml-3">
                    <a
                      href={`${GetElementValue(props.page_fields, "demo-button-url")}`}
                      target="_blank"
                      className="w-full flex items-center justify-center px-8 py-3 border bg-gradient-to-b from-gray-400 via-gray-300 to-gray-400 hover:bg-gradient-to-t hover:from-gray-300 hover:via-gray-400 hober:to-gray-300 border-transparent text-base leading-6 font-medium rounded-md text-white-700 bg-primary-100 hover:text-gray-700 hover:bg-primary-50 focus:outline-none focus:shadow-outline focus:border-primary-300 transition duration-150 ease-in-out md:py-4 md:text-lg md:px-10"
                    >
                        {GetElementValue(props.page_fields, 'demo-button-title')}
                    </a>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        </>
    )
})

export default MainHeroSection
