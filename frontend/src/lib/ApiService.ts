import { authService, type AuthService } from './AuthService';

export interface ResponseBody<T> {
    message: string,
    data: T
}

export class ApiService {
    #baseUrl: string;
    #authService: AuthService;

    constructor(baseUrl: string) {
        this.#baseUrl = baseUrl;
        this.#authService = authService;
    }

    async get<T>(path: string): Promise<ResponseBody<T>> {
        const accessToken = await this.#authService.getAccessToken();
        const response = await fetch(`${this.#baseUrl}/${path}`, {
            headers: { 'Authorization': `Bearer ${accessToken}`}
        });

        const data = await response.json() as ResponseBody<T>;
        if (!response.ok) {
            throw new Error(data.message)
        }

        return data;
    }

    async post<T>(path: string, body: string): Promise<ResponseBody<T>> {
        const accessToken = await this.#authService.getAccessToken();
        const response = await fetch(`${this.#baseUrl}/${path}`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${accessToken}`,
                'Content-Type': 'application/json'
            },
            body
        });

        const data = await response.json() as ResponseBody<T>;
        if (!response.ok) {
            throw new Error(data.message)
        }

        return data;
    }

    async put<T>(path: string, body: string): Promise<ResponseBody<T>> {
        const accessToken = await this.#authService.getAccessToken();
        const response = await fetch(`${this.#baseUrl}/${path}`, {
            method: 'PUT',
            headers: {
                'Authorization': `Bearer ${accessToken}`,
                'Content-Type': 'application/json'
            },
            body
        });

        const data = await response.json() as ResponseBody<T>;
        if (!response.ok) {
            throw new Error(data.message)
        }

        return data;
    }

    async delete<T>(path: string): Promise<ResponseBody<T>> {
        const accessToken = await this.#authService.getAccessToken();
        const response = await fetch(`${this.#baseUrl}/${path}`, {
            headers: { 'Authorization': `Bearer ${accessToken}`}
        });

        const data = await response.json() as ResponseBody<T>;
        if (!response.ok) {
            throw new Error(data.message)
        }

        return data;
    }
}

export const apiService = new ApiService(import.meta.env.VITE_APP_API_URL);