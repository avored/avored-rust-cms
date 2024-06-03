import axios from 'axios';

// @ts-ignore
const baseURL = import.meta.env.VITE_AVORED_BACKEND_BASE_URL + "/api"


// axios instance with 'withCredentials' flag
export const client = axios.create({
    baseURL,
    headers : {
        'Content-Type' : 'application/json'
    },
    withCredentials: false
})