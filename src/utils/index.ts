import { customAlphabet, urlAlphabet } from 'nanoid'
import { SYSTEM_ENV, type useBridgeCb, useJSBridge } from '@/plugins/Bridge'

import type { User } from '@/store'
import type { Result } from '@/apis/public'
import { APP_NAME } from './constant'

export * from './time'
export * from './constant'
export { SYSTEM_ENV, useJSBridge } from '@/plugins/Bridge'
export const { useBridge } = useJSBridge()
export const getRandID = customAlphabet(urlAlphabet, 16)

export const env = window.utools ? SYSTEM_ENV.UTOOLS : SYSTEM_ENV.TAURI

export interface UserInfo extends User {
  ua: string
  home: string
}

/**
 * utool
 */
export type UtoolCB = (
  bridge: useBridgeCb<SYSTEM_ENV.UTOOLS>,
  resolve: (value: any) => void,
  reject: (reason: any) => void,
) => void

/**
 *
 */
export type TauriCB = (
  bridge: useBridgeCb<SYSTEM_ENV.TAURI>,
  resolve: (value: any) => void,
  reject: (reason: any) => void,
) => void

/**
 * 使用 bridge
 * @param utoolCb
 * @param tauriCb
 * @returns Promise
 */
export const useBridgeFunc = <T = any>(utoolCb: UtoolCB, tauriCb: TauriCB): Promise<Result<T>> => {
  return new Promise((resolve, reject) => {
    if (env === SYSTEM_ENV.UTOOLS) {
      useBridge((bridge) => {
        // NOTE: 待实现
        utoolCb(bridge, resolve, reject)
      })
    }
    if (env === SYSTEM_ENV.TAURI) {
      useBridge<SYSTEM_ENV.TAURI>((bridge) => {
        tauriCb(bridge, resolve, reject)
      })
    }
  })
}

/**
 * 设置登录用户信息
 * @param mode
 * @param user
 */
export const setLoginUser = (mode: string, user: any) => {
  window.sessionStorage.setItem(
    APP_NAME,
    window.btoa(
      encodeURIComponent(
        JSON.stringify({
          mode,
          time: +new Date(),
          info: user,
        }),
      ),
    ),
  )
}

/**
 * 获取登录用户信息
 * @returns any
 */
export function getLoginUser() {
  const data = window.sessionStorage.getItem(APP_NAME)
  return data ? JSON.parse(decodeURIComponent(window.atob(data))) : null
}

/**
 * 打开文件
 * @param file
 */
export function openFile(file?: string) {
  if (!file)
    return

  return useBridgeFunc((bridge, resolve, reject) => {
    try {
      bridge.shellShowItemInFolder(file)
      resolve({})
    }
    catch (error) {
      reject(error)
    }
  }, (bridge, resolve, reject) => {
    bridge.invoke('openFile', {
      file,
    }).then(resolve).catch(reject)
  })
}

/**
 * 打开文件夹
 * @param file
 */
export function openDirectory(file?: string) {
  return useBridgeFunc((bridge, resolve, reject) => {
    try {
      const paths = bridge.showOpenDialog({
        title: '选择储存数据文件夹',
        defaultPath: file || bridge.getPath('home'),
        properties: ['openDirectory'],
      })
      resolve(paths && paths[0])
    }
    catch (error) {
      reject(error)
    }
  }, (bridge, resolve, reject) => {
    bridge.invoke('openDirectory', {
      file,
    }).then(resolve).catch(reject)
  })
}

/**
 * 获取类型
 * @param o
 * @returns type
 */
export function getType(o: unknown): string {
  const s: string = Object.prototype.toString.call(o)
  const s_ = s.match(/\[object (.*)\]/) as RegExpMatchArray
  return s_[1].toLowerCase()
}

/**
 * 获取系统信息
 * @returns object { ua: string }
 */
export function getSystemInfo() {
  return useBridgeFunc(() => {
    // NOTE: 暂时实现
  }, (bridge, resolve, reject) => {
    bridge.invoke('get_system_info', {})
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

export const debugUser = () => {
  const data = {
    avatar_url: 'https://avatars.githubusercontent.com/u/6128107?s=80&v=4',
    created_at: '2024-02-18T07:42:49Z',
    email: 'admin@qq.com',
    id: 3,
    is_del: 0,
    is_third: 0,
    name: 'ad',
    status: 0,
    third_account_uid: '-1',
    updated_at: '2024-02-18T07:42:49Z',
    user_level: 0,
  }
  if (!window.__TAURI_IPC__) {
    setLoginUser('email', data)
    return
  }

  return useBridgeFunc(() => {
    // NOTE: 暂时实现
  }, (bridge, resolve, reject) => {
    bridge.invoke('debug_user', { userId: data.id })
      .then((res) => {
        setLoginUser('email', data)
        resolve(res)
      })
      .catch(err => reject(err))
  })
}
