import { getSystemInfo } from "./index";
import { useSettingsStore } from "@/store";
export type LevelType = "error" | "warn" | "system";

export interface Options {
  msg: string;
  level?: LevelType;
}

/**
 * 发送日志
 * @param msg
 */
export const sendLog = (options: Options): void => {
  const store = useSettingsStore();
  const data = {
    time: +new Date(),
    content: options.msg,
    system: getSystemInfo(),
    level: options.level || "system",
  };

  if (store.canSendData) {
    console.log("可以发送日志", data);
  } else {
    console.log("不可以发送日志", data);
  }
};
