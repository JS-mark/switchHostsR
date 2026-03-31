import type { editor } from 'monaco-editor'

import { has, unset } from 'lodash-es'

import { useEmitter } from '@/utils'

const EDITOR_KEY = '__MonacoEditor'
const emitter = useEmitter()
type EditorType = editor.IStandaloneCodeEditor
type UseEditorCB = (editor: EditorType) => void

/**
 * 更新数据
 * @param id
 * @param editor
 */
export function setEditor(id: string, editor: EditorType) {
  if (!window[EDITOR_KEY]) {
    window[EDITOR_KEY] = {}
  }

  // 已经存在 不得再继续注册
  if (has(window[EDITOR_KEY], id)) {
    throw new Error('editor already exists')
  }

  window[EDITOR_KEY][id] = editor
  // 通知
  emitter.emit('editor-init', { editor, id })
}

/**
 * 销毁editor
 */
export function clearEditor(id: string) {
  if (!window[EDITOR_KEY]) {
    return
  }
  if (Object.keys(window[EDITOR_KEY]).length === 1) {
    window[EDITOR_KEY] = null
  }
  else {
    unset(window[EDITOR_KEY], id)
  }
}

/**
 * 使用editor
 * @returns void | Promise<Editor>
 */
export function useEditor(id: string): Promise<EditorType>
export function useEditor(id: string, cb: UseEditorCB): void
export function useEditor(id: string, cb?: UseEditorCB): void | Promise<EditorType> {
  const useEditorCB = (cb: (editor: EditorType) => void) => {
    if (window[EDITOR_KEY] && window[EDITOR_KEY][id]) {
      return cb(window[EDITOR_KEY][id])
    }

    const onEditorInit = (options: { editor: EditorType, id: string }) => {
      const { editor } = options
      cb && cb(editor)
      // NOTE: 移除监听
      emitter.off('editor-init', onEditorInit)
    }
    emitter.on('editor-init', onEditorInit)
  }
  if (!cb) {
    return new Promise((resolve) => {
      useEditorCB(resolve)
    })
  }
  // 直接调用 cb
  useEditorCB(cb)
}
