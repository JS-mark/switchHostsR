import type { Monaco } from '@monaco-editor/loader'
import type { editor } from 'monaco-editor/esm/vs/editor/editor.api'

import { nanoid } from 'nanoid'
import { delay } from 'lodash-es'
import { useDark } from '@vueuse/core'
import loader from '@monaco-editor/loader'
import { readText, writeText } from '@tauri-apps/plugin-clipboard-manager'

import { registerHostsLanguage } from './languages'
import { clearEditor, setEditor, useEditor } from './hook'

// Tauri WebView 中 navigator.clipboard API 受限，
// 用 Tauri 剪贴板插件 polyfill 以确保 Monaco 编辑器复制粘贴正常工作
if (window.__TAURI_INTERNALS__) {
  const clipboardPolyfill = {
    writeText: (text: string) => writeText(text),
    readText: () => readText(),
    write: navigator.clipboard?.write?.bind(navigator.clipboard),
    read: navigator.clipboard?.read?.bind(navigator.clipboard),
    addEventListener: navigator.clipboard?.addEventListener?.bind(navigator.clipboard),
    removeEventListener: navigator.clipboard?.removeEventListener?.bind(navigator.clipboard),
    dispatchEvent: navigator.clipboard?.dispatchEvent?.bind(navigator.clipboard),
  }
  Object.defineProperty(navigator, 'clipboard', {
    value: clipboardPolyfill,
    writable: false,
    configurable: true,
  })
}

export interface CancelablePromise<T> extends Promise<T> {
  cancel: () => void
}

/**
 * hook 函数
 * @param language
 * @returns { void }
 */
export default function useMonaco(language = 'shell') {
  const id = nanoid()
  const isDark = useDark({
    selector: 'body',
    attribute: 'color-scheme',
    valueDark: 'vs-dark',
    valueLight: 'vs',
  })
  let initReadOnly = false

  const onFormatDoc = () => {
    useEditor(id, async (editor) => {
      editor?.updateOptions({ readOnly: true })
      delay(async () => {
        await editor?.getAction('editor.action.formatDocument')?.run()
      }, 100)
      editor?.updateOptions({ readOnly: initReadOnly })
    })
  }

  const updateVal = async (val: string, format = true) => {
    useEditor(id, (editor) => {
      editor?.setValue(val)

      format && setTimeout(onFormatDoc, 100)
    })
  }

  const getTheme = (theme: 'dark' | 'light' | 'auto', language: string) => {
    const lang = language
    switch (theme) {
      case 'auto':
        if (lang === 'hosts') {
          return isDark.value ? 'hosts-dark' : 'hosts'
        }
        else {
          return isDark.value ? 'vs-dark' : 'vs'
        }
      case 'light':
        if (lang === 'hosts') {
          return 'hosts'
        }
        else {
          return 'vs-dark'
        }
      case 'dark':
        if (lang === 'hosts') {
          return 'hosts-dark'
        }
        else {
          return 'vs-dark'
        }
    }
  }

  const switchTheme = (theme: 'dark' | 'light' | 'auto') => {
    const theme_ = getTheme(theme, language)
    useEditor(id, (editor) => {
      editor.updateOptions({
        theme: theme_,
      })
    })
  }

  const createEditor = (
    el: HTMLElement | null,
    editorOption: editor.IStandaloneEditorConstructionOptions = {},
    onReady: (editor: editor.IStandaloneCodeEditor) => void,
    onInit?: (event: CancelablePromise<Monaco>) => void,
  ): Promise<editor.IStandaloneCodeEditor | void> => {
    loader.config({
      'paths': {
        vs: '/vs',
      },
      'vs/nls': {
        availableLanguages: {
          '*': 'zh-cn',
        },
      },
    })
    const monacoCtrl = loader.init()
    onInit && onInit(monacoCtrl)
    return new Promise((resolve, reject) => {
      monacoCtrl.then((monaco) => {
        // 在 loader 加载的 Monaco 实例上注册 hosts 语言和主题
        registerHostsLanguage(monaco)

        initReadOnly = !!editorOption.readOnly
        const theme = getTheme('dark', language)

        const __MonacoEditor
          = el
            && (monaco.editor.create(el, {
              value: '',
              language,
              foldingStrategy: 'indentation', // 代码可分小段折叠
              overviewRulerBorder: false, // 不要滚动条的边框
              minimap: { enabled: false },
              theme,
              lineNumbers: 'on',
              multiCursorModifier: 'ctrlCmd',
              scrollbar: {
                verticalScrollbarSize: 8,
                horizontalScrollbarSize: 8,
              },
              tabSize: 2,
              automaticLayout: true, // 自适应宽高
              ...editorOption,
            }) as unknown as editor.IStandaloneCodeEditor)
        // 编辑器 ready

        if (!__MonacoEditor)
          return
        setEditor(id, __MonacoEditor)
        onReady(__MonacoEditor)
        resolve(__MonacoEditor)
      }).catch(err => reject(err))
    })
  }

  const destroy = (id: string) => {
    useEditor(id, (editor) => {
      editor?.dispose()
      clearEditor(id)
    })
  }

  return {
    id,
    updateVal,
    destroy,
    switchTheme,
    createEditor,
    onFormatDoc,
  }
}
