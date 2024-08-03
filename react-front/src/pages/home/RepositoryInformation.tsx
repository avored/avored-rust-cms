import {
  ArrowDownTrayIcon,
  PuzzlePieceIcon,
  StarIcon,
} from "@heroicons/react/24/solid";
import { useRepositoryInformation } from "./hooks/useRepositoryInformation";

const RepositoryInformation = () => {
  const repositoryInformation = useRepositoryInformation();
  return (
    <>
      <hr />
      <div className="md:flex mt-5 pb-5">
        <div className="md:flex ml-3 group hover:bg-primary-600 rounded-lg p-5">
          <div className="flex-1">
            <div className="text-4xl group-hover:text-white text-primary-600">
              <StarIcon className="h-12 w-12" />
            </div>
            <div className="text-lg group-hover:text-white font-semibold mt-3">
              {/*{repositoryInformation.data?.data.data.star ?? 65}*/}
              65
            </div>
          </div>
          <div className="ml-3 group-hover:text-white">
            <h5 className="text-lg p-3 heading-font font-semibold">Rate Us</h5>
            <p className="text-sm">
              Help us by giving a star on GitHub. Spread the word by
              recommending AvoRed to your network and help to get the better
              product.
            </p>
          </div>
        </div>
        <div className="md:flex ml-3 group hover:bg-primary-600 hover:text-white rounded-lg p-5">
          <div className="flex-1">
            <div className="text-4xl">
              <div className="text-4xl group-hover:text-white text-primary-600">
                <ArrowDownTrayIcon className="h-12 w-12" />
              </div>
            </div>
            <div className="text-lg group-hover:text-white font-semibold mt-3">
              {/*{repositoryInformation.data?.data.data.commit ?? 1234}*/}
              1234
            </div>
          </div>
          <div className="ml-3 group-hover:text-white">
            <h5 className="text-lg p-3 heading-font font-semibold">Commit</h5>
            <p className="text-sm">
              Our project has over 500 commits, showcasing our team's
              dedication. Each commit enhances features, fixes bugs, and
              improves performance, ensuring a high-quality software product.
            </p>
          </div>
        </div>
        <div className="md:flex ml-3 group hover:bg-primary-600 hover:text-white rounded-lg p-5">
          <div className="flex-1">
            <div className="text-4xl">
              <div className="text-4xl group-hover:text-white text-primary-600">
                <PuzzlePieceIcon className="w-12 h-12" />
              </div>
            </div>
            <div className="text-lg group-hover:text-white font-semibold mt-3">
              {/*{JSON.stringify(repositoryInformation.data?.data.data.contribute)}*/}
              5
            </div>
          </div>
          <div className="ml-3">
            <h5 className="text-lg group-hover:text-white p-3 heading-font font-semibold">
              Contribute
            </h5>
            <p className="text-sm">
              Wish to contribute, AvoRed is 100% free and open-source under the
              GPL-3.0 license. Fork it on GitHub and help make it better
            </p>
          </div>
        </div>
      </div>
      <hr />
    </>
  );
};

export default RepositoryInformation;
