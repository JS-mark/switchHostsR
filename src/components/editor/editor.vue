<script lang="ts" setup>
import useMonaco from './monaco'
import { useLoadingBar } from 'naive-ui'
import { useSettingsStore } from '@/store'
import { onMounted, onUnmounted, reactive } from 'vue'

defineOptions({
  name: 'Editor'
})

const props = withDefaults(
  defineProps<{
    id: string
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
}>()

const appSettings = useSettingsStore()

const loadingBar = useLoadingBar()

const data = reactive({
  value: '',
  origin: '',
})

const {
  updateVal,
  useEditor,
  switchTheme,
  destroy,
  createEditor,
  onFormatDoc
} = useMonaco(
  props.language,
)

function initEditor() {
  const el = document.querySelector(`#${props.id}`)

  if (el)
    createEditor(props.id, el as HTMLElement, props.options)
}

function getValue(): Promise<string | Record<string, any>> {
  return new Promise((resolve, reject) => {
    loadingBar.start()
    useEditor(props.id, (editor) => {
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
  useEditor(props.id, (editor) => {
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

/**
 * 更新值
 * @param _val
 * @param format
 */
const updateMonacoVal = (_val?: string, format?: boolean) =>{
  const { modelValue, preComment } = props
  const val = preComment
    ? `${preComment}\n${_val || modelValue}`
    : _val || modelValue
  updateVal(props.id, val, format)
}



/**
 * 更新值
 * @param value
 */
const setValue = (value: string) => {
  data.value = value
  updateMonacoVal(value, props.format)
}

defineExpose({
  useEditor,
  getValue,
  setValue,
  onFormatDoc,
})


onMounted(() => {
  loadingBar.start()
  setTimeout(() => {
    initEditor()
    initEditorEvent()
    // 保留原始数据
    data.origin = props.modelValue
    switchTheme(props.id, appSettings.theme)
    loadingBar.finish()
  }, 100)
})

onUnmounted(() => {
  destroy(props.id)
})
</script>

<template>
  <div class="editor__box">
    <div :id="id" class="editor__box__container" />
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
