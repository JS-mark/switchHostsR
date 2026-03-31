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

import type { SettingSpace } from '@/store/useSettings'

import { useEditor } from '@/components/editor/hook'
import { useSettingsStore } from '@/store/useSettings'

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
  for (const [key, value] of Object.entries(store.cmd)) {
    Reflect.set(modelData, key, value)
  }
  // 更新编辑器数据
  useEditor('cmd-editor', (editor) => {
    editor.setValue(modelData.cmd)
  })
  console.log(modelData)
})

function confirm() {
  store.setSettingsByData({ cmd: modelData })
  message.success(t('更新配置成功！'))
  cancel()
}
function cancel() {
  store.hide()
}
</script>

<template>
  <div class="h-[calc(100%-44px)] pt-10px">
    <!-- shell 编辑器 -->
    <editor
      id="cmd-editor"
      v-model="modelData.cmd"
      class="editor"
      language="shell"
      :options="{
        readOnly: false,
      }"
      :format="true"
    />
  </div>
  <!-- footer -->
  <section class="row f-end a-center">
    <n-space class="user-control a-center mt-10px">
      <slot name="control" />
      <!-- 取消 -->
      <n-button strong secondary size="small" type="error" @click="cancel">
        {{ $t("cancel") }}
      </n-button>
      <!-- 确认 -->
      <n-button strong secondary size="small" type="primary" @click="confirm">
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
