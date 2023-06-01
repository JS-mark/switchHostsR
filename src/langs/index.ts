import en from "./en";
import ja from "./ja";
import zhCN from "./zh-cn";
import { createI18n } from "vue-i18n";

const messages = {
  "zh-cn": zhCN,
  en,
  ja,
};

const i18n = createI18n({
  legacy: false,
  locale: "zh-cn",
  messages,
});

export type Lang = keyof typeof messages;

/**
 * 更新 lang
 * @param key
 */
export const setDefaultLang = (key: string) => {
  i18n.global.locale.value = key as Lang;
};

export const useLanguages = () => {
  return [
    { label: "中文", key: "zh-cn" },
    { label: "English", key: "en" },
    { label: "日本語", key: "ja" },
  ];
};

export type TFn = typeof i18n.global.t;

export default i18n;
