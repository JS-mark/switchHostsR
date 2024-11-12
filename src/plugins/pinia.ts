import type { App } from 'vue'

import { createPinia } from 'pinia'
import piniaPersist from 'pinia-plugin-persist'

export const useStoreContext = createPinia()

export function useStore(app: App) {
  useStoreContext.use(piniaPersist)
  app.use(useStoreContext)
}
