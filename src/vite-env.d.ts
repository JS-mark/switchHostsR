/// <reference types="vite/client" />
import { MessageApiInjection } from "naive-ui";
declare let window: Window & typeof globalThis;

declare global {
  interface Window {
    utools: any;
    __MonacoEditor: monaco.editor.IStandaloneCodeEditor;
    $useMessage: MessageApiInjection;
  }
}

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<object, object, any>;
  export default component;
}
