const determineApiUrl = () => {
  if (window.location && import.meta.env.PROD) {
    const locationValue = `${window.location.protocol}//${window.location.host}`;
    return `${locationValue}/api`;
  }
  return "http://localhost:5479";
};

export const API_BACKEND_URL = `${determineApiUrl()}/v1`;

export interface AccessToken {
  accessToken: string;
}
