import axios from "axios";

// const injectAccessToken = (config: InternalAxiosRequestConfig) => {
//   const accessToken = localStorage.getItem("access_token");
//   if (accessToken)
//     config.headers!.common["Authorization"] = `Bearer ${accessToken}`;
//   return config;
// };


// const accessToken = localStorage.getItem("access_token");
  
const config = {
  baseURL: "https://purvesh.northcentralus.cloudapp.azure.com",
  headers: {
      // accessToken ? 'Authoziation': 'Bearer ' + accessToken : null
  }
};

const avoRedRustApi = axios.create(config);

// API.interceptors.request.use(injectAccessToken);

// export const avoredApiGet = () => {
//   avoredApiGet.get(uri, headers)
// }
export default avoRedRustApi;
