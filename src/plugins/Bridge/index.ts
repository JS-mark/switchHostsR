import { initUTool } from './utools'
import Emitter from './emitter'
import type { invoke } from './tauri-bridge'
import tauriBridge from './tauri-bridge'

// 状态
const enum STATUS {
  INIT = 'init',
  READY = 'ready',
  FAILED = 'failed',
}

type useBridgeCb<T> = T extends SYSTEM_ENV.TAURI ? typeof invoke : UToolsApi

// env
export const enum SYSTEM_ENV {
  NONE = 'none',
  UTOOLS = 'utools',
  TAURI = 'tauri',
}

/**
 * Bridge
 */
class Bridge extends Emitter {
  private status: STATUS = STATUS.INIT
  private env: SYSTEM_ENV = SYSTEM_ENV.NONE
  private bridge: UToolsApi | typeof invoke | unknown
  constructor(env: SYSTEM_ENV) {
    super()
    this.env = env
    this.bridge = null
    // @ts-expect-error
    this.on(STATUS.READY, this.readyBridge.bind(this))
    this.createBridge()
  }

  /**
   * created
   */
  createBridge() {
    (this.env === SYSTEM_ENV.TAURI ? tauriBridge() : initUTool())
      .then((ins) => {
        this.status = STATUS.READY
        this.emit(STATUS.READY, ins)
      })
      .catch(() => {
        this.status = STATUS.FAILED
      })
  }

  /**
   * useBridge
   * @example
   * useBridge<SYSTEM_ENV.TAURI>
   * @param cb
   * @returns
   */
  useBridge<T extends SYSTEM_ENV[keyof SYSTEM_ENV]>(
    cb?: (bridge: useBridgeCb<T>) => void,
  ) {
    if (this.status === STATUS.READY) {
      if (!cb)
        return this.bridge as useBridgeCb<T>
      else cb(this.bridge as useBridgeCb<T>)
    }
    else {
      this.once(STATUS.READY, (bridge) => {
        cb && cb(bridge as useBridgeCb<T>)
      })
    }
  }

  /**
   * ready
   * @param env
   */
  readyBridge<T extends keyof SYSTEM_ENV>(bridge: useBridgeCb<T>) {
    this.bridge = bridge
  }
}

/**
 * 桥方法
 */
export default Bridge

export function useJSBridge() {
  const env = window.utools ? SYSTEM_ENV.UTOOLS : SYSTEM_ENV.TAURI
  const ins = new Bridge(env)
  return {
    on: ins.on.bind(ins),
    off: ins.off.bind(ins),
    emit: ins.emit.bind(ins),
    useBridge: ins.useBridge.bind(ins),
  }
}
