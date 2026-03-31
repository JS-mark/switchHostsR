import type { App } from 'vue'

import { createRouter, createWebHashHistory } from 'vue-router'

import { useUserStore } from '@/store'
import { getLoginUser } from '@/utils'

import routes from './routes'

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

router.beforeEach((to) => {
  const store = useUserStore()
  const data = getLoginUser()
  if (data) {
    const now = +new Date()
    if (!(now - data.time >= 7 * 24 * 60 * 3600 * 1000)) {
      store.setLogin(true)
      store.setMode(data.mode)
      store.setUserInfo(data.info)
      return true
    }
  }
  // 未登录时，允许访问登录页，其他页面重定向到登录
  if (!store.isLogin && to.name !== 'Login') {
    return { name: 'Login' }
  }
  return true
})

export function useRouter(app: App, callback?: (app: App) => void) {
  app.use(router)

  if (typeof callback === 'function') {
    router.isReady().finally(() => {
      callback(app)
    })
  }
}
