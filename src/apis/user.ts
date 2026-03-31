// import api from '@/plugins/request'
import { Octokit } from '@octokit/core'

import { useBridgeFunc } from '@/utils'

/**
 * 通过 GitHub API 获取用户信息
 * @param name GitHub 用户名
 * @param token 可选的 GitHub Personal Access Token（从环境变量或用户设置获取）
 */
export function getUserInfoByGithub(name: string, token?: string) {
  const octokit = new Octokit({
    // 从调用方传入或使用环境变量，不再硬编码
    auth: token || import.meta.env.VITE_GITHUB_TOKEN || undefined,
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
      remember_me: options.remember_me,
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
      refresh_token: refreshToken,
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
      new_password: options.new_password,
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
      token,
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
      username,
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
      email,
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}
/**
 * 重置密码（忘记密码）
 * @param options
 * @param options.email 注册邮箱
 * @param options.new_password 新密码
 * @returns Promise
 */
export function resetPassword(options: { email: string, new_password: string }) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('reset_password', {
      email: options.email,
      new_password: options.new_password,
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 获取所有用户列表
 * @returns Promise
 */
export function getAllUsers() {
  return useBridgeFunc(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_users')
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 更新用户信息（管理员操作）
 * @param userId 用户 ID
 * @param request 更新参数
 * @returns Promise
 */
export function updateUserInfo(userId: number, request: {
  username?: string
  email?: string
  avatar?: string
  role?: string
  is_active?: boolean
}) {
  return useBridgeFunc(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('update_user', {
      user_id: userId,
      request,
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 删除用户（管理员操作）
 * @param userId 用户 ID
 * @returns Promise
 */
export function deleteUserById(userId: number) {
  return useBridgeFunc(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('delete_user', {
      user_id: userId,
    })
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
    bridge.core.invoke('register_user', {
      request: {
        ...options,
        avatar: options.avatar || 'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
        role: options.role || 'user',
      },
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
        ...options,
      },
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}
