import { h } from 'vue'
import type { RouteRecordRaw } from 'vue-router'
import { type MenuOption } from 'naive-ui'
import { Home } from '@vicons/ionicons5'

export const processRoutes = (routes: RouteRecordRaw[], parentPath = '') => {
  return routes.map((route) => {
    const _route = Object.assign({}, route)
    const { children, path, meta = {} } = _route
    _route.path = parentPath + path
    _route.meta = meta
    if (children && children.length) {
      _route.children = processRoutes(children, _route.path)
    } else {
      delete _route.children
    }
    return _route
  })
}

export const genSidersMenus = (routes: RouteRecordRaw[]): MenuOption[] => {
  return routes.map((route) => {
    const menu = {
      label: route.name,
      key: route.name,
      path: route.path,
      disabled: route.meta?.hidden,
      icon: () => h((route.meta?.icon as typeof Home) || Home),
      show: !route.meta?.hidden,
    } as MenuOption

    if (route.children && route.children.length > 0) {
      menu.children = genSidersMenus(route.children)
    }

    return menu
  })
}

export const generateRoutes = (routes: RouteRecordRaw[]): RouteRecordRaw[] => {
  const _routes: RouteRecordRaw[] = []
  routes.forEach((route) => {
    const { children, component, redirect } = route

    if (children) {
      _routes.push(...generateRoutes(children))
    } else if (component) {
      const _route = Object.assign({}, route)
      delete _route.children
      _routes.push(_route)
    } else if (redirect) {
      _routes.push(route)
    } else {
      // 无效路由
      if (!route.meta?.link) {
        console.log('该路由无效：', route)
      }
    }
  })

  return _routes
}
