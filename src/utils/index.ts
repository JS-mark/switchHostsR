import { APP_NAME } from './constant'
import { customAlphabet, urlAlphabet } from 'nanoid'
import { SYSTEM_ENV, useJSBridge } from '@/plugins/Bridge'

export * from './constant'
export * from './time'
import type { User } from '@/store'

export const getRandID = customAlphabet(urlAlphabet, 16)

export const env = window.utools ? SYSTEM_ENV.UTOOLS : SYSTEM_ENV.TAURI

export const { useBridge } = useJSBridge()

/**
 * 获取系统信息
 * @returns object { ua: string }
 */
export function getSystemInfo() {
  return {
    ua: window.navigator.userAgent,
  }
}

/**
 * 打开文件
 * @param file
 */
export function openFile(file?: string) {
  if (!file)
    return
  if (env === SYSTEM_ENV.UTOOLS) {
    useBridge((bridge) => {
      bridge.shellShowItemInFolder(file)
    })
  }

  if (env === SYSTEM_ENV.TAURI) {
    useBridge<SYSTEM_ENV.TAURI>((bridge) => {
      bridge('openFile', {
        file,
      })
    })
  }
}

/**
 * 打开文件夹
 * @param file
 */
export function openDirectory(file?: string) {
  return new Promise((resolve) => {
    if (env === SYSTEM_ENV.UTOOLS) {
      useBridge((bridge) => {
        const paths = bridge.showOpenDialog({
          title: '选择储存数据文件夹',
          defaultPath: file || bridge.getPath('home'),
          properties: ['openDirectory'],
        })
        resolve(paths && paths[0])
      })
    }

    if (env === SYSTEM_ENV.TAURI) {
      useBridge<SYSTEM_ENV.TAURI>((bridge) => {
        bridge('openDirectory', {
          file,
        }).then((res) => {
          resolve(res)
        })
      })
    }
  })
}

interface UserInfo extends User {
  ua: string
  home: string
}

/**
 * 获取登录用户信息
 * @param name
 * @returns
 */
export function getLoginUser() {
  const data = window.sessionStorage.getItem(APP_NAME)
  return data ? JSON.parse(decodeURIComponent(window.atob(data))) : null
}

/**
 * 获取用户信息
 * @returns Promise
 */
export function getUserInfo(): Promise<UserInfo> {
  return new Promise((resolve) => {
    if (env === SYSTEM_ENV.UTOOLS) {
      useBridge((bridge) => {
        const user = bridge.getUser()
        resolve({
          ua: window.navigator.userAgent,
          home: bridge.getPath('home'),
          ...user,
        })
      })
    }

    if (env === SYSTEM_ENV.TAURI) {
      useBridge<SYSTEM_ENV.TAURI>((bridge) => {
        bridge('getUserInfo').then((res: any) => {
          resolve({
            ua: window.navigator.userAgent,
            home: res.home,
            ...res.user,
          })
        })
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
 * 处理微博用户信息
 * @param data
 * @returns object
 */
export function handerUserInfoByWeibo(data: any) {
  return {
    avatar: data.profile_image_url,
    nickname: data.screen_name,
    messageNUm: data.friends_count,
    home: `https://weibo.com/u/${data.id}`,
    extend: data,
  }
}

/**
 * 处理github用户信息
 * @param data
 * @returns object
 */
export function handerUserInfoByGithub(data: any) {
  return {
    avatar: data.avatar_url,
    nickname: data.name,
    messageNUm: data.followers,
    home: data.html_url,
    extend: data,
  }
}
