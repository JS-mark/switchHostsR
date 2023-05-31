import { defineStore } from "pinia";
import { set, isEmpty } from "lodash-es";

export type UserMode =
  | "weibo"
  | "googel"
  | "github"
  | "instagram"
  | "linkedin"
  | "email";

export interface User {
  nickname: string;
  email: string;
  home: string;
  avatar: string;
  messageNum: number;
  [key: string]: any;
}

export const useUserStore = defineStore("user", {
  state: () => ({
    mode: "",
    isLogin: false,
    info: {
      nickname: "",
      email: "",
      home: "",
      avatar: "",
      messageNum: 0,
    },
  }),
  actions: {
    setMode(mode: UserMode) {
      if (!mode) return;
      this.mode = mode;
    },
    setLogin(login: boolean) {
      this.isLogin = login;
    },
    /**
     * 设置用户数据
     * @param content
     * @returns
     */
    setUserInfo(data: User) {
      if (!data || isEmpty(data)) return;
      for (const [key, value] of Object.entries(data)) {
        set(this.info, key, value);
      }
    },
  },
});