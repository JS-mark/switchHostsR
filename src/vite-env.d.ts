/// <reference types="vite/client" />
import type monaco from 'monaco-editor'
import type { MessageApiInjection } from 'naive-ui/lib/message/src/MessageProvider'

declare let window: Window & typeof globalThis
declare module 'monaco-editor/esm/vs/basic-languages/_.contribution';

declare global {
  interface Window {
    utools: UToolsApi
    __MonacoEditor: monaco.editor.IStandaloneCodeEditor | null
    $useMessage: MessageApiInjection
  }
}

declare module '*.vue' {
  import type { DefineComponent } from 'vue'

  const component: DefineComponent<object, object, any>
  export default component
}
