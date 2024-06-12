import { useBridgeFunc } from '@/utils'
import type { GetPage } from './public'

/**
 * 获取所有操作日志
 * @returns Promise
 */
export function getAllLogs(options: GetPage) {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.invoke('get_all_logs', options)
      .then(res => resolve(res as any))
      .catch(err => reject(err))
  })
}

/**
 * 插入本地操作日志
 * @param logType -- 操作类型 0:更新,1:删除,2:增加,-1:未知
 * @param { object } data
 * @returns Promise
 */
export function insertLog(logType: number, data: Record<string, any>) {
  const content = JSON.stringify(data)
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.invoke('add_log', {
      logOptions: {
        // logType: 2, // 0: 更新，1 删除，2 增加，-1 未知
        content,
        log_type: logType,
      },
    })
      .then((res) => {
        resolve(res)
      })
      .catch(err => reject(err))
  })
}
