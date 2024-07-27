import { ContactSection } from "./ContactSection";
import FeaturesSection from "./FeaturesSection";
import MainHeroSection from "./MainHeroSection";
import RepositoryInformation from "./RepositoryInformation";

const DashboardPage = () => {
  return (
    <>
      <div className="relative bg-white overflow-hidden">
        <MainHeroSection />
        <div className="lg:absolute lg:inset-y-0 lg:right-0 lg:w-1/2">
          <img
            className="h-full p-6 hidden lg:block object-cover"
            src="/assets/main-hero.e867feaf.svg"
            alt=""
          />
        </div>
      </div>

      <RepositoryInformation />
      <FeaturesSection />
      <ContactSection />
    </>
  );
};
export default DashboardPage;
