<script lang="ts">
export default {
  name: 'Settings',
}
</script>

<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { onBeforeMount, ref, watchEffect } from 'vue'

import { APP_NAME } from '@/utils/constant'
import { useSettingsStore } from '@/store/useSettings'

import Cmd from './views/cmd.vue'
import Proxy from './views/proxy.vue'
import General from './views/general.vue'
import Advanced from './views/advanced.vue'

const showModal = ref(false)
const store = useSettingsStore()
const { isShowSettings } = storeToRefs(store)

function onClose() {
  store.hide()
}

onBeforeMount(async () => {
  store.setSettingsByData(
    {
      user: {} as any,
      advanced: {
        hostsPath: `${''}/.${APP_NAME}`,
      },
    },
    false,
  )
})
watchEffect(() => {
  showModal.value = isShowSettings.value
})
</script>

<template>
  <n-modal
    v-model:show="showModal"
    preset="card"
    :title="$t('偏好设置')"
    :bordered="false"
    size="medium"
    style="width: 600px"
    transform-origin="center"
    :on-close="onClose"
    :on-mask-click="onClose"
    :on-esc="onClose"
  >
    <!-- 表单 -->
    <n-tabs
      class="card-tabs"
      default-value="general"
      type="line"
      size="medium"
      animated
      style="margin: 0 -4px"
      pane-style="height: 380px; padding: 0 4px; box-sizing: border-box;"
    >
      <!-- 通用 -->
      <n-tab-pane name="general" :tab="$t('general')">
        <General />
      </n-tab-pane>
      <!-- 命令 -->
      <n-tab-pane name="cmd" :tab="$t('order')">
        <Cmd />
      </n-tab-pane>
      <!-- 代理 -->
      <n-tab-pane name="proxy" :tab="$t('proxy')">
        <Proxy />
      </n-tab-pane>
      <!-- 高级 -->
      <n-tab-pane name="advanced" :tab="$t('advanced')">
        <Advanced />
      </n-tab-pane>
    </n-tabs>
  </n-modal>
</template>

<style lang="stylus" scoped>
.main-modal
  min-width 600px
  max-width 600px
</style>
