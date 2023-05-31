<template>
  <n-layout-header bordered>
    <header class="header row f-between a-center">
      <section row>
        <n-space class="row a-center">
          <svg-icon name="logo-header" size="35px"></svg-icon>
          <span class="logo-name">{{ APP_NAME }}</span>
        </n-space>
      </section>
      <section class="row f-center a-center">
        <n-space class="user-control a-center">
          <slot name="control"></slot>
          <!-- 添加 -->
          <n-tooltip trigger="hover">
            <template #trigger>
              <n-button
                strong
                secondary
                circle
                type="primary"
                @click="onAddHosts"
              >
                <template #icon>
                  <svg-icon name="plus" size="16px"></svg-icon>
                </template>
              </n-button>
            </template>
            添加新的 Hosts 内容
          </n-tooltip>
          <!-- 设置 -->
          <n-dropdown
            placement="bottom-end"
            trigger="hover"
            size="large"
            :options="options"
            :show-arrow="true"
            @select="onDropdownSelected"
          >
            <n-button strong secondary circle type="info">
              <template #icon>
                <svg-icon name="settings" size="16px"></svg-icon>
              </template>
            </n-button>
          </n-dropdown>
        </n-space>

        <n-divider vertical />
        <n-space class="row f-end a-center">
          <n-badge :value="info.messageNum">
            <n-avatar round size="small" :src="info.avatar"></n-avatar>
          </n-badge>
        </n-space>
      </section>
    </header>
  </n-layout-header>
</template>

<script lang="ts">
export default {
  name: "LHeader",
};
</script>

<script lang="ts" setup>
import { h } from "vue";
import { storeToRefs } from "pinia";
import SvgIcon from "@/components/svg.vue";
import { APP_NAME } from "@/utils/constant";
import { useUserStore, useHostsStore, useSettingsStore } from "@/store";
const store = useSettingsStore();
const userStore = useUserStore();
const { show: showAddHosts } = useHostsStore();
const { settings } = storeToRefs(store);
const { info } = storeToRefs(userStore);
const emits = defineEmits<{
  (event: "onSettting"): void;
  (event: "onAddHosts"): void;
}>();

const options = [
  {
    label: "主页",
    key: "home",
    icon() {
      return h(SvgIcon, { name: "home", size: "16px" });
    },
  },
  {
    label: "关于",
    key: "about",
    icon() {
      return h(SvgIcon, { name: "about", size: "16px" });
    },
  },
  { type: "divider" },
  {
    label: "检查更新",
    key: "outdated",
    icon() {
      return h(SvgIcon, { name: "updated", size: "16px" });
    },
  },
  {
    label: "意见反馈",
    key: "issues",
    icon() {
      return h(SvgIcon, { name: "issues", size: "16px" });
    },
  },
  { type: "divider" },
  {
    label: "偏好设置",
    key: "settings",
    icon() {
      return h(SvgIcon, { name: "settings", size: "16px" });
    },
  },
  {
    label: "导出",
    key: "export",
    disabled: true,
    icon() {
      return h(SvgIcon, { name: "export", size: "16px" });
    },
  },
  {
    label: "从本地导入",
    key: "import-local",
    disabled: true,
    icon() {
      return h(SvgIcon, { name: "import", size: "16px" });
    },
  },
  {
    label: "从远程导入",
    key: "import-remote",
    disabled: true,
    icon() {
      return h(SvgIcon, { name: "import-remote", size: "16px" });
    },
  },
];

/**
 * methods
 */

const onDropdownSelected = (event: any) => {
  switch (event) {
    case "home":
      window.open("https://js-mark.com/mfe-helper");
      break;
    case "about":
      window.open("https://js-mark.com/mfe-helper");
      break;
    case "issues":
      window.open("https://github.com/JS-mark/mfe-helper/issues");
      break;
    case "outdated":
      window.open("https://github.com/JS-mark/mfe-helper");
      break;
    case "settings":
      // 展示设置
      store.show();
      emits("onSettting");
      break;

    default:
      break;
  }
};

const onAddHosts = () => {
  showAddHosts({ mode: "create", settings: settings.value });
  emits("onAddHosts");
};
</script>

<style lang="stylus" scoped>
.header
  width 100%
  padding 10px 25px
  box-sizing border-box

  & .logo-name
    color #333
    font-size 18px
    font-weight 600

  & .user
    &-control
      margin-right 10px

  & .label-name
    color #333
    font-weight 600
</style>
