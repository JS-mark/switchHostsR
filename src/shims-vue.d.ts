import type monaco from 'monaco-editor'
import type { MessageApiInjection } from 'naive-ui/lib/message/src/MessageProvider'

declare let window: Window & typeof globalThis

declare global {
  interface Window {
    utools: UToolsApi
    __MonacoEditor: monaco.editor.IStandaloneCodeEditor | null
    $useMessage: MessageApiInjection
  }
}
