import { defineStore } from 'pinia'
import { get, isEmpty, set } from 'lodash-es'
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

export interface Hosts {
  id: string
  content: string
}

export interface LocalHosts extends Hosts {
  title: string
}
export interface RemoteHosts extends Hosts {
  title: string
  config: {
    remoteUrl: string
    refreshType: string
  }
}

export interface HostsList {
  id: string
  name: string
}

export const useHostsStore = defineStore('hosts', {
  state: (): {
    isShowEditor: boolean
    mode: Mode
    content: Content | null
    hostsData: { [key: string]: LocalHosts | RemoteHosts }
    localHosts: HostsList[]
    remoteHosts: HostsList[]
  } => ({
    isShowEditor: false,
    mode: 'create',
    content: null,
    hostsData: {},
    localHosts: [],
    remoteHosts: [],
  }),
  actions: {
    setMode(mode: Mode) {
      if (!mode)
        return
      this.mode = mode
    },
    /**
     * 更新数据
     * @param data
     * @returns { void }
     */
    updateHosts(data: LocalHosts | RemoteHosts): void {
      if (get(this.hostsData, data.id))
        return
      this.hostsData[data.id] = data
    },
    /**
     * 添加
     * @param type
     * @param hosts
     * @returns { void }
     */
    addGolblHosts(type: 'local' | 'remote', hosts: LocalHosts) {
      if (get(this.hostsData, hosts.id))
        return

      if (type === 'local') {
        this.localHosts.push({
          id: hosts.id,
          name: hosts.title,
        })
      }
      if (type === 'remote') {
        this.remoteHosts.push({
          id: hosts.id,
          name: hosts.title,
        })
      }
      this.hostsData[hosts.id] = hosts
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
  persist: {
    enabled: true, // true 表示开启持久化保存
  },
})
