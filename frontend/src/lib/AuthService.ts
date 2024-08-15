import { UserManager, type UserManagerSettings, type UserProfile } from "oidc-client-ts";

export class AuthService {
    #userManager: UserManager;

    constructor() {
        const setting: UserManagerSettings = {
            authority: import.meta.env.VITE_APP_AUTH_URL,
            client_id: import.meta.env.VITE_APP_AUTH_CLIENT_ID,
            redirect_uri: import.meta.env.VITE_APP_AUTH_REDIRECT_URL,
            response_type: 'code',
            automaticSilentRenew: true,
        };

        this.#userManager = new UserManager(setting);
    }

    async login() {
        await this.#userManager.signinRedirect();
    }

    async logout() {
        await this.#userManager.signoutRedirect();
    }

    handleLoginRedirect() {
        return this.#userManager.signinRedirectCallback()
    }

    /**
     * Checks whether or not a user is currently logged in.
     *
     * Returns a promise which will be resolved to true/false or be rejected with an error.
     */
    async isUserLoggedIn(): Promise<boolean> {
        const user = await this.#userManager.getUser();
        return user !== null;
    }

    /**
     * Get the profile data for the currently authenticated user.
     *
     * Returns an empty object if no user is logged in.
     */
    async getProfile(): Promise<UserProfile | undefined> {
        const user = await this.#userManager.getUser();
        return user?.profile;
    }

    /**
     * Get the access token.
     *
     * Can be used to make requests to the backend.
     */
    async getAccessToken(): Promise<string | undefined> {
        const user = await this.#userManager.getUser();
        return user?.access_token;
    }
}

export const authService = new AuthService();