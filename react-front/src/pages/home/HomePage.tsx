import { ContactSection } from "./ContactSection";
import FeaturesSection from "./FeaturesSection";
import MainHeroSection from "./MainHeroSection";
import RepositoryInformation from "./RepositoryInformation";
import {useHomeCmsPage} from "./hooks/useHomeCmsPage";
import _ from "lodash";
import {PageFieldType} from "../../types/CmsPageType";

const HomePage = () => {
    const {data} = useHomeCmsPage()
    const GetPageFields = ((): Array<PageFieldType> => {
        return _.get(data, 'data.data.page_model.page_fields', [])
    })
  return (
    <>
      <div className="relative bg-white overflow-hidden">
        <MainHeroSection page_fields={GetPageFields()} />

        <div className="lg:absolute lg:inset-y-0 lg:right-0 lg:w-1/2">
          <img
            className="h-96 pt-20 mt-5 hidden lg:block object-cover"
            src="/main-hero.svg"
            alt="avored rust main hero image"
          />
        </div>
      </div>

      <RepositoryInformation RepositoryComponent={GetPageFields()} />
      <FeaturesSection KeyFeaturesComponent={GetPageFields()} />
      <ContactSection ContactUsComponent={GetPageFields()} />
    </>
  );
};
export default HomePage;
