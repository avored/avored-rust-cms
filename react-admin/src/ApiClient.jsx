import axios from "axios";

var test = axios.create({
    baseURL: process.env.REACT_APP_AVORED_BACKEND_BASEURL + "/api",
    headers: {
        "Content-type": "application/json"
    }
});
export default test