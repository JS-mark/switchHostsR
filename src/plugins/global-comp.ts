import { type App, defineAsyncComponent } from 'vue'
import { useMonacoEditor } from '../components/editor/monaco'

function adapterNaiveCss() {
  const meta = document.createElement('meta')
  meta.name = 'naive-ui-style'
  document.head.appendChild(meta)
}

export function getComponentName(key: string) {
  const paths = key.split('/')
  const name = paths
    .filter(it => !!it && it !== '.')
    .reverse()
    .find(
      it =>
        it !== 'index.vue'
        && it !== 'index.ts'
        && it !== 'index.tsx'
        && it !== 'index.js'
        && it !== 'index.jsx',
    )
    ?.replace('.vue', '')
  return name || ''
}

/**
 * 注册
 * @param app
 */
export function registerComponents(app: App) {
  const components = import.meta.glob<any>('../components/**/**.{vue,tsx}', { import: 'default' })
  for (const [_, component] of Object.entries(components)) {
    component().then((res) => {
      // 读取的是组件的 name 属性
      app.component(res.name, defineAsyncComponent(component))
    })
  }
}

export function useGlobalComponents(app: App) {
  adapterNaiveCss()
  registerComponents(app)
  useMonacoEditor()
}
