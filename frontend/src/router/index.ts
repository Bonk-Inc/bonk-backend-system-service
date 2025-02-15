import { authService } from '@/lib/AuthService';
import { createRouter, createWebHashHistory } from 'vue-router';
import { appRoutes } from './app';

declare module 'vue-router' {
  interface RouteMeta {
    requireAuth: boolean
  }
}

const router = createRouter({
  history: createWebHashHistory(),
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
      children: appRoutes,
      meta: {
        requireAuth: true,
      }
    }
  ]
});

router.beforeEach(async (to, _, next) => {
  const authenticated = await authService.isUserLoggedIn();

  if (to.meta.requireAuth && !authenticated) {
    next({ name: 'authenticate', query: {} });
  } else if (to.path === '/login') {
    try {
      await authService.handleLoginRedirect();
      next({ name: 'app_home', query: {} });
    } catch(e) {
      next({ name: 'authenticate', query: {} });
    }
  } else {
    next();
  }
});

export default router;