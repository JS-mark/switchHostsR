<script lang="ts" setup>
import type { editor } from 'monaco-editor/esm/vs/editor/editor.api'

import { useSettingsStore } from '@/store'
import { useNaiveApi } from '@/plugins/naive-api'
import { onMounted, onUnmounted, reactive } from 'vue'

import useMonaco from './monaco'
import { useEditor } from './hook'

defineOptions({
  name: 'Editor',
})

const props = withDefaults(
  defineProps<{
    modelValue: string
    language?: string
    format?: boolean
    preComment?: string
    options?: { [key: string]: any }
  }>(),
  {
    modelValue: '',
    language: 'shell',
    preComment: '',
    format: true,
    options: () => ({}),
  },
)

const emits = defineEmits<{
  (event: 'update:modelValue', value: string): void
  (event: 'onChange', value: {
    originValue: string
    newValue: string
  }): void
  (event: 'focus'): void
  (event: 'blur'): void
  (event: 'ready', value: editor.IStandaloneCodeEditor): void
}>()
const appSettings = useSettingsStore()
const { loadingBar, notification } = useNaiveApi()

const data = reactive({
  value: '',
  origin: '',
  isLoading: false,
  containerId: '',
  isInitError: false,
})

const {
  id,
  updateVal,
  switchTheme,
  destroy,
  createEditor,
  onFormatDoc,
} = useMonaco(
  props.language,
)

data.containerId = id

function initEditor() {
  const el = document.querySelector(`#container__${id}`)
  if (el) {
    createEditor(el as HTMLElement, props.options, (editor) => {
      emits('ready', editor)
      data.isLoading = false
    }).catch(() => {
      data.isInitError = true
      notification.error({
        title: '系统提示',
        content: '初始化失败, 请检查浏览器是否支持Monaco Editor',
      })
    })
  }
  else {
    destroy(id)
  }
}

function getValue(): Promise<string | Record<string, any>> {
  return new Promise((resolve, reject) => {
    loadingBar.start()
    useEditor(id, (editor) => {
      try {
        loadingBar.finish()
        resolve(editor?.getValue())
      }
      catch (error) {
        loadingBar.error()
        reject(error)
      }
    })
  })
}

function initEditorEvent() {
  useEditor(id, (editor) => {
    editor.onDidFocusEditorText(() => {
      emits('focus')
    })
    editor.onDidBlurEditorText(() => {
      emits('blur')
    })
    editor.onDidChangeModelContent(() => {
      data.value = editor.getValue()
      emits('update:modelValue', data.value)
      // 对外响应事件
      data.value !== data.origin && emits('onChange', {
        originValue: data.origin,
        newValue: data.value,
      })
    })
  })
}

function updateMonacoVal(_val?: string, format?: boolean) {
  const { modelValue, preComment } = props
  const val = preComment
    ? `${preComment}\n${_val || modelValue}`
    : _val || modelValue
  updateVal(val, format)
}

function setValue(value: string) {
  data.value = value
  updateMonacoVal(value, props.format)
}

function init() {
  data.isLoading = true
  loadingBar.start()
  setTimeout(() => {
    initEditor()
    initEditorEvent()
    // 保留原始数据
    data.origin = props.modelValue
    switchTheme(appSettings.theme)
    loadingBar.finish()
  }, 100)
}

function onRetry() {
  data.isInitError = false
  initEditor()
  // 更新值
  updateMonacoVal(data.value, props.format)
  // 重新初始化
  init()
}

defineExpose({
  getId() {
    return id
  },
  useEditor,
  getValue,
  setValue,
  onFormatDoc,
  destroy,
})

onMounted(() => {
  init()
})

onUnmounted(() => {
  destroy(id)
})
</script>

<template>
  <div class="editor__box relative">
    <div v-if="data.isLoading" class="absolute left-1/2 top-1/2 flex justify-center items-center">
      <n-spin :show="data.isLoading" :size="23">
        <template #description>
          加载中...
        </template>
      </n-spin>
    </div>
    <div v-show="!data.isInitError" :id="`container__${data.containerId}`" class="editor__box__container" />
    <n-result
      v-if="data.isInitError" class="h-500px flex justify-center flex-col" status="404" title="资源不存在"
      description=""
    >
      <template #footer>
        <n-button @click="onRetry">
          点击重试
        </n-button>
      </template>
    </n-result>
  </div>
</template>

<style lang="less" scoped>
.editor__box {
  width: 100%;
  height: 100%;

  &__container {
    width: 100%;
    height: 100%;
  }
}
</style>
