import { MessageApiInjection } from "naive-ui";
declare let window: Window & typeof globalThis;

declare global {
  interface Window {
    utools: any;
    __MonacoEditor: monaco.editor.IStandaloneCodeEditor;
    $useMessage: MessageApiInjection;
  }
}
