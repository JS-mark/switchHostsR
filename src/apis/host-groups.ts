import { useBridgeFunc } from '@/utils'

import type { Result } from './public'

/** 主机组接口（含统计信息） */
export interface HostGroup {
  id: number
  user_id: number
  name: string
  description: string | null
  is_active: number
  created_at: number
  updated_at: number
  host_count?: number
  active_host_count?: number
}

/** 创建主机组请求 */
export interface CreateHostGroupRequest {
  name: string
  description?: string
  is_active?: boolean
}

/** 更新主机组请求 */
export interface UpdateHostGroupRequest {
  name?: string
  description?: string
  is_active?: boolean
}

/**
 * 获取所有主机组
 */
export function getHostGroups() {
  return useBridgeFunc<Result<HostGroup[]>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_host_groups')
      .then(res => resolve(res as Result<HostGroup[]>))
      .catch(err => reject(err))
  })
}

/**
 * 创建主机组
 * @param request 创建参数
 */
export function createHostGroup(request: CreateHostGroupRequest) {
  return useBridgeFunc<Result<HostGroup>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('create_host_group', { request })
      .then(res => resolve(res as Result<HostGroup>))
      .catch(err => reject(err))
  })
}

/**
 * 更新主机组
 * @param groupId 主机组 ID
 * @param request 更新参数
 */
export function updateHostGroup(groupId: number, request: UpdateHostGroupRequest) {
  return useBridgeFunc<Result<HostGroup>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('update_host_group', { group_id: groupId, request })
      .then(res => resolve(res as Result<HostGroup>))
      .catch(err => reject(err))
  })
}

/**
 * 删除主机组
 * @param groupId 主机组 ID
 */
export function deleteHostGroup(groupId: number) {
  return useBridgeFunc<Result<void>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('delete_host_group', { group_id: groupId })
      .then(res => resolve(res as Result<void>))
      .catch(err => reject(err))
  })
}

/**
 * 切换主机组激活状态
 * @param groupId 主机组 ID
 */
export function toggleGroupActive(groupId: number) {
  return useBridgeFunc<Result<HostGroup>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('toggle_group_active', { group_id: groupId })
      .then(res => resolve(res as Result<HostGroup>))
      .catch(err => reject(err))
  })
}

/**
 * 添加 host 到组
 * @param groupId 主机组 ID
 * @param hostId Host ID
 */
export function addHostToGroup(groupId: number, hostId: number) {
  return useBridgeFunc<Result<void>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('add_host_to_group', { group_id: groupId, host_id: hostId })
      .then(res => resolve(res as Result<void>))
      .catch(err => reject(err))
  })
}

/**
 * 从组中移除 host
 * @param groupId 主机组 ID
 * @param hostId Host ID
 */
export function removeHostFromGroup(groupId: number, hostId: number) {
  return useBridgeFunc<Result<void>>(() => {
    // NOTE: uTools 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('remove_host_from_group', { group_id: groupId, host_id: hostId })
      .then(res => resolve(res as Result<void>))
      .catch(err => reject(err))
  })
}
