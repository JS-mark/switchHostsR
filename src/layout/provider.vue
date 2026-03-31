<script lang="ts" setup>
defineOptions({
  name: 'Provider',
})
import type { GlobalThemeOverrides } from 'naive-ui'

import { computed, ref } from 'vue'
import { darkTheme, dateZhCN, useOsTheme, zhCN } from 'naive-ui'

import { useSettingsStore } from '@/store'

const appSetting = useSettingsStore()
const osTheme = useOsTheme()
const theme = computed(() => {
  if (appSetting.theme === 'auto')
    return osTheme.value === 'dark' ? darkTheme : null

  return appSetting.theme === 'dark' ? darkTheme : null
})

const themeOverrides = ref<GlobalThemeOverrides>({
  common: {
    primaryColor: '#1677ff',
    errorColor: '#FF4D4F',
    errorColorHover: '#ff7875',
    primaryColorHover: '#4096ff',
  },
  Button: {
    colorError: '#FF4D4F',
    textColorDisabledPrimary: '#fff',
    textColorPrimary: '#fff',
    textColorHover: '#fff',
    textColorHoverPrimary: '#fff',
    colorPrimary: 'rgba(22, 119, 255, 0.8)',
    colorPressedPrimary: 'rgba(22, 119, 255, 0.8)',
    textColorFocusPrimary: '#fff',
    textColorPressedPrimary: '#fff',
    colorHoverPrimary: 'rgba(64, 150, 255, 1)',
    fontSizeMedium: '13px',
    fontSizeSmall: '12px',
    heightMedium: '30px',
    heightSmall: '26px',
    paddingMedium: '0 12px',
    paddingSmall: '0 8px',
    iconSizeMedium: '16px',
    iconSizeSmall: '14px',
  },
})
</script>

<template>
  <n-config-provider :theme="theme" :theme-overrides="themeOverrides" :locale="zhCN" :date-locale="dateZhCN">
    <n-global-style />
    <n-loading-bar-provider>
      <n-notification-provider :max="1">
        <n-message-provider :max="1">
          <n-dialog-provider>
            <n-modal-provider>
              <slot />
            </n-modal-provider>
          </n-dialog-provider>
        </n-message-provider>
      </n-notification-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>
