import { defineStore } from 'pinia'
import { isEmpty, set } from 'lodash-es'
import type { SettingSpace } from './useSettings'

export interface Content {
  [key: string]: any
}

export interface ShowOptions {
  settings?: SettingSpace.Settings
  content?: Content
  mode?: Mode
}

export type Mode = 'create' | 'edit'

export const useHostsStore = defineStore('hosts', {
  state: () => ({
    isShowEditor: false,
    mode: 'create',
    content: null,
  }),
  actions: {
    setMode(mode: Mode) {
      if (!mode)
        return
      this.mode = mode
    },
    /**
     * 设置数据
     * @param content
     * @returns { void }
     */
    setContent(content: Content) {
      if (!content)
        return
      set(this, 'content', content)
    },
    /**
     * show
     * @param options
     */
    show(options?: ShowOptions) {
      this.isShowEditor = true
      if (!isEmpty(options)) {
        const { content, mode } = options
        content && isEmpty(content) && this.setContent(content)
        mode && this.setMode(mode)
      }
    },
    hide() {
      this.isShowEditor = false
      this.content = null
    },
  },
})
