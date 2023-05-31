import * as tauriAPI from "@tauri-apps/api";
export {
  invoke,
  app,
  cli,
  clipboard,
  dialog,
  fs,
  globalShortcut,
  path,
  updater,
  os,
  notification,
  http,
  process,
  shell,
  tauri,
  event as tauriEvent,
  window as tauriWindow,
} from "@tauri-apps/api";

/**
 * 动态加载库
 * @returns
 */
export default function initTauri() {
  return new Promise((resolve) => {
    tauriAPI && resolve(tauriAPI);
  });
}
