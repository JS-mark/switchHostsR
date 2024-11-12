// import api from '@/plugins/request'
import { Octokit } from '@octokit/core'
import { useBridgeFunc } from '@/utils'

import type { GetPage } from './public'

export function getUserInfoByGithub(name: string) {
  const octokit = new Octokit({
    // 个人 token
    auth: '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',
  })

  return octokit.request('GET /users/{username}', {
    username: name,
    headers: {
      'X-GitHub-Api-Version': '2022-11-28',
    },
  }).then((res) => {
    if (res.status === 200) {
      if (!res.data.email)
        return Promise.reject(res)
      else return Promise.resolve(res.data)
    }
    else { return Promise.reject(res) }
  }).catch((err) => {
    return Promise.reject(err)
  })
}

export const getUser = {
  github: getUserInfoByGithub,
}

/**
 * 用户登录
 * @param options
 * @param options.email
 * @param options.password
 * @returns Promise
 */
export function loginPlatform(options: { email: string, password: string }) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('user_login', options)
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 用户退出登录
 * @returns Promise
 */
export function logoutPlatform() {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('logout', {})
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}
/**
 * 用户退出登录
 * @returns Promise
 */
export function getAllUsers(options: GetPage) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_all_users', options)
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 创建用户
 * @param options
 * @param options.name
 * @param options.email
 * @param options.password
 * @returns Promise
 */
export function addUser(options: {
  name: string
  email: string
  password: string
}) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('add_user', options)
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 三方登录
 * @param options
 * @param options.name
 * @param options.email
 * @param options.uid
 * @param options.account
 * @param options.avatarUrl
 * @param options.password
 * @param options.createdAt
 * @param options.updatedAt
 * @returns Promise
 */
export function thirdAccountLogin(options: {
  email: string
  name: string
  uid: string
  account: string
  avatarUrl: string
  password: string
  createdAt: string
  updatedAt: string
}) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('third_account_login', options)
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}
