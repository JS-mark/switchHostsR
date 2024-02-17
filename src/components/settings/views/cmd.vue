<script lang="ts">
/**
 * 命令设置
 */
export default {
  name: 'CMD',
}
</script>

<script lang="ts" setup>
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { onBeforeMount, reactive } from 'vue'
import type { SettingSpace } from '@/store/useSettings'
import { useSettingsStore } from '@/store/useSettings'

const store = useSettingsStore()
const message = useMessage()
const { t } = useI18n()
const model = reactive<SettingSpace.Cmd & { useFormat: boolean }>({
  cmd: '',
  useFormat: true,
})

onBeforeMount(() => {
  for (const [key, value] of Object.entries(store.cmd))
    Reflect.set(model, key, value)
})

function confirm() {
  cancel()
  store.setSettingsByData({ cmd: model })
  message.success(t('更新配置成功！'))
}
function cancel() {
  store.hide()
}
</script>

<template>
  <div class="form">
    <!-- shell 编辑器 -->
    <editor v-model="model.cmd" :format="model.useFormat" language="shell" class="editor" />
  </div>
  <!-- footer -->
  <section class="row f-end a-center">
    <n-space class="user-control a-center">
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
.form
  height calc(100% - 34px)

.tip
  color #718096
  font-size 12px
  margin-top 6px
</style>
