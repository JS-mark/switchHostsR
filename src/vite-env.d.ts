/// <reference types="vite/client" />
import type monaco from 'monaco-editor'
import type { MessageApiInjection } from 'naive-ui/lib/message/src/MessageProvider'

declare let window: Window & typeof globalThis
declare module 'monaco-editor/esm/vs/basic-languages/_.contribution';

declare module '*.vue' {
  import type { DefineComponent } from 'vue'

  const component: DefineComponent<object, object, any>
  export default component
}
import type {
  ComponentPublicInstance,
  ComponentRenderProxy,
  ComputedRef,
  FunctionalComponent,
  Ref,
  VNode,
  VNodeChild,
  PropType as VuePropType,
} from 'vue'

declare global {
  /** 构建日期，由 Vite define 注入 */
  const __BUILD_DATE__: string
  const __TAURI_IPC__: any
  const __APP_INFO__: {
    pkg: {
      name: string
      version: string
      dependencies: Recordable<string>
      devDependencies: Recordable<string>
    }
    lastBuildTime: string
  }

  // vue
  declare type PropType<T> = VuePropType<T>
  declare type VueNode = VNodeChild | JSX.Element

  export type Writable<T> = {
    -readonly [P in keyof T]: T[P];
  }

  declare interface InResult<T = any> {
    code: number
    message: string
    result: T
  }

  declare type Nullable<T> = T | null
  declare type NonNullable<T> = T extends null | undefined ? never : T
  declare type Recordable<T = any> = Record<string, T>
  declare interface ReadonlyRecordable<T = any> {
    readonly [key: string]: T
  }
  declare interface Indexable<T = any> {
    [key: string]: T
  }
  declare type DeepPartial<T> = {
    [P in keyof T]?: DeepPartial<T[P]>;
  }
  declare type TimeoutHandle = ReturnType<typeof setTimeout>
  declare type IntervalHandle = ReturnType<typeof setInterval>

  declare interface ChangeEvent extends Event {
    target: HTMLInputElement
  }

  declare interface WheelEvent {
    path?: EventTarget[]
  }

  interface ImportMetaEnv extends ViteEnv {
    __: unknown
  }

  declare interface Window extends globalThis {
    utools: UToolsApi
    __TAURI_IPC__: any
    __TAURI_INTERNALS__: Record<string, unknown>
    __MonacoEditor: Record<string, monaco.editor.IStandaloneCodeEditor> | null
    $useMessage: MessageApiInjection
  }

  declare interface ViteEnv {
    VITE_PORT: number
    VITE_USE_MOCK: boolean
    VITE_PUBLIC_PATH: string
    VITE_GLOB_APP_TITLE: string
    VITE_GLOB_APP_SHORT_NAME: string
    VITE_DROP_CONSOLE: boolean
    VITE_GLOB_PROD_MOCK: boolean
    VITE_GLOB_IMG_URL: string
    VITE_PROXY: string
    VITE_BUILD_COMPRESS: 'gzip' | 'brotli' | 'none'
    VITE_BUILD_COMPRESS_DELETE_ORIGIN_FILE: boolean
  }

  declare function parseInt(s: string | number, radix?: number): number

  declare function parseFloat(string: string | number): number

  namespace JSX {
    // tslint:disable no-empty-interface
    type Element = VNode
    // tslint:disable no-empty-interface
    type ElementClass = ComponentRenderProxy

    interface ElementAttributesProperty {
      $props: any
    }

    interface IntrinsicElements {
      [elem: string]: any
    }

    interface IntrinsicAttributes {
      [elem: string]: any
    }
  }
}

declare module 'vue' {
  export type JSXComponent<Props = any>
    = | { new(): ComponentPublicInstance<Props> }
      | FunctionalComponent<Props>
}

export type DynamicProps<T> = {
  [P in keyof T]: Ref<T[P]> | T[P] | ComputedRef<T[P]>;
}
