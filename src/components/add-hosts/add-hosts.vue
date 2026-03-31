<script lang="ts" setup>
import type { FormInst, FormItemRule } from 'naive-ui'

import { useI18n } from 'vue-i18n'
import { storeToRefs } from 'pinia'
import { useMessage } from 'naive-ui'
import {
  computed,
  onMounted,
  reactive,
  ref,
  watch,
} from 'vue'

import { createHost } from '@/apis'
import { useHostsStore } from '@/store/index'
import { globalEventEmitter } from '@/utils/event'

defineOptions({
  name: 'AddHostsDrawer',
})

const emit = defineEmits<{
  /** 创建成功后刷新列表 */
  (e: 'created'): void
}>()

const { t } = useI18n()
const message = useMessage()
const store = useHostsStore()
const { isShowEditor, mode } = storeToRefs(store)
const formRef = ref<FormInst | null>(null)
const isSubmitting = ref(false)

const data = reactive<{
  isShow: boolean
  model: {
    title: string
    description: string
    content: string
    hostsType: string | null
    url: string
    refreshType: number | null
  }
}>({
  isShow: false,
  model: {
    title: '',
    description: '',
    content: '',
    hostsType: null,
    url: '',
    refreshType: null,
  },
})

const isShowRemote = computed(() => {
  return data.model.hostsType === 'remote'
})

const rules = computed(() => {
  const rule: Record<string, FormItemRule | FormItemRule[]> = {
    title: {
      required: true,
      trigger: ['blur', 'input'],
      message: t('请输入名称'),
    },
    hostsType: {
      required: true,
      trigger: ['blur', 'change'],
      message: t('请选择 Hosts 类型'),
    },
  }
  if (data.model.hostsType === 'remote') {
    rule.url = {
      required: true,
      trigger: ['blur', 'input'],
      message: t('请输入远程 Url 地址'),
      validator(_rule: FormItemRule, value: string) {
        const reg = /(?:http|https):\/\/[\w.]\S*/
        if (!value)
          return new Error(t('请输入远程 Url 地址'))
        return reg.test(value)
          ? true
          : new Error(t('请输入正确的远程 Url 地址'))
      },
    }
    rule.refreshType = {
      required: true,
      trigger: ['blur', 'change'],
      message: t('请选择刷新类型'),
      validator(_rule: FormItemRule, value: number | null) {
        if (value === null || value === undefined)
          return new Error(t('请选择刷新类型'))
        return true
      },
    }
  }
  return rule
})

const title = computed(() => {
  return mode.value === 'create' ? t('创建 Hosts') : t('编辑 Hosts')
})

const refreshTypeOptions = computed(() => [
  { label: t('never'), value: 0 },
  { label: t('分钟', { m: 1 }), value: 1 * 60 * 1000 },
  { label: t('分钟', { m: 5 }), value: 5 * 60 * 1000 },
  { label: t('分钟', { m: 15 }), value: 15 * 60 * 1000 },
  { label: t('小时', { s: 1 }), value: 60 * 3600 * 1000 },
  { label: t('小时', { s: 24 }), value: 24 * 60 * 3600 * 1000 },
  { label: t('天', { d: 7 }), value: 7 * 24 * 60 * 3600 * 1000 },
])

const hostsTypeOptions = computed(() => [
  { label: t('local'), value: 'local' },
  { label: t('remote'), value: 'remote' },
])

function onClose() {
  formRef.value?.restoreValidation()
  data.model = {
    title: '',
    description: '',
    content: '',
    hostsType: null,
    url: '',
    refreshType: null,
  }
  store.hide()
}

function onSubmit(event: MouseEvent) {
  event.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      isSubmitting.value = true

      // 构建初始内容
      let initialContent = data.model.content || ''
      if (data.model.hostsType === 'remote' && data.model.url) {
        initialContent = `# 远程 Hosts: ${data.model.url}\n# 刷新类型: ${data.model.refreshType}\n${initialContent}`
      }

      createHost({
        name: data.model.title,
        content: initialContent,
        description: data.model.description || undefined,
        is_active: false,
      }).then((res) => {
        if (res.code === 200) {
          message.success(t('创建成功'))
          globalEventEmitter.emit('hosts-created')
          onClose()
        }
        else {
          message.error(res.msg || t('创建失败'))
        }
      }).catch((err) => {
        message.error(err.msg || t('创建失败'))
      }).finally(() => {
        isSubmitting.value = false
      })
    }
    else {
      message.error(t('请检查表单'))
    }
  })
}

function onCancel() {
  onClose()
}

onMounted(() => {
  watch(
    () => isShowEditor.value,
    () => {
      data.isShow = isShowEditor.value
    },
    { immediate: true },
  )
})
</script>

<template>
  <n-drawer
    v-model:show="data.isShow"
    placement="right"
    :width="502"
    :on-after-leave="onClose"
  >
    <n-drawer-content>
      <template #header>
        {{ title }}
      </template>
      <!-- 表单 -->
      <n-form
        ref="formRef"
        :model="data.model"
        :rules="rules"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
        size="medium"
      >
        <!-- hosts 类型 -->
        <n-form-item :label="$t('Hosts 类型')" path="hostsType">
          <n-select
            v-model:value="data.model.hostsType"
            :placeholder="$t('请选择 Hosts 类型')"
            :options="hostsTypeOptions"
          />
        </n-form-item>

        <!-- 名称 -->
        <n-form-item :label="$t('名称')" path="title">
          <n-input
            v-model:value="data.model.title"
            :placeholder="$t('请输入名称')"
          />
        </n-form-item>

        <!-- 描述 -->
        <n-form-item :label="$t('描述')">
          <n-input
            v-model:value="data.model.description"
            type="textarea"
            :placeholder="$t('可选，输入描述信息')"
            :rows="2"
          />
        </n-form-item>

        <!-- url -->
        <n-form-item v-show="isShowRemote" :label="$t('远程地址')" path="url">
          <n-input
            v-model:value="data.model.url"
            :placeholder="$t('请输入远程地址')"
          />
        </n-form-item>

        <!-- refresh 类型 -->
        <n-form-item
          v-show="isShowRemote"
          path="refreshType"
          :label="$t('刷新类型')"
        >
          <n-select
            v-model:value="data.model.refreshType"
            :placeholder="$t('请选择刷新类型')"
            :options="refreshTypeOptions"
          />
        </n-form-item>
      </n-form>

      <template #footer>
        <n-button round size="small" type="warning" @click="onCancel">
          {{ $t("cancel") }}
        </n-button>
        <n-button round size="small" type="primary" :loading="isSubmitting" @click="onSubmit">
          {{ $t("confirm") }}
        </n-button>
      </template>
    </n-drawer-content>
  </n-drawer>
</template>
