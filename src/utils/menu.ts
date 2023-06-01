import { h } from "vue";
import { type TFn } from "@/langs";
import { storeToRefs } from "pinia";
import { useUserStore } from "@/store";
import SvgIcon from "@/components/svg.vue";

export const bottomMenus = (t: TFn) => {
  return [
    {
      label: t("home"),
      key: "home",
      icon() {
        return h(SvgIcon, { name: "home", size: "16px" });
      },
    },
    {
      label: t("about"),
      key: "about",
      icon() {
        return h(SvgIcon, {
          name: "about",
          size: "16px",
        });
      },
    },
    { type: "divider" },
    {
      label: t("检查更新"),
      key: "outdated",
      icon() {
        return h(SvgIcon, { name: "updated", size: "16px" });
      },
    },
    {
      label: t("issues"),
      key: "issues",
      icon() {
        return h(SvgIcon, { name: "issues", size: "16px" });
      },
    },
    { type: "divider" },
    {
      label: t("回收站"),
      key: "recycle",
      icon() {
        return h(SvgIcon, { name: "recycle", size: "16px" });
      },
    },
  ];
};

export const rightMenus = (t: TFn) => {
  return [
    {
      label: t("home"),
      key: "home",
      icon() {
        return h(SvgIcon, { name: "home", size: "16px" });
      },
    },
    {
      label: t("about"),
      key: "about",
      icon() {
        return h(SvgIcon, { name: "about", size: "16px" });
      },
    },
    { type: "divider" },
    {
      label: t("检查更新"),
      key: "outdated",
      icon() {
        return h(SvgIcon, { name: "updated", size: "16px" });
      },
    },
    {
      label: t("issues"),
      key: "issues",
      icon() {
        return h(SvgIcon, { name: "issues", size: "16px" });
      },
    },
    { type: "divider" },
    {
      label: t("偏好设置"),
      key: "settings",
      icon() {
        return h(SvgIcon, { name: "settings", size: "16px" });
      },
    },
    {
      label: t("export"),
      key: "export",
      disabled: true,
      icon() {
        return h(SvgIcon, { name: "export", size: "16px" });
      },
    },
    {
      label: t("local-import"),
      key: "local-import",
      disabled: true,
      icon() {
        return h(SvgIcon, { name: "import", size: "16px" });
      },
    },
    {
      label: t("从远程导入"),
      key: "import-remote",
      disabled: true,
      icon() {
        return h(SvgIcon, { name: "import-remote", size: "16px" });
      },
    },
  ];
};

export const personalMenus = (t: TFn) => {
  const store = useUserStore();
  return [
    {
      label: t("quit"),
      key: "quit",
      disabled: !store.isLogin,
      icon() {
        return h(SvgIcon, { name: "quit", size: "16px" });
      },
    },
  ];
};
