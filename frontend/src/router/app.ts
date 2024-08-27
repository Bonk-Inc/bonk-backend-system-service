import type { RouteRecordRaw } from "vue-router";

export const appRoutes: RouteRecordRaw[] = [
    {
        path: "",
        name: 'app_home',
        component: () => import('../views/app/Home.vue'),
    },
    {
        path: "game/:gameId",
        name: "game",
        component: () => import('../views/app/Game.vue'),
    }
]