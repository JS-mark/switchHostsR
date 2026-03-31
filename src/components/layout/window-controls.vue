<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'

defineOptions({
  name: 'WindowControls',
})

const appWindow = getCurrentWindow()

function onClose() {
  appWindow.hide()
}

function onMinimize() {
  appWindow.minimize()
}

async function onFullscreen() {
  const isFullscreen = await appWindow.isFullscreen()
  await appWindow.setFullscreen(!isFullscreen)
}
</script>

<template>
  <div class="window-controls" data-tauri-drag-region>
    <div class="traffic-lights">
      <!-- 关闭 -->
      <button class="btn btn-close" @click="onClose">
        <svg class="icon" viewBox="0 0 12 12">
          <path d="M3.5 3.5L8.5 8.5M8.5 3.5L3.5 8.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" />
        </svg>
      </button>
      <!-- 最小化 -->
      <button class="btn btn-minimize" @click="onMinimize">
        <svg class="icon" viewBox="0 0 12 12">
          <path d="M2.5 6H9.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" />
        </svg>
      </button>
      <!-- 最大化 -->
      <button class="btn btn-maximize" @click="onFullscreen">
        <svg class="icon" viewBox="0 0 12 12">
          <!-- 左上箭头 -->
          <path d="M2 5.5V2H5.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" fill="none" />
          <path d="M2 2L5 5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
          <!-- 右下箭头 -->
          <path d="M10 6.5V10H6.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" fill="none" />
          <path d="M10 10L7 7" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style lang="stylus" scoped>
.window-controls
  position fixed
  top 0
  left 0
  height 38px
  z-index 100
  display flex
  align-items center
  padding-left 12px
  padding-right 12px
  pointer-events none

  .traffic-lights
    display flex
    align-items center
    gap 8px
    pointer-events auto

    .btn
      width 14px
      height 14px
      border-radius 50%
      border none
      cursor pointer
      display flex
      align-items center
      justify-content center
      padding 0
      outline none
      position relative

      .icon
        width 8px
        height 8px
        color transparent
        transition color 0.1s

      &:hover .icon
        color rgba(0, 0, 0, 0.5)

      &:active
        opacity 0.7

    .btn-close
      background-color #ff5f57
      box-shadow inset 0 0 0 0.5px rgba(0, 0, 0, 0.12)

    .btn-minimize
      background-color #febc2e
      box-shadow inset 0 0 0 0.5px rgba(0, 0, 0, 0.12)

    .btn-maximize
      background-color #28c840
      box-shadow inset 0 0 0 0.5px rgba(0, 0, 0, 0.12)

  // hover 整个 traffic-lights 区域时显示所有图标
  .traffic-lights:hover .btn .icon
    color rgba(0, 0, 0, 0.5)
</style>
