import { useEffect } from "react";
import {client} from '../lib/axios';
import _ from 'lodash';

export const useAxios = () => {
    useEffect(() => {
        const reqInterceptor = client.interceptors.request.use(
            config => {
                const token = localStorage.getItem("AUTH_TOKEN");
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
