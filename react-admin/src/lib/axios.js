import axios from 'axios';

const baseURL = process.env.REACT_APP_SERVER_URL 
export default axios.create({
    baseURL 
})

// axios instance with 'withCredentials' flag
export const client = axios.create({
    baseURL,
    headers : {
        'Content-Type' : 'application/json'
    },
    withCredentials: true
})