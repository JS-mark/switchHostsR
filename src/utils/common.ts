import { customAlphabet, urlAlphabet } from 'nanoid'
import { SYSTEM_ENV, type useBridgeCb, useJSBridge } from '@/plugins/Bridge'

import type { User } from '@/store'
import type { Result } from '@/apis/public'

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
