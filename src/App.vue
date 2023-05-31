<template>
  <n-config-provider :theme="theme">
    <n-loading-bar-provider>
      <n-message-provider>
        <!-- 头部 -->
        <l-header></l-header>
        <!-- 主体区域 -->
        <n-layout class="main" has-sider>
          <!-- 侧边栏 -->
          <l-sider></l-sider>
          <n-layout>
            <n-layout-content :class="['layout-content']">
              <router-view v-slot="{ Component, route }">
                <transition name="fade" mode="out-in">
                  <component :is="Component" :key="route.path" />
                </transition>
              </router-view>
              <n-back-top :bottom="100" :visibility-height="300"></n-back-top>
            </n-layout-content>
            <!-- 底部 -->
            <n-layout-footer class="layout-footer" bordered>
              @2023 By Mark
            </n-layout-footer>
          </n-layout>
        </n-layout>
        <!-- 添加 hosts 分类抽屉 -->
        <add-hosts />
        <!-- 设置组件 -->
        <settings />
      </n-message-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>

<script lang="ts">
import { useRoute } from "vue-router";
import { useDark } from "@vueuse/core";
import { computed, defineComponent } from "vue";
import LSider from "@/components/layout/l-sider.vue";
import LHeader from "@/components/layout/l-header.vue";
import { darkTheme, type GlobalTheme } from "naive-ui";
import AddHosts from "@/components/add-hosts/index.vue";
import Settings from "@/components/settings/index.vue";

export default defineComponent({
  name: "App",
  components: {
    LSider,
    LHeader,
    Settings,
    AddHosts,
  },
  setup() {
    const route = useRoute();
    const title = computed(() => {
      return route.meta.title;
    });
    const isDark = useDark({
      selector: "body",
      attribute: "color-scheme",
      valueDark: "dark",
      valueLight: "light",
    });

    const theme = computed<GlobalTheme | null>(() => {
      return isDark.value ? darkTheme : null;
    });

    const isShowHeader = computed(
      () => !(route.name === "Home" || route.name === "NotFound"),
    );

    return {
      title,
      theme,
      isShowHeader,
    };
  },
});
</script>

<style lang="stylus" scoped>
.main
  width 100vw
  height calc(100vh - 58px)

  & .layout-content
    width 100%
    transition height 1.3s liner
    height calc(100% - 46px)
    padding 10px 0

    &.home
      height calc(100% - 96px)

  & .layout-header
    width 100%
    height 50px

  & .layout-footer
    width 100%
    text-align center
    font-size 16px
    padding 10px 0
    font-weight 600
</style>
