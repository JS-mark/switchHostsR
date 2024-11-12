import type { User } from '@/store'
import type { Result } from '@/apis/public'

import { customAlphabet, urlAlphabet } from 'nanoid'
import { SYSTEM_ENV, type useBridgeCb, useJSBridge } from '@/utils/bridge'

export * from './time'
export * from './constant'
export { SYSTEM_ENV, useJSBridge } from '@/utils/bridge'
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
export function useBridgeFunc<T = any>(utoolCb: UtoolCB, tauriCb: TauriCB): Promise<Result<T>> {
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
    bridge.core.invoke('get_system_info', {})
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * Sums the passed percentage to the R, G or B of a HEX color
 * @param {string} color The color to change
 * @param {number} amount The amount to change the color by
 * @returns {string} The processed part of the color
 */
function addLight(color: string, amount: number) {
  const cc = parseInt(color, 16) + amount
  const c = cc > 255 ? 255 : cc
  return c.toString(16).length > 1 ? c.toString(16) : `0${c.toString(16)}`
}

/**
 * Lightens a 6 char HEX color according to the passed percentage
 * @param {string} color The color to change
 * @param {number} amount The amount to change the color by
 * @returns {string} The processed color represented as HEX
 */
export function lighten(color: string, amount: number) {
  color = color.includes('#') ? color.substring(1, color.length) : color
  amount = Math.trunc((255 * amount) / 100)
  return `#${addLight(color.substring(0, 2), amount)}${addLight(
    color.substring(2, 4),
    amount,
  )}${addLight(color.substring(4, 6), amount)}`
}

/**
 * 判断是否 url
 */
export function isUrl(url: string) {
  return /^(?:http|https):\/\//.test(url)
}
