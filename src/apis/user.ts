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
 * @param options.username 用户名
 * @param options.password 密码
 * @param options.remember_me 记住我
 * @returns Promise
 */
export function loginPlatform(options: { username: string, password: string, remember_me?: boolean }) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('user_login', {
      username: options.username,
      password: options.password,
      remember_me: options.remember_me
    })
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
    bridge.core.invoke('logout')
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 刷新令牌
 * @param refreshToken 刷新令牌
 * @returns Promise
 */
export function refreshToken(refreshToken: string) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('refresh_token', {
      refresh_token: refreshToken
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 修改密码
 * @param options
 * @param options.old_password 旧密码
 * @param options.new_password 新密码
 * @returns Promise
 */
export function changePassword(options: { old_password: string, new_password: string }) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('change_password', {
      old_password: options.old_password,
      new_password: options.new_password
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 验证令牌
 * @param token 访问令牌
 * @returns Promise
 */
export function verifyToken(token: string) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('verify_token', {
      token
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 获取当前用户信息
 * @returns Promise
 */
export function getCurrentUserInfo() {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_current_user_info')
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 检查用户名可用性
 * @param username 用户名
 * @returns Promise
 */
export function checkUsernameAvailability(username: string) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('check_username_availability', {
      username
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 检查邮箱可用性
 * @param email 邮箱
 * @returns Promise
 */
export function checkEmailAvailability(email: string) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('check_email_availability', {
      email
    })
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
 * @param options.username
 * @param options.email
 * @param options.password
 * @param options.avatar
 * @param options.role
 * @returns Promise
 */
export function addUser(options: {
  email: string
  password: string
  username?: string
  avatar?: string
  role?: string
}) {
  return useBridgeFunc(() => {
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('create_user', {
      request: {
        ...options,
        avatar: options.avatar || "https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg",
        role: options.role || 'user'
      }
    })
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
    bridge.core.invoke('third_account_login', {
      request: {
        ...options
      }
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}
