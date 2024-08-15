import { authService } from '@/lib/AuthService';
import { createRouter, createWebHistory } from 'vue-router';
import { appRoutes } from './app';

declare module 'vue-router' {
  interface RouteMeta {
    requireAuth: boolean
  }
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: to => {
        return 'authenticate';
      },
      meta: {
        requireAuth: false,
      }
    },
    {
      path: '/authenticate',
      name: 'authenticate',
      component: () => import('../views/Authenticate.vue'),
      meta: {
        requireAuth: false,
      }
    },
    {
      path: '/app',
      name: 'app_home',
      component: () => import('../views/app/AppHome.vue'),
      meta: {
        requireAuth: true,
      },
      children: appRoutes
    }
  ]
});

router.beforeEach(async (to, from, next) => {
  const authenticated = await authService.isUserLoggedIn();
  if (to.meta.requireAuth && !authenticated) {
    next('/authenticate');
  }

  if (to.path === '/login') {
    try {
      await authService.handleLoginRedirect();
      next('/app');  
    } catch(e) {
      next('/authenticate');
    }
  }

  next()
});

export default router;