import { defineStore } from 'pinia'
import { isEmpty, set } from 'lodash-es'
import { getType } from '@/utils/index'
import { APP_NAME } from '@/utils/constant'

export namespace SettingSpace {
  export interface Settings {
    system?: System
    user?: User
    general?: General
    proxy?: Proxy
    cmd?: Cmd
    advanced?: Advanced
  }

  export interface System {
    [key: string]: any
    defaultSeetingsHome: string
  }

  export interface User {
    ua: string
    home: string
  }
  export interface General {
    theme: 'black' | 'light' | 'auto'
    language: string
    writeMode: '1' | '2' // 追加 1，覆盖 2
    selectedMode: '1' | '2' // 单选 1，多选 2
    palletTitle: boolean // 托盘标题
    hideAtStartup: boolean // 托盘标题
  }

  export interface Proxy {
    useProxy: boolean
    origin: 'http' | 'https'
    host: string
    port: string
  }

  export interface Cmd {
    cmd: string
  }

  export interface Advanced {
    canSendData?: boolean
    hostsPath?: string
    SwitchHostsRPath?: string
  }
}

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    isShowSettings: false,
    settings: {
      system: {
        defaultSeetingsHome: `.${APP_NAME}`,
      },
      user: {
        ua: window.navigator.userAgent,
        home: '',
        info: {
          avatar: '',
          nickname: '',
          type: '',
        },
      },
      // 通用
      general: {
        theme: 'auto' as const,
        language: 'zh-cn' as const,
        writeMode: '1' as const, // 追加 1，覆盖 2
        selectedMode: '2' as const, // 单选 1，多选 2
        palletTitle: false, // 托盘标题
        hideAtStartup: false, // 托盘标题
      },
      // 代理
      proxy: {
        useProxy: false,
        origin: 'http' as const,
        host: '',
        port: '',
      },
      // 命令
      cmd: {
        cmd: '',
      },
      // 高级设置
      advanced: {
        canSendData: true,
        hostsPath: '',
        SwitchHostsRPath: '',
      },
    },
  }),
  getters: {
    canSendData(): boolean {
      return this.settings.advanced.canSendData
    },
    user(): SettingSpace.User {
      return this.settings.user
    },
    proxy(): SettingSpace.Proxy {
      return this.settings.proxy
    },
    cmd(): SettingSpace.Cmd {
      return this.settings.cmd
    },
    general(): SettingSpace.General {
      return this.settings.general
    },
    advanced(): SettingSpace.Advanced {
      return this.settings.advanced
    },
  },
  actions: {
    getSettings() {
      return this.settings
    },
    /**
     * 设置数据
     * @param data
     */
    setSettingsByData(data: SettingSpace.Settings, overlay = true) {
      const deepSet = (originData: any, data: any) => {
        if (!isEmpty(data)) {
          for (const [key, value] of Object.entries(data)) {
            if (getType(value) !== 'object' || overlay)
              set(originData, key, value)

            else
              deepSet(originData[key], value)
          }
        }
      }
      deepSet(this.settings, data)
    },
    setCanSendData(flag: boolean) {
      this.settings.advanced.canSendData = flag
    },
    /**
     * show
     * @param options
     */
    show() {
      this.isShowSettings = true
    },
    hide() {
      this.isShowSettings = false
    },
  },
})
