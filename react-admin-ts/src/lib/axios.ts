import axios from 'axios';

const baseURL = process.env.REACT_APP_AVORED_BACKEND_BASE_URL + "/api"
export default axios.create({
    baseURL
})

// axios instance with 'withCredentials' flag
export const client = axios.create({
    baseURL,
    headers : {
        'Content-Type' : 'application/json'
    },
    withCredentials: false
})