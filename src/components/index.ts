import type { App } from 'vue'

export function useGlobalComponents(app: App) {
  const components = import.meta.glob<any>('./**/index.ts', { import: 'install' })
  for (const [_, component] of Object.entries(components)) {
    component().then((install) => {
      install && install(app)
    })
  }
}
