import { useBridgeFunc } from '@/utils'

/**
 * 获取所有操作日志
 * @returns Promise
 */
export function getAllLogs() {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('get_logs')
      .then(res => resolve(res as any))
      .catch(err => reject(err))
  })
}

export interface CreateLogOptions {
  action: string
  targetType: string
  targetId?: number
  details?: string
}

/**
 * 创建操作日志
 * @param options 日志选项
 * @returns Promise
 */
export function insertLog(options: CreateLogOptions) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.core.invoke('create_log', {
      request: {
        action: options.action,
        target_type: options.targetType,
        target_id: options.targetId ?? null,
        details: options.details ?? null,
      },
    })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })
}
