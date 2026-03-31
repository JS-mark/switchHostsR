import * as tauriAPI from '@tauri-apps/api'

export { tauriAPI }

export * from '@tauri-apps/api'

/**
 * 动态加载库
 * @returns Promise<Tauri>
 */
export function initTauri() {
  return new Promise((resolve) => {
    tauriAPI && resolve(tauriAPI)
  })
}
