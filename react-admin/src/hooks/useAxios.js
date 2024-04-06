import { useEffect } from "react";
import {client} from '../lib/axios';

export const useAxios = () => {
    useEffect(() => {
        const reqInterceptor = client.interceptors.request.use(
            config => {
                if (!config.headers['Authorization']) {
                    const token = localStorage.getItem("AUTH_TOKEN");
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