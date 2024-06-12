<script lang="ts">
/**
 * 命令设置
 */
export default {
  name: 'CMD',
}
</script>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { useMessage } from 'naive-ui'
import { onBeforeMount, reactive } from 'vue'
import { useSettingsStore } from '@/store/useSettings'
import type { SettingSpace } from '@/store/useSettings'

const store = useSettingsStore()
const message = useMessage()
const { t } = useI18n()
const modelData = reactive<SettingSpace.Cmd & {
  useFormat: boolean
}>({
  cmd: '',
  useFormat: true,
})

onBeforeMount(() => {
  for (const [key, value] of Object.entries(store.cmd))
    Reflect.set(modelData, key, value)
})

function confirm() {
  cancel()
  store.setSettingsByData({ cmd: modelData })
  message.success(t('更新配置成功！'))
}
function cancel() {
  store.hide()
}
</script>

<template>
  <div class="h-[calc(100%-44px)] pt-10px">
    <!-- shell 编辑器 -->
    <editor class="editor" v-model="modelData.cmd" language="shell" id="cmd-editor" :options="{
      readOnly: false
    }" :format="true" />
  </div>
  <!-- footer -->
  <section class="row f-end a-center">
    <n-space class="user-control a-center mt-10px">
      <slot name="control" />
      <!-- 取消 -->
      <n-button strong secondary type="error" @click="cancel">
        {{ $t("cancel") }}
      </n-button>
      <!-- 确认 -->
      <n-button strong secondary type="primary" @click="confirm">
        {{ $t("confirm") }}
      </n-button>
    </n-space>
  </section>
</template>

<style lang="stylus" scoped>
.tip
  color #718096
  font-size 12px
  margin-top 6px
</style>
