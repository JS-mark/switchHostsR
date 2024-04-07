import { useBridgeFunc } from '@/utils'
import type { GetPage } from './public'

/**
 * 获取所有操作日志
 * @returns Promise
 */
export const getAllLogs = (options: GetPage) => {
  return useBridgeFunc(() => {
    // NOTE: 待实现
  }, (bridge, resolve, reject) => {
    bridge.invoke('get_all_logs', options)
      .then(res => resolve(res as any))
      .catch(err => reject(err))
  })
}
