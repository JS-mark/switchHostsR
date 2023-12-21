<script lang="ts">
export default {
  name: 'LSider',
}
</script>

<script lang="ts" setup>
import { type MenuOption, NIcon } from 'naive-ui'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { h, onMounted, ref, shallowRef, watchEffect } from 'vue'
import { CaretDownOutline, SettingsSharp } from '@vicons/ionicons5'
import {
  menus as defaultMenus,
  defaultRoute,
  genSidersMenus,
} from '@/router/routes'
import emitter from '@/plugins/emitter'
import { bottomMenus } from '@/utils/menu'
import { HistoryMenus } from '@/router/history'

const route = useRoute()
const router = useRouter()
const menus = shallowRef<MenuOption[]>(defaultMenus)

const collapsed = ref(true)
const selectedKey = ref(defaultRoute)

/**
 * Methods
 */
function onCollapse() {
  collapsed.value = true
}

function onExpand() {
  collapsed.value = false
}

function onDropdownSelected(event: string) {
  switch (event) {
    case 'home':
      window.open('https://js-mark.com/mfe-helper')
      break
    case 'about':
      window.open('https://js-mark.com/mfe-helper')
      break
    case 'issues':
      window.open('https://github.com/JS-mark/mfe-helper/issues')
      break
    case 'outdated':
      window.open('https://github.com/JS-mark/mfe-helper')
      break
    case 'recycle':
      console.warn('recycle')
      break
    default:
      break
  }
}

function renderMenuLabel(option: MenuOption) {
  if ('path' in option) {
    return h(
      RouterLink,
      {
        to: {
          name: option.key as string,
        },
      },
      { default: () => option.label },
    )
  }
  return option.label as string
}
function renderMenuIcon(option: MenuOption) {
  return h(NIcon, null, { default: () => option.icon && option.icon() })
}
function expandIcon() {
  return h(NIcon, null, { default: () => h(CaretDownOutline) })
}

emitter.on('onAddRoute', () => {
  setTimeout(() => {
    const routes = router.getRoutes()
    menus.value = genSidersMenus(routes)
  }, 0)
})

emitter.on('onRestoreHistoryRoute', (list) => {
  list.forEach((item) => {
    item.meta!.icon = SettingsSharp
    router.addRoute(item)
  })

  setTimeout(() => {
    const routes = router.getRoutes()
    menus.value = genSidersMenus(routes)
    // 默认为首页
    router.replace(routes[0])
  }, 0)
})

onMounted(() => {
  HistoryMenus()
})

watchEffect(() => {
  selectedKey.value = route.name as string
})
</script>

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
    <section class="sider column f-between">
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
      <n-dropdown
        placement="right"
        trigger="hover"
        size="large"
        :options="bottomMenus($t)"
        :show-arrow="true"
        @select="onDropdownSelected"
      >
        <n-button
          strong
          :quaternary="!collapsed"
          :secondary="collapsed"
          type="info"
          :circle="collapsed"
          class="btn" :class="[{ collapsed }]"
        >
          <template #icon>
            <svg-icon name="settings" size="16px" />
          </template>
          <template v-if="!collapsed">
            {{ $t("更多") }}
          </template>
        </n-button>
      </n-dropdown>
    </section>
  </n-layout-sider>
</template>

<style lang="stylus" scoped>
.sider
  height 100%

  & .btn
    margin 10px
    box-sizing border-box
    width calc(100% - 20px)

    &.collapsed
      width auto
</style>
