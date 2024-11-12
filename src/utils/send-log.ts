import { insertLog } from '@/apis'
import { useSettingsStore } from '@/store'

import { getSystemInfo } from './index'

export type LevelType = 'error' | 'warn' | 'system'

export interface Options {
  msg: string
  level?: LevelType
}

/**
 * 发送日志
 * @param options { Options }
 */
export async function sendLog(options: Options): Promise<void> {
  const store = useSettingsStore()
  const systemInfo = (await getSystemInfo()).data
  const data = {
    time: +new Date(),
    content: options.msg,
    system: systemInfo,
    info: {
      ua: window.navigator.userAgent,
    },
    level: options.level || 'system',
  }

  if (store.canSendData) {
    insertLog(2, data)
    console.warn('可以发送日志', data)
  }
  else {
    console.warn('不可以发送日志', data)
  }
}
