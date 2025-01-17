import { useState } from 'react';
import * as React from 'react';
import { AuthenticationContext } from '../../hooks/useAuthentication';
import { AccessToken, API_BACKEND_URL } from '../../api';

export const AuthenticationProvider = ({ children }: { children: React.ReactNode }) => {
    const [token, setToken] = useState<AccessToken>(() => {
        const tokenSessionStorage = window.sessionStorage.getItem('thereiwas:token');
        if (tokenSessionStorage) {
            return JSON.parse(tokenSessionStorage);
        }
        return { accessToken: '' };
    });

    const signin = (username: string, password: string, successCallback: VoidFunction, failCallback: VoidFunction) => {
        // prepare the data for the authentication request
        const requestData = {
            username,
            password,
        };

        // try to get the token from the backend with the supplied information
        fetch(`${API_BACKEND_URL}/auth/token`, {
            method: 'POST',
            body: JSON.stringify(requestData),
            headers: {
                'Content-type': 'application/json; charset=UTF-8',
            },
        })
            .then((response) => {
                if (response.status !== 200) {
                    return Promise.reject();
                }
                return response;
            })
            .then((response) => response.json())
            .then((receivedToken: AccessToken) => {
                setToken(receivedToken);
                window.sessionStorage.setItem('thereiwas:token', JSON.stringify(receivedToken));
                successCallback();
            })
            .catch(() => {
                failCallback();
            });
    };

    const signout = (callback: VoidFunction) => {
        setToken({ accessToken: '' });
        window.sessionStorage.removeItem('thereiwas:token');
        callback();
    };

    const value = { token, signin, signout };

    return <AuthenticationContext.Provider value={value}>{children}</AuthenticationContext.Provider>;
};