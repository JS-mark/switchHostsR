<script lang="ts">
/**
 * 代理设置
 */
export default {
  name: 'Proxy',
}
</script>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { useMessage } from 'naive-ui'
import { onBeforeMount, reactive, ref } from 'vue'
import type { SettingSpace } from '@/store/useSettings'
import { useSettingsStore } from '@/store/useSettings'

const { t } = useI18n()
const store = useSettingsStore()
const message = useMessage()
const formRef = ref(null)
const model = reactive<SettingSpace.Proxy>({
  origin: 'http',
  useProxy: false,
  host: '',
  port: '',
})

onBeforeMount(() => {
  for (const [key, value] of Object.entries(store.proxy))
    Reflect.set(model, key, value)
})

const origins = [
  { label: 'HTTP', key: 'http' },
  { label: 'HTTPS', key: 'https' },
]

function confirm() {
  cancel()
  store.setSettingsByData({ proxy: model })
  message.success(t('更新配置成功！'))
}
function cancel() {
  store.hide()
}
</script>

<template>
  <n-form
    ref="formRef"
    :model="model"
    class="form"
    label-placement="left"
    label-width="auto"
    require-mark-placement="right-hanging"
  >
    <n-form-item :label="$t('使用代理')" path="useProxy">
      <n-switch v-model:value="model.useProxy" />
    </n-form-item>

    <n-form-item :label="$t('协议')" path="origin">
      <n-select
        v-model:value="model.origin"
        :disabled="!model.useProxy"
        :placeholder="$t('选择协议')"
        :options="origins"
        value-field="key"
      />
    </n-form-item>

    <n-form-item :label="$t('主机')" path="host">
      <n-input
        v-model:value="model.host"
        :disabled="!model.useProxy"
        :placeholder="$t('输入主机地址')"
      />
    </n-form-item>

    <n-form-item :label="$t('port')" path="port">
      <n-input
        v-model:value="model.port"
        :disabled="!model.useProxy"
        :placeholder="$t('请输入端口')"
      />
    </n-form-item>
  </n-form>
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
