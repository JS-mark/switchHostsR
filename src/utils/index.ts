import { User } from "@/store";
import { urlAlphabet, customAlphabet } from "nanoid";
import { useJSBridge, SYSTEM_ENV } from "@/plugins/Bridge";
import { APP_NAME } from "./constant";

export const getRandID = customAlphabet(urlAlphabet, 16);

export const env = window.utools ? SYSTEM_ENV.UTOOLS : SYSTEM_ENV.TAURI;

export const { useBridge } = useJSBridge();

/**
 * 获取系统信息
 * @returns
 */
export const getSystemInfo = () => {
  return {
    ua: window.navigator.userAgent,
  };
};

/**
 * 打开文件
 * @param file
 */
export const openFile = (file?: string) => {
  if (!file) return;
  if (env === SYSTEM_ENV.UTOOLS) {
    useBridge((bridge) => {
      bridge.shellShowItemInFolder(file);
    });
  }

  if (env === SYSTEM_ENV.TAURI) {
    useBridge<SYSTEM_ENV.TAURI>((bridge) => {
      bridge("openFile", {
        file,
      });
    });
  }
};

/**
 * 打开文件夹
 * @param file
 */
export const openDirectory = (file?: string) => {
  return new Promise((resolve) => {
    if (env === SYSTEM_ENV.UTOOLS) {
      useBridge((bridge) => {
        const paths = bridge.showOpenDialog({
          title: "选择储存数据文件夹",
          defaultPath: file || bridge.getPath("home"),
          properties: ["openDirectory"],
        });
        resolve(paths && paths[0]);
      });
    }

    if (env === SYSTEM_ENV.TAURI) {
      useBridge<SYSTEM_ENV.TAURI>((bridge) => {
        bridge("openDirectory", {
          file,
        }).then((res) => {
          resolve(res);
        });
      });
    }
  });
};

interface UserInfo extends User {
  ua: string;
  home: string;
}

/**
 * 获取登录用户信息
 * @param name
 * @returns
 */
export const getLoginUser = () => {
  const data = window.sessionStorage.getItem(APP_NAME);
  return data ? JSON.parse(decodeURIComponent(window.atob(data))) : null;
};

/**
 * 获取用户信息
 * @param file
 */
export const getUserInfo = (): Promise<UserInfo> => {
  return new Promise((resolve) => {
    if (env === SYSTEM_ENV.UTOOLS) {
      useBridge((bridge) => {
        const user = bridge.getUser();
        resolve({
          ua: window.navigator.userAgent,
          home: bridge.getPath("home"),
          ...user,
        });
      });
    }

    if (env === SYSTEM_ENV.TAURI) {
      useBridge<SYSTEM_ENV.TAURI>((bridge) => {
        bridge("getUserInfo").then((res: any) => {
          resolve({
            ua: window.navigator.userAgent,
            home: res.home,
            ...res.user,
          });
        });
      });
    }
  });
};
/**
 * 获取类型
 * @param o
 * @returns type
 */
export const getType = (o: unknown): string => {
  const s: string = Object.prototype.toString.call(o);
  const s_ = s.match(/\[object (.*)\]/) as RegExpMatchArray;
  return s_[1].toLowerCase();
};
