import { useBridgeFunc } from '@/utils'

import type { GetPage, ListResult } from './public'

export interface Hosts {
  id: number
  name: string
  hosts_type: number // 0: 本地, 1: 远程
  hosts_path: string // 路径
  content: string // 内容
  status: number // 0: 启用, 1: 未启用
  is_del: number // 0: 未删除, 1: 已删除
  is_readonly: number // 0: 非只读, 1: 只读
  created_at: string
  updated_at: string
  hosts_refresh_time: number // 刷新时间
  last_refresh_time: string // 最后刷新时间
}

export type HostsData = Omit<Hosts, 'id' | 'is_del' | 'created_at' | 'updated_at' | 'hosts_refresh_time' | 'last_refresh_time' | 'is_readonly'>

export function getAllHosts(options: GetPage) {
  return useBridgeFunc<ListResult<Hosts>>(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_all_hosts_data', options)
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}

/**
 * 更新 hosts data
 * @param id
 * @param data
 */
export function updateHostsData(id: number, data: HostsData) {
  return useBridgeFunc(() => {
    // NOTE: 暂未实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('update_host', {
      id,
      host: { ...data },
    }).then(res => resolve(res)).catch(err => reject(err))
  })
}
