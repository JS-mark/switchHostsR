import type { App } from 'vue'

import { useAsyncComp } from '@/utils'

export function install(app: App) {
  app.component('GlobalModel', useAsyncComp(() => import('./global-model.vue')))
}
