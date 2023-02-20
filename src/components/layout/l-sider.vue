<template>
  <n-layout-sider
    bordered
    collapse-mode="width"
    :collapsed-width="64"
    :collapsed="collapsed"
    show-trigger
    @collapse="onCollapse"
    @expand="onExpand"
  >
    <n-menu
      v-model:value="selectedKey"
      accordion
      :collapsed="collapsed"
      :collapsed-width="64"
      :collapsed-icon-size="20"
      :options="menus"
      :render-label="renderMenuLabel"
      :render-icon="renderMenuIcon"
      :expand-icon="expandIcon"
    />
  </n-layout-sider>
</template>


<script lang="ts">
export default {
  name: "LSider"
}
</script>

<script lang="ts" setup>
import emitter from '@/plugins/emitter'
import { HistoryMenus } from '@/router/history'
import { NIcon, type MenuOption } from 'naive-ui'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { shallowRef, ref, h, watchEffect, onMounted } from 'vue'
import { menus as defaultMenus, genSidersMenus, defaultRoute } from '@/router/routes'

import {
  CaretDownOutline,
  SettingsSharp
} from '@vicons/ionicons5'

const route = useRoute()
const router = useRouter()
const menus = shallowRef<MenuOption[]>(defaultMenus)
const collapsed = ref(false)
const selectedKey = ref(defaultRoute)

/**
 * Methods
 */
const onCollapse = () => {
  collapsed.value = true
}

const onExpand = () => {
  collapsed.value = false
}

const renderMenuLabel = (option: MenuOption) => {
  if ('path' in option) {
    return h(
      RouterLink,
      {
        to: {
          name: option.key as string,
        }
      },
      { default: () => option.label }
    )
  }
  return option.label as string
}
const renderMenuIcon = (option: MenuOption) => {
  return h(NIcon, null, { default: () => option.icon && option.icon() })
}
const expandIcon = () =>  {
  return h(NIcon, null, { default: () => h(CaretDownOutline) })
}

emitter.on("onAddRoute", () => {
  setTimeout(() => {
    const routes = router.getRoutes()
    menus.value = genSidersMenus(routes)
  }, 0);
})

emitter.on("onRestoreHistoryRoute", (list) => {
  list.forEach(item => {
    // @ts-ignore
    item.meta.icon = SettingsSharp
    router.addRoute(item)
  })

  setTimeout(() => {
    const routes = router.getRoutes()
    menus.value = genSidersMenus(routes)
    // 默认为首页
    router.replace(routes[0])
  }, 0);
})

onMounted(() => {
  HistoryMenus()
})

watchEffect(() => {
  selectedKey.value = route.name as string
})

</script>
