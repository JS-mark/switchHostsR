import { insertLog } from '@/apis'
import { useSettingsStore } from '@/store'

export type LevelType = 'error' | 'warn' | 'system'

export interface Options {
  msg: string
  level?: LevelType
  targetType?: string
  targetId?: number
}

/**
 * 发送日志
 * @param options { Options }
 */
export async function sendLog(options: Options): Promise<void> {
  const store = useSettingsStore()

  if (store.canSendData) {
    insertLog({
      action: options.msg,
      targetType: options.targetType || 'system',
      targetId: options.targetId,
      details: JSON.stringify({ level: options.level || 'system', time: +new Date() }),
    })
    console.warn('可以发送日志', options)
  }
  else {
    console.warn('不可以发送日志', options)
  }
}
