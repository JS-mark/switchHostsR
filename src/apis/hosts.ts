import { useBridgeFunc } from '@/utils'

import type { ListResult, Result } from './public'

/** 后端 Host 模型对应的前端接口 */
export interface Hosts {
  id: number
  user_id: number
  name: string
  description: string | null
  content: string
  is_active: number // 0: 未激活, 1: 已激活 (对应后端 i32)
  is_system: number // 0: 用户创建, 1: 系统 (对应后端 i32)
  created_at: number
  updated_at: number
}

/** 创建 Host 请求参数 */
export interface CreateHostRequest {
  name: string
  content: string
  description?: string
  is_active?: boolean
  group_id?: number
}

/** 更新 Host 请求参数 */
export interface UpdateHostRequest {
  name?: string
  content?: string
  description?: string
  is_active?: boolean
  group_id?: number
}

/** 应用 hosts 到系统文件的结果 */
export interface ApplyHostsResult {
  success: boolean
  entries_count: number
  dns_flushed: boolean
  message: string
}

/**
 * 获取所有 hosts 列表
 */
export function getAllHosts() {
  return useBridgeFunc<Result<Hosts[]>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_hosts')
      .then(res => resolve(res as Result<Hosts[]>))
      .catch(err => reject(err))
  })
}

/**
 * 根据 ID 获取单个 host
 * @param hostId Host ID
 */
export function getHostById(hostId: number) {
  return useBridgeFunc<Result<Hosts>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_host', { host_id: hostId })
      .then(res => resolve(res as Result<Hosts>))
      .catch(err => reject(err))
  })
}

/**
 * 创建新 host
 * @param request 创建参数
 */
export function createHost(request: CreateHostRequest) {
  return useBridgeFunc<Result<Hosts>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('create_host', { request })
      .then(res => resolve(res as Result<Hosts>))
      .catch(err => reject(err))
  })
}

/**
 * 更新 host
 * @param hostId Host ID
 * @param request 更新参数
 */
export function updateHost(hostId: number, request: UpdateHostRequest) {
  return useBridgeFunc<Result<Hosts>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('update_host', { host_id: hostId, request })
      .then(res => resolve(res as Result<Hosts>))
      .catch(err => reject(err))
  })
}

/**
 * 删除 host
 * @param hostId Host ID
 */
export function deleteHost(hostId: number) {
  return useBridgeFunc<Result<void>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('delete_host', { host_id: hostId })
      .then(res => resolve(res as Result<void>))
      .catch(err => reject(err))
  })
}

/**
 * 切换 host 激活状态
 * @param hostId Host ID
 */
export function toggleHostActive(hostId: number) {
  return useBridgeFunc<Result<Hosts>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('toggle_host_active', { host_id: hostId })
      .then(res => resolve(res as Result<Hosts>))
      .catch(err => reject(err))
  })
}

/**
 * 获取所有已激活的 hosts
 */
export function getActiveHosts() {
  return useBridgeFunc<Result<Hosts[]>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_active_hosts')
      .then(res => resolve(res as Result<Hosts[]>))
      .catch(err => reject(err))
  })
}

/**
 * 将所有激活的 hosts 应用到系统 hosts 文件
 */
export function applyHostsToSystem() {
  return useBridgeFunc<Result<ApplyHostsResult>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('apply_hosts_to_system')
      .then(res => resolve(res as Result<ApplyHostsResult>))
      .catch(err => reject(err))
  })
}

/**
 * 读取系统 hosts 文件内容（只读）
 */
export function readSystemHostsFile() {
  return useBridgeFunc<Result<string>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('read_system_hosts_file')
      .then(res => resolve(res as Result<string>))
      .catch(err => reject(err))
  })
}

/**
 * 搜索 hosts
 * @param request 搜索参数
 */
export function searchHosts(request: {
  keyword: string
  group_id?: number
  is_active?: boolean
  page: number
  page_size: number
}) {
  return useBridgeFunc<Result<ListResult<Hosts>>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('search_hosts', { request })
      .then(res => resolve(res as Result<ListResult<Hosts>>))
      .catch(err => reject(err))
  })
}
