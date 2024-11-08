import axios from 'axios';

const baseURL = import.meta.env.VITE_AVORED_BACKEND_BASE_URL + "/"
const token = import.meta.env.VITE_AVORED_CMS_TOKEN
export default axios.create({
    baseURL
})

// axios instance with 'withCredentials' flag
export const client = axios.create({
    baseURL,
    headers : {
        'Content-Type' : 'application/json',
        'Authorization': 'Bearer ' + token
    },
    withCredentials: false
})
