import { EventEmitter } from '@/utils/event'

import type { tauriAPI } from './tauri'

import tauriBridge from './tauri'
import { initUTool } from './utools'

// 状态
const enum STATUS {
  INIT = 'init',
  READY = 'ready',
  FAILED = 'failed',
}

export type useBridgeCb<T> = T extends SYSTEM_ENV.TAURI ? typeof tauriAPI : UToolsApi

// env
export const enum SYSTEM_ENV {
  NONE = 'none',
  UTOOLS = 'utools',
  TAURI = 'tauri',
}

/**
 * Bridge
 */
export class Bridge extends EventEmitter {
  private _status: STATUS = STATUS.INIT
  private _env: SYSTEM_ENV = SYSTEM_ENV.NONE
  private _bridge?: useBridgeCb<SYSTEM_ENV[keyof SYSTEM_ENV]>
  constructor(env: SYSTEM_ENV) {
    super()
    this._env = env
    this.once(STATUS.READY, this.readyBridge.bind(this, this._bridge))
    this.createBridge()
  }

  get status() {
    return this._status
  }

  get env() {
    return this._env
  }

  get bridge() {
    return this._bridge
  }

  /**
   * created
   */
  createBridge() {
    (this._env === SYSTEM_ENV.TAURI ? tauriBridge() : initUTool())
      .then((ins) => {
        this._status = STATUS.READY
        this.emit(STATUS.READY, ins)
      })
      .catch(() => {
        this._status = STATUS.FAILED
      })
  }

  /**
   * useBridge
   * @example
   * useBridge<SYSTEM_ENV.TAURI>
   * @param cb
   */
  useBridge<T extends SYSTEM_ENV[keyof SYSTEM_ENV]>(
    cb?: (bridge: useBridgeCb<T>) => void,
  ) {
    if (this._status === STATUS.READY) {
      if (!cb)
        return this._bridge as useBridgeCb<T>
      else cb(this._bridge as useBridgeCb<T>)
    }
    else {
      this.once(STATUS.READY, (bridge) => {
        cb && cb(bridge as useBridgeCb<T>)
      })
    }
  }

  /**
   * ready
   * @param bridge
   */
  readyBridge<T extends keyof SYSTEM_ENV>(bridge?: useBridgeCb<T>) {
    bridge && (this._bridge = bridge)
  }
}
