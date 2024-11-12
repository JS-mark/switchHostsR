import type { App } from 'vue'

import { Copy } from './copy'
import { Auth } from './v-auth'
import { Debounce } from './debounce'
import { Throttle } from './throttle'
import { Draggable } from './draggable'
import { Longpress } from './longpress'
import { ClickOutside } from './clickOutside'

export function useDirectives(app: App) {
  app.directive('auth', Auth)
  app.directive('copy', Copy)
  app.directive('draggable', Draggable)
  app.directive('debounce', Debounce)
  app.directive('throttle', Throttle)
  app.directive('longpress', Longpress)
  app.directive('click-outside', ClickOutside)
}
