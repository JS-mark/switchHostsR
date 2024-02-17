import piniaPersist from 'pinia-plugin-persist'
import type { App } from 'vue'
import { createPinia } from 'pinia'

export function usePinia(app: App) {
  const pinia = createPinia()
  pinia.use(piniaPersist)

  app.use(pinia)
}
