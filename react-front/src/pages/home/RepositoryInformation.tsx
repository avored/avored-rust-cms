import {
  ArrowDownTrayIcon,
  PuzzlePieceIcon,
  StarIcon,
} from "@heroicons/react/24/solid";
import { useRepositoryInformation } from "./hooks/useRepositoryInformation";
import {ComponentContentType} from "../../types/CmsPageType";
import {GetElementValue} from "../../lib/page";

type RepositoryComponentProps = {
  RepositoryComponent?: ComponentContentType
}

const RepositoryInformation = (props: RepositoryComponentProps) => {
  console.log(props)
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
            <h5 className="text-lg p-3 heading-font font-semibold">
              {GetElementValue(props.RepositoryComponent, 'rate-us-title')}
            </h5>
            <p className="text-sm">
              {GetElementValue(props.RepositoryComponent, 'rate-us-description')}
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
              {GetElementValue(props.RepositoryComponent, 'commit-number')}
            </div>
          </div>
          <div className="ml-3 group-hover:text-white">
            <h5 className="text-lg p-3 heading-font font-semibold">
              {GetElementValue(props.RepositoryComponent, 'commit-title')}
            </h5>
            <p className="text-sm">
              {GetElementValue(props.RepositoryComponent, 'commit-description')}
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
              {GetElementValue(props.RepositoryComponent, 'contribute-number')}
            </div>
          </div>
          <div className="ml-3">
            <h5 className="text-lg group-hover:text-white p-3 heading-font font-semibold">
              {GetElementValue(props.RepositoryComponent, 'contribute-title')}
            </h5>
            <p className="text-sm">
              {GetElementValue(props.RepositoryComponent, 'contribute-description')}
            </p>
          </div>
        </div>
      </div>
      <hr />
    </>
  );
};

export default RepositoryInformation;
