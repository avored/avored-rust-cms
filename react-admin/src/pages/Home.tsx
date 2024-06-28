import { ChevronRightIcon } from "@heroicons/react/24/solid";
import logo from "../assets/logo_only.svg";
function Dashboard() {
  return (
    <div className="relative sm:flex sm:justify-center sm:items-center min-h-screen bg-center bg-gray-100 selection:bg-primary-500 selection:text-white">
      <div className="max-w-7xl mx-auto p-6 lg:p-8">
        <div className="flex justify-center">
          <img src={logo} className="w-20 h-20" alt="Avored Rust Cms" />
        </div>

        <div className="mt-16">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 lg:gap-8">
            <a
              href="/admin/login"
              className="scale-100 p-6 bg-white rounded-lg shadow-2xl shadow-gray-500/20 flex  focus:outline focus:outline-2 focus:outline-primary-500"
            >
              <div>
                <div className="h-16 w-16 bg-primary-50 flex items-center justify-center rounded-full">
                  <i
                    className="w-7 h-7 stroke-primary-500"
                    data-feather="framer"
                  ></i>
                </div>

                <h2 className="mt-6 text-xl font-semibold text-gray-900">
                  Administrator
                </h2>

                <p className="mt-4 text-gray-500 text-sm leading-relaxed">
                  An app administrator plays a pivotal role in ensuring the
                  smooth operation of the avored cms. They are responsible for
                  managing user access, content updates and overseeing updates.
                  The administrator acts as the gatekeeper, safeguarding the
                  app's content and user experience while facilitating its
                  growth and improvement. In essence, they are the backbone of a
                  well-maintained and secure avored rust cms ecosystem.
                </p>
              </div>
              <div className="self-center pl-5">
                <ChevronRightIcon className="w-6 h-6" />
              </div>
            </a>

            <a
              href="https://github.com/avored/avored-rust-cms"
              target="_blank"
              rel="noopener noreferrer"
              className="scale-100 p-6 bg-white  rounded-lg shadow-2xl flex  focus:outline focus:outline-2 focus:outline-primary-500"
            >
              <div>
                <div className="h-16 w-16 bg-primary-50 flex items-center justify-center rounded-full">

                  <i
                    className="w-7 h-7 stroke-primary-500"
                    data-feather="github"
                  ></i>
                </div>

                <h2 className="mt-6 text-xl font-semibold text-gray-900">
                  Github
                </h2>

                <p className="mt-4 text-gray-500 text-sm leading-relaxed">
                  GitHub Star: When you "star" a GitHub repository, it signifies
                  your interest in that project. Starring a repository also
                  sends a signal to the repository owner that you appreciate
                  their work or find it valuable. Community Help: The GitHub
                  community is known for its collaborative nature. Developers
                  frequently help each other by contributing to open-source
                  projects, reporting issues, suggesting improvements, and even
                  submitting pull requests to fix bugs or add features. This
                  collective effort is essential for the growth and success of
                  open-source projects hosted on GitHub.
                </p>
              </div>

              <div className="self-center pl-5">
                <i
                  className="w-6 h-6 stroke-primary-500"
                  data-feather="chevrons-right"
                ></i>
              </div>
            </a>
          </div>
        </div>

        <div className="flex justify-center mt-16 px-0 sm:items-center sm:justify-between">
          <div className="text-center text-sm text-gray-500  sm:text-left">
            <div className="flex items-center gap-4">
              <a
                href="https://github.com/sponsors/indpurvesh"
                className="group inline-flex items-center hover:text-gray-700 dark:hover:text-white focus:outline focus:outline-2 focus:rounded-sm focus:outline-primary-500"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  strokeWidth="1.5"
                  className="-mt-px mr-1 w-5 h-5 stroke-gray-400 dark:stroke-gray-600 group-hover:stroke-gray-600 dark:group-hover:stroke-gray-400"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z"
                  />
                </svg>
                Sponsor
              </a>
            </div>
          </div>

          <div className="ml-4 text-center text-sm text-gray-500 sm:text-right sm:ml-0">
            AvoRed Rust CMS
          </div>
        </div>
      </div>
    </div>
  );
}

export default Dashboard;
