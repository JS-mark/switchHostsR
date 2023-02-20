import { RouteRecordRaw } from 'vue-router'
import { Home, Albums } from '@vicons/ionicons5'
import { generateRoutes, processRoutes, genSidersMenus } from './utils'

export * from './utils'
export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    meta: {
      title: '首页',
      icon: Home,
      hidden: false,
    },
    component: () => import('@/pages/home/index.vue'),
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
  (item) => !!item.path || !!item.children?.length || !!item.redirect
)

export default generateRoutes(routeList)
