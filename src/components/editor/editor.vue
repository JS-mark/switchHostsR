<script lang="ts">
export default {
  name: 'Editor',
}
</script>

<script lang="ts" setup>
import useMonaco from './monaco'
import { useLoadingBar } from 'naive-ui'
import { useSettingsStore } from '@/store'
import { onMounted, onUnmounted, reactive, watchEffect } from 'vue'

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
    language: 'hosts',
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
}>()

const appSettings = useSettingsStore()

const loadingBar = useLoadingBar()

const data = reactive({
  value: '',
  origin: '',
})

const { updateVal, useEditor, switchTheme, destroy, createEditor, onFormatDoc } = useMonaco(
  props.language,
)

function initEditor() {
  const el = document.querySelector('#container')

  if (el)
    createEditor(el as HTMLElement, props.options)
  else
    destroy()
}

function getValue(): Promise<{ [key: string]: unknown }> {
  return new Promise((resolve, reject) => {
    loadingBar.start()
    useEditor((editor) => {
      try {
        loadingBar.finish()
        resolve(JSON.parse(editor?.getValue()))
      }
      catch (error) {
        loadingBar.error()
        reject(error)
      }
    })
  })
}

function initEditorEvent() {
  useEditor((editor) => {
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

defineExpose({
  useEditor,
  getValue,
  onFormatDoc,
})

watchEffect(() => {
  data.value = props.modelValue
  switchTheme(appSettings.theme)
})

onMounted(() => {
  loadingBar.start()
  setTimeout(() => {
    initEditor()
    initEditorEvent()
    // 保留原始数据
    data.origin = props.modelValue
    updateMonacoVal(props.modelValue, props.format)
    loadingBar.finish()
  }, 100)
})

onUnmounted(() => {
  destroy()
})
</script>

<template>
  <div class="editor__box">
    <div id="container" class="editor__box__container" />
  </div>
</template>

<style lang="stylus" scoped>
.editor__box
  width 100%
  height 100%

  &__container
    width 100%
    height 100%
</style>
