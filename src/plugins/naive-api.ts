import { computed } from 'vue'
import { createDiscreteApi, darkTheme } from 'naive-ui'

import { lighten } from '@/utils/index'
import { useSettingsStore } from '@/store'

/**
 * 挂载 Naive-ui 脱离上下文的 API
 * 如果你想在 setup 外使用 useDialog、useMessage、useNotification、useLoadingBar，可以通过 createDiscreteApi 来构建对应的 API。
 * https://www.naiveui.com/zh-CN/dark/components/discrete
 */

export function hookDesignSetting() {
  return computed(() => {
    try {
      const appSettings = useSettingsStore()

      return {
        theme: appSettings.theme ? darkTheme : undefined,
        appTheme: appSettings.theme,
      }
    }
    catch (error) {
      return {
        theme: undefined,
        appTheme: '#1890ff',
      }
    }
  })
}

export function createNaiveDiscreteApi() {
  const designSetting = hookDesignSetting()
  const configProviderPropsRef = computed(() => ({
    theme: designSetting.value.theme,
    themeOverrides: {
      common: {
        primaryColor: designSetting.value.appTheme,
        primaryColorHover: lighten(designSetting.value.appTheme, 6),
        primaryColorPressed: lighten(designSetting.value.appTheme, 6),
      },
      LoadingBar: {
        colorLoading: designSetting.value.appTheme,
      },
    },
  }))
  const { message, dialog, notification, loadingBar } = createDiscreteApi(
    ['message', 'dialog', 'notification', 'loadingBar'],
    {
      messageProviderProps: {
        max: 1,
      },
      notificationProviderProps: {
        max: 1,
      },
      configProviderProps: configProviderPropsRef,
    },
  )
  return {
    message,
    dialog,
    notification,
    loadingBar,
  }
}

let naiveApi: ReturnType<typeof createNaiveDiscreteApi> | undefined

export function useNaiveApi() {
  if (!naiveApi) {
    naiveApi = createNaiveDiscreteApi()
  }
  return naiveApi
}
