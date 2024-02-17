<script lang="ts">
import { useRoute } from 'vue-router'
import { useDark } from '@vueuse/core'
import { computed, defineComponent } from 'vue'
import { type GlobalTheme, darkTheme } from 'naive-ui'
import { useSettingsStore } from '@/store'

export default defineComponent({
  name: 'App',
  setup() {
    const route = useRoute()
    const settingsStore = useSettingsStore()
    const title = computed(() => {
      return route.meta.title
    })
    const isDark = useDark({
      selector: 'body',
      attribute: 'color-scheme',
      valueDark: 'dark',
      valueLight: 'light',
    })

    const theme = computed<GlobalTheme | null>(() => {
      if (settingsStore.theme === 'auto')
        return isDark.value ? darkTheme : null
      if (settingsStore.theme === 'black')
        return darkTheme
      return null
    })

    const isShowHeader = computed(
      () => !(route.name === 'Home' || route.name === 'NotFound'),
    )

    return {
      title,
      theme,
      isShowHeader,
    }
  },
})
</script>

<template>
  <n-config-provider :theme="theme">
    <n-loading-bar-provider>
      <n-message-provider :max="1">
        <!-- 头部 -->
        <LHeader />
        <!-- 主体区域 -->
        <n-layout class="main" has-sider>
          <!-- 侧边栏 -->
          <LSider />
          <n-layout>
            <n-layout-content class="layout-content">
              <router-view v-slot="{ Component, route }">
                <transition name="fade" mode="out-in">
                  <component :is="Component" :key="route.path" />
                </transition>
              </router-view>
              <n-back-top :bottom="100" :visibility-height="300" />
            </n-layout-content>
            <!-- 底部 -->
            <n-layout-footer class="layout-footer" bordered>
              @2023 By Mark
            </n-layout-footer>
          </n-layout>
        </n-layout>
        <!-- 添加 hosts 分类抽屉 -->
        <AddHosts />
        <!-- 设置组件 -->
        <Settings />
        <!-- 全屏弹窗 -->
        <GlobalModel />
      </n-message-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>

<style lang="stylus" scoped>
.main
  width 100vw
  height calc(100vh - 58px)

  & .layout-content
    width 100%
    transition height 1.3s liner
    height calc(100% - 46px)

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
