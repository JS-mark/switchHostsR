import type { App } from 'vue'

import { useAsyncComp } from '@/utils'

export function install(app: App) {
  app.component('Editor', useAsyncComp(() => import('./editor.vue')))
}
