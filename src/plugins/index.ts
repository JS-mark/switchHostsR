import type { App } from 'vue'

import { useGlobalComponents } from '@/components'

import { useStore } from './pinia'
import { useRouter } from './router'
import { useFeature } from './use-feature'
import { useDirectives } from '../directives'
import { useAppHandler } from './app-handler'

export function usePlugins(app: App) {
  useStore(app)
  useRouter(app)
  useAppHandler(app)
  useDirectives(app)
  useFeature(app)
  useGlobalComponents(app)
}
