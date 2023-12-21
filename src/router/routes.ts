import type { RouteRecordRaw } from 'vue-router'
import { Albums, Home } from '@vicons/ionicons5'
import { genSidersMenus, generateRoutes, processRoutes } from './utils'

export * from './utils'
export const routes: RouteRecordRaw[] = [
  {
    path: '/home',
    name: 'Home',
    meta: {
      title: '首页',
      icon: Home,
      hidden: false,
    },
    component: () => import('@/pages/home/welcome.vue'),
  },
  {
    path: '/login',
    name: 'Login',
    meta: {
      title: '登录',
      hidden: true,
    },
    component: () => import('@/pages/login/index.vue'),
  },
  {
    path: '/about',
    name: 'About',
    meta: {
      title: '关于',
      icon: Albums,
      hidden: false,
    },
    component: () => import('@/pages/about/index.vue'),
  },
  {
    path: '/error',
    name: 'Error',
    meta: {
      title: '异常页面',
      icon: 'local_fire_department',
      hidden: true,
    },
    children: [
      {
        path: '/404',
        meta: {
          title: '404 - NotFound',
          hidden: true,
        },
        redirect: '/',
      },
    ],
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    meta: {
      hidden: true,
    },
    component: () => import('@/pages/404/404.vue'),
  },
]

export const defaultRoute = 'Home'

export const menus = genSidersMenus(routes)

const routeList = processRoutes(routes).filter(
  item => !!item.path || !!item.children?.length || !!item.redirect,
)

export default generateRoutes(routeList)
