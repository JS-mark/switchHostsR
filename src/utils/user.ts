import { useBridgeFunc } from "./common"
import { APP_NAME } from "./constant"


/**
 * 设置登录用户信息
 * @param mode
 * @param user
 */
export const setLoginUser = (mode: string, user: any) => {
  window.sessionStorage.setItem(
    APP_NAME,
    window.btoa(
      encodeURIComponent(
        JSON.stringify({
          mode,
          time: +new Date(),
          info: user,
        }),
      ),
    ),
  )
}

/**
 * 获取登录用户信息
 * @returns any
 */
export function getLoginUser() {
  const data = window.sessionStorage.getItem(APP_NAME)
  return data ? JSON.parse(decodeURIComponent(window.atob(data))) : null
}


export const debugUser = () => {
  const data = {
    avatar_url: 'https://avatars.githubusercontent.com/u/6128107?s=80&v=4',
    created_at: '2024-02-18T07:42:49Z',
    email: 'admin@qq.com',
    id: 3,
    is_del: 0,
    is_third: 0,
    name: 'ad',
    status: 0,
    third_account_uid: '-1',
    updated_at: '2024-02-18T07:42:49Z',
    user_level: 0,
  }
  if (!window.__TAURI_IPC__) {
    setLoginUser('email', data)
    return
  }

  return useBridgeFunc(() => {
    // NOTE: 暂时实现
  }, (bridge, resolve, reject) => {
    bridge.invoke('debug_user', { userId: data.id })
      .then((res) => {
        setLoginUser('email', data)
        resolve(res)
      })
      .catch(err => reject(err))
  })
}
