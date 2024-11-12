import { Bridge, SYSTEM_ENV, type useBridgeCb } from './bridge-class'

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

export { Bridge, SYSTEM_ENV, useBridgeCb }
