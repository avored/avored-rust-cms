import { ContactSection } from "./ContactSection";
import FeaturesSection from "./FeaturesSection";
import MainHeroSection from "./MainHeroSection";
import RepositoryInformation from "./RepositoryInformation";
import {useHomeCmsPage} from "./hooks/useHomeCmsPage'";
import _ from "lodash";
import {ComponentContentType} from "../../types/CmsPageType";

const HomePage = () => {
    const {data: home_page_model} = useHomeCmsPage()
    const GetComponent = ((component_identifier: string): ComponentContentType | undefined => {
        const components =  _.get(home_page_model, 'data.data.page_model.components_content', [])
        return components.find((com: ComponentContentType) => {
            return com.identifier === component_identifier
        })
    })
    // console.log(home_page_model?.data.data?.page_model?.components_content)
  return (
    <>
      <div className="relative bg-white overflow-hidden">
        <MainHeroSection HeroComponent={GetComponent('hero-component')} />

        <div className="lg:absolute lg:inset-y-0 lg:right-0 lg:w-1/2">
          <img
            className="h-96 pt-20 mt-5 hidden lg:block object-cover"
            src="/main-hero.svg"
            alt="avored rust main hero image"
          />
        </div>
      </div>

      <RepositoryInformation RepositoryComponent={GetComponent('repository-component')} />
      <FeaturesSection KeyFeaturesComponent={GetComponent('key-features-component')} />
      <ContactSection ContactUsComponent={GetComponent('contact-us-component')} />
    </>
  );
};
export default HomePage;
