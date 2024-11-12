import type { App } from 'vue'

import { useAsyncComp } from '@/utils'

export function install(app: App) {
  app.component('AddHosts', useAsyncComp(() => import('./add-hosts.vue')))
}
