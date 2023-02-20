/// <reference types="vite/client" />

declare let window: Window & typeof globalThis

declare global {
  interface Window {
    __MonacoEditor: monaco.editor.IStandaloneCodeEditor
  }
}

declare module "*.vue" {
  import type { DefineComponent } from "vue"
  const component: DefineComponent<{}, {}, any>
  export default component
}
