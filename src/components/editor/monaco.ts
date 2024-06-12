import 'monaco-editor/esm/vs/editor/editor.all.js'
import 'monaco-editor/esm/vs/language/json/monaco.contribution'
import 'monaco-editor/esm/vs/basic-languages/monaco.contribution'
import 'monaco-editor/esm/vs/basic-languages/shell/shell.contribution'
import * as monaco from 'monaco-editor'
import { useDark } from '@vueuse/core'
import emitter from '@/plugins/emitter'
import './languages/index'

export { monaco }

export function useMonacoEditor() {
  self.MonacoEnvironment = {
    getWorker(workerId, label) {
      const getWorkerModule = (moduleUrl: string, label: string) => {
        // @ts-expect-error
        return new Worker(self.MonacoEnvironment?.getWorkerUrl(moduleUrl as string, label as string), {
          name: label,
          type: 'module',
        })
      }

      switch (label) {
        case 'json':
          return getWorkerModule('/monaco-editor/esm/vs/language/json/json.worker?worker', label)
        case 'css':
        case 'scss':
        case 'less':
          return getWorkerModule('/monaco-editor/esm/vs/language/css/css.worker?worker', label)
        case 'html':
        case 'handlebars':
        case 'razor':
          return getWorkerModule('/monaco-editor/esm/vs/language/html/html.worker?worker', label)
        case 'typescript':
        case 'javascript':
          return getWorkerModule('/monaco-editor/esm/vs/language/typescript/ts.worker?worker', label)
        default:
          return getWorkerModule('/monaco-editor/esm/vs/editor/editor.worker?worker', label)
      }
    },
  }
}

/**
 * hook 函数
 * @param language
 * @returns { void }
 */
export default function useMonaco(language = 'shell') {
  const isDark = useDark({
    selector: 'body',
    attribute: 'color-scheme',
    valueDark: 'vs-dark',
    valueLight: 'vs',
  })
  let initReadOnly = false

  const useEditor = (
    id: string,
    cb: (editor: monaco.editor.IStandaloneCodeEditor) => void,
  ) => {
    if (window.__MonacoEditor) {
      cb(window.__MonacoEditor[id] as monaco.editor.IStandaloneCodeEditor)
      return
    }
    console.warn('editor not ready! 通过 cb 执行！')

    emitter.on('ready', (editor: monaco.editor.IStandaloneCodeEditor) => {
      cb(editor)
      emitter.off('ready')
    })
  }

  const updateVal = async (id: string, val: string, format = true) => {
    useEditor(id, (editor) => {
      editor?.setValue(val)
    })
    setTimeout(() => {
      useEditor(id, async (editor) => {
        editor?.updateOptions({ readOnly: initReadOnly })
        format
          && (await editor?.getAction('editor.action.formatDocument')?.run())
      })
    }, 100)
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

  const switchTheme = (id: string, theme: 'dark' | 'light' | 'auto') => {
    const theme_ = getTheme(theme, language)
    useEditor(id, (editor) => {
      editor.updateOptions({
        theme: theme_,
      })
    })
  }

  const createEditor = (
    id: string,
    el: HTMLElement | null,
    editorOption: monaco.editor.IStandaloneEditorConstructionOptions = {},
  ) => {
    if (window.__MonacoEditor && window.__MonacoEditor[id])
      return
    if (!window.__MonacoEditor) {
      window.__MonacoEditor = {}
    }
    initReadOnly = !!editorOption.readOnly
    const theme = getTheme('auto', language)

    window.__MonacoEditor[id]
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
      }) as unknown as monaco.editor.IStandaloneCodeEditor)
    // 编辑器 ready
    emitter.emit('ready', window.__MonacoEditor)
    return window.__MonacoEditor
  }

  const destroy = (id: string) => {
    useEditor(id, (editor) => {
      editor.dispose()
      window.__MonacoEditor[id] = null
    })
  }

  const onFormatDoc = (id: string) => {
    useEditor(id, (editor) => {
      editor?.getAction('editor.action.formatDocument')?.run()
    })
  }

  return {
    updateVal,
    useEditor,
    destroy,
    switchTheme,
    createEditor,
    onFormatDoc,
  }
}
