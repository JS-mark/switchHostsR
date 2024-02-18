<script lang="ts">
export default {
  name: 'LHeader',
}
</script>

<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useRouter } from 'vue-router'
import { defineAsyncComponent } from 'vue'
import { APP_NAME } from '@/utils/constant'
import { personalMenus, rightMenus } from '@/utils/menu'
import { useHostsStore, useSettingsStore, useUserStore } from '@/store'

const emits = defineEmits<{
  (event: 'onSettting'): void
  (event: 'onAddHosts'): void
}>()
const SvgIcon = defineAsyncComponent(() => import('@/components/svg.vue'))
const router = useRouter()
const store = useSettingsStore()
const userStore = useUserStore()
const { settings } = storeToRefs(store)
const { show: showAddHosts } = useHostsStore()
const { isLogin, info } = storeToRefs(userStore)
/**
 * methods
 */

function onDropdownSelected(event: any) {
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
    case 'settings':
      // 展示设置
      store.show()
      emits('onSettting')
      break
    case 'quit':
      clearLoginInfo()
      break

    default:
      break
  }
}

function clearLoginInfo() {
  window.sessionStorage.removeItem(APP_NAME)
  userStore.setLogin(false)
  userStore.clearUserInfo()
  router.replace({
    name: 'Login',
  })
}

function onAddHosts() {
  showAddHosts({ mode: 'create', settings: settings.value })
  emits('onAddHosts')
}
</script>

<template>
  <n-layout-header bordered>
    <header class="header row f-between a-center">
      <section row>
        <n-space class="row a-center">
          <SvgIcon name="logo-header" size="35px" />
          <span class="logo-name">{{ APP_NAME }}</span>
        </n-space>
      </section>
      <section class="row f-center a-center">
        <n-space class="user-control a-center">
          <slot name="control" />
          <!-- 添加 -->
          <n-tooltip v-if="isLogin" trigger="hover">
            <template #trigger>
              <n-button
                strong
                secondary
                circle
                type="primary"
                @click="onAddHosts"
              >
                <template #icon>
                  <SvgIcon name="plus" size="16px" />
                </template>
              </n-button>
            </template>
            {{ $t("添加新的 Hosts 内容") }}
          </n-tooltip>
          <!-- 设置 -->
          <n-dropdown
            placement="bottom-end"
            trigger="hover"
            size="large"
            :options="rightMenus($t)"
            :show-arrow="true"
            @select="onDropdownSelected"
          >
            <n-button strong secondary circle type="info">
              <template #icon>
                <SvgIcon name="settings" size="16px" />
              </template>
            </n-button>
          </n-dropdown>
        </n-space>

        <n-divider vertical />
        <n-space class="row f-end a-center">
          <n-dropdown
            placement="bottom-end"
            trigger="hover"
            size="large"
            :options="personalMenus($t)"
            :show-arrow="true"
            @select="onDropdownSelected"
          >
            <n-avatar round size="small" :src="info.avatar_url" />
          </n-dropdown>
        </n-space>
      </section>
    </header>
  </n-layout-header>
</template>

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
