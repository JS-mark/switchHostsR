import routes from './routes'
import { createRouter, createWebHashHistory } from 'vue-router'

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})


import type { App } from 'vue'

export const useRouter = (app: App, callback?: (app: App) => void) => {
  app.use(router)

  if (callback instanceof Function) {
    router.isReady().finally(() => {
      callback(app)
    })
  }
}
