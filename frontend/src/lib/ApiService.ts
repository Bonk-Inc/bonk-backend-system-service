import type { Router } from 'vue-router';
import { authService, type AuthService } from './AuthService';

export interface ResponseBody<T> {
    message: string,
    data: T
}

export class ApiService {
    #baseUrl: string;
    #authService: AuthService;
    #router: Router;

    constructor(baseUrl: string, router: Router) {
        this.#baseUrl = baseUrl;
        this.#authService = authService;
        this.#router = router;
    }

    async get<T>(path: string): Promise<ResponseBody<T>> {
        const accessToken = await this.#authService.getAccessToken();
        const response = await fetch(`${this.#baseUrl}/${path}`, {
            headers: { 'Authorization': `Bearer ${accessToken}`}
        });

        const data = await response.json() as ResponseBody<T>;
        if (!response.ok) {
            if(response.status === 401) {
                this.#authService.logout();
            }

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
            if(response.status === 401) {
                this.#authService.logout();
            }

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
            if(response.status === 401) {
                this.#authService.logout();
            }

            throw new Error(data.message)
        }

        return data;
    }

    async delete(path: string): Promise<void> {
        const accessToken = await this.#authService.getAccessToken();
        const response = await fetch(`${this.#baseUrl}/${path}`, {
            method: 'DELETE',
            headers: { 'Authorization': `Bearer ${accessToken}`}
        });

        if (!response.ok) {
            if(response.status === 401) {
                this.#authService.logout();
            }
            
            const data = await response.json() as ResponseBody<void>;
            throw new Error(data.message)
        }
    }
}

export const apiService = (router: Router) => new ApiService(import.meta.env.VITE_APP_API_URL, router);