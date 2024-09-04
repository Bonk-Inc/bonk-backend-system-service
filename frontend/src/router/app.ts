import type { RouteRecordRaw } from "vue-router";

export const appRoutes: RouteRecordRaw[] = [
    {
        path: "",
        name: 'app_home',
        component: () => import('../views/app/Home.vue'),
    },
    {
        path: "game/:gameId",
        children: [
            {
                path: "",
                name: "game_home",
                component: () => import('../views/app/Game.vue'),
            },
            {
                path: "levels",
                name: "game_levels",
                component: () => import('../views/app/Levels.vue'),
            },
            {
                path: "scores",
                name: "game_scores",
                component: () => import('../views/app/Scores.vue'),
            },
            {
                path: "settings",
                name: "game_settings",
                component: () => import('../views/app/Settings.vue')
            }
        ]
    }
]