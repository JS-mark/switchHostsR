import type { App } from 'vue'
import Svg from '@/components/svg.vue'

export function useGlobalComp(app: App) {
  app.component('SvgIcon', Svg)
}
