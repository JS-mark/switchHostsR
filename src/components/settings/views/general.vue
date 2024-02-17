<script lang="ts">
/**
 * 通用设置
 */
export default {
  name: 'General',
}
</script>

<script lang="ts" setup>
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { computed, onBeforeMount, reactive, ref } from 'vue'
import { setDefaultLang, useLanguages } from '@/langs'
import type { SettingSpace } from '@/store/useSettings'
import { useSettingsStore } from '@/store/useSettings'

const { t } = useI18n()
const store = useSettingsStore()
const message = useMessage()
const formRef = ref(null)
const model = reactive<SettingSpace.General>({
  theme: 'auto',
  language: 'zh-cn',
  writeMode: '1', // 追加 1，覆盖 2
  selectedMode: '2', // 单选 1，多选 2
  palletTitle: false, // 托盘标题
  hideAtStartup: false, // 托盘标题
})

onBeforeMount(() => {
  for (const [key, value] of Object.entries(store.general))
    Reflect.set(model, key, value)
})

const themes = [
  { label: t('dark'), key: 'black' },
  { label: t('light'), key: 'light' },
  { label: t('systemTheme'), key: 'auto' },
]
const languages = useLanguages()

const writeModeTip = computed(() => {
  return model.writeMode === '1'
    ? t('新记录将追加到现有系统 Hosts 的文件末尾')
    : t('新记录将覆盖现有系统 Hosts 的文件内容')
})

function confirm() {
  cancel()
  if (store.general.language !== model.language)
    setDefaultLang(model.language)
  store.setSettingsByData({ general: model })
  message.success(t('更新配置成功！'))
}
function cancel() {
  store.hide()
}
</script>

<template>
  <n-form
    ref="formRef"
    class="form"
    :model="model"
    label-placement="left"
    label-width="auto"
    require-mark-placement="right-hanging"
  >
    <n-form-item :label="$t('language')" path="language">
      <n-select
        v-model:value="model.language"
        :placeholder="$t('选择语言')"
        :options="languages"
        value-field="key"
      />
    </n-form-item>

    <n-form-item :label="$t('theme')" path="theme">
      <n-select
        v-model:value="model.theme"
        :placeholder="$t('选择主题色')"
        :options="themes"
        value-field="key"
      />
    </n-form-item>

    <n-form-item :label="$t('写入模式')" path="writeMode">
      <section class="column">
        <n-radio-group v-model:value="model.writeMode" name="writeMode">
          <n-space>
            <n-radio value="1">
              {{ $t("append") }}
            </n-radio>
            <n-radio value="2">
              {{ $t("overlay") }}
            </n-radio>
          </n-space>
        </n-radio-group>
        <span class="tip">{{ writeModeTip }}</span>
      </section>
    </n-form-item>

    <n-form-item :label="$t('选择模式')" path="selectedMode">
      <section class="column">
        <n-radio-group v-model:value="model.selectedMode" name="selectedMode">
          <n-space>
            <n-radio value="1">
              {{ $t("radio") }}
            </n-radio>
            <n-radio value="2">
              {{ $t("multiple") }}
            </n-radio>
          </n-space>
        </n-radio-group>
        <span class="tip">
          {{ $t("只对顶层项目生效，每个文件夹可设置自己的选择模式。") }}
        </span>
      </section>
    </n-form-item>

    <n-form-item :label="$t('显示托盘标题')" path="palletTitle">
      <n-switch v-model:value="model.palletTitle" />
    </n-form-item>

    <n-form-item :label="$t('启动时隐藏')" path="hideAtStartup">
      <n-switch v-model:value="model.hideAtStartup" />
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
