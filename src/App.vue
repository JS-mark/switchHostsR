<script lang="ts">
import { useRoute } from 'vue-router'
import { computed, defineComponent } from 'vue'

import Provider from './layout/provider.vue'

export default defineComponent({
  name: 'App',
  components: {
    Provider,
  },
  setup() {
    const route = useRoute()
    const isLoginPage = computed(() => route.name === 'Login')

    return {
      isLoginPage,
    }
  },
})
</script>

<template>
  <Provider>
    <!-- macOS 窗口控制按钮（所有页面） -->
    <WindowControls />

    <!-- 未登录：全屏登录页 -->
    <template v-if="isLoginPage">
      <div class="drag-region" data-tauri-drag-region />
      <router-view v-slot="{ Component, route }">
        <transition name="fade" mode="out-in">
          <component :is="Component" :key="route.path" />
        </transition>
      </router-view>
    </template>

    <!-- 已登录：完整布局 -->
    <template v-else>
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
        </n-layout>
      </n-layout>
      <!-- 添加 hosts 分类抽屉 -->
      <AddHosts />
      <!-- 设置组件 -->
      <Settings />
      <!-- 全屏弹窗 -->
      <GlobalModel />
    </template>
  </Provider>
</template>

<style lang="stylus" scoped>
.main
  width 100vw
  height calc(100vh - 46px)

  & .layout-content
    width 100%
    height 100%

.drag-region
  position fixed
  top 0
  left 0
  right 0
  height 38px
  z-index 99
  -webkit-app-region drag
</style>
