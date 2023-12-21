import type { App } from 'vue'
import { type Feature, useFeatureStore } from '../store/featureStore'

export function useFeature(app: App, options?: { list?: Feature[] }) {
  console.warn(`当前版本: ${app.version}`)

  useFeatureStore().init(options?.list || [
    { key: 'export', enable: false },
    { key: 'local-import', enable: false },
    { key: 'import-remote', enable: false },
    { key: 'settings', enable: true },
    { key: 'outdated', enable: true },
    { key: 'recycle', enable: false },
  ])
}
