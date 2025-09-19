import { useEffect } from "react";
import _ from 'lodash';
import axios from 'axios';

const baseURL = import.meta.env.REACT_APP_BACKEND_BASE_URL + "/api"

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

export const UseAxiosHook = () => {
    useEffect(() => {
        const reqInterceptor = client.interceptors.request.use(
            config => {
                const token = localStorage.getItem("token");
                if (!_.isEmpty(token) && !config.headers['Authorization']) {
                    config.headers['Authorization'] = `Bearer ${token}` 
                }

                return config;
            }, (error) => Promise.reject(error)
        )

        return () => {
            client.interceptors.request.eject(reqInterceptor);
        }
    }, [])

    return client;
}