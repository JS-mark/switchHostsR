import { defineStore } from 'pinia'
import { isEmpty, set } from 'lodash-es'
import { APP_NAME } from '@/utils/constant'

export type UserMode =
  | 'weibo'
  | 'googel'
  | 'github'
  | 'instagram'
  | 'linkedin'
  | 'email'

export interface User {
  nickname?: string
  email?: string
  home?: string
  avatar?: string
  [key: string]: any
}

export const useUserStore = defineStore('user', {
  state: () => ({
    mode: '',
    isLogin: false,
    info: {
      name: '',
      email: '',
      avatar_url: '',
      status: -1,
      is_third: 0,
      user_level: -1,
      third_account_uid: -1,
      created_at: '',
      updated_at: '',
    },
  }),
  actions: {
    clearUserInfo() {
      this.mode = ''
      this.info = {
        name: '',
        email: '',
        avatar_url: '',
        status: -1,
        is_third: 0,
        user_level: -1,
        third_account_uid: -1,
        created_at: '',
        updated_at: '',
      }
    },
    setMode(mode: UserMode) {
      if (!mode)
        return
      this.mode = mode
    },
    setLogin(login: boolean) {
      this.isLogin = login
    },
    /**
     * 设置用户数据
     * @param data
     * @returns { void }
     */
    setUserInfo(data: User) {
      if (!data || isEmpty(data))
        return
      for (const [key, value] of Object.entries(data))
        set(this.info, key, value)

      window.sessionStorage.setItem(
        APP_NAME,
        window.btoa(
          encodeURIComponent(
            JSON.stringify({
              mode: this.mode,
              time: +new Date(),
              info: data,
            }),
          ),
        ),
      )
    },
  },
})
