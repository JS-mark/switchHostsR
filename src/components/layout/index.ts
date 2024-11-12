import type { App } from 'vue'

import { useAsyncComp } from '@/utils'

export function install(app: App) {
  app.component('LHeader', useAsyncComp(() => import('./l-header.vue')))
  app.component('LSider', useAsyncComp(() => import('./l-sider.vue')))
}
