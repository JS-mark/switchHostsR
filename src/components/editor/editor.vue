<template>
  <div class="editor__box">
    <div id="container" class="editor__box__container" />
  </div>
</template>

<script lang="ts">
export default {
  name: "Editor",
};
</script>
<script lang="ts" setup>
import useMonaco from "./monaco";
import { useLoadingBar } from "naive-ui";
import { watchEffect, reactive, onMounted, onUnmounted } from "vue";

const loadingBar = useLoadingBar();

const data = reactive({
  value: "",
});

const props = withDefaults(
  defineProps<{
    modelValue: string;
    language?: string;
    format?: boolean;
    preComment?: string;
    options?: { [key: string]: any };
  }>(),
  {
    modelValue: "",
    language: "shell",
    preComment: "",
    format: true,
    options: () => ({}),
  },
);

const { updateVal, useEditor, destroy, createEditor, onFormatDoc } = useMonaco(
  props.language,
);

const emits = defineEmits<{
  (event: "update:modelValue", value: string): void;
  (event: "focus"): void;
  (event: "blur"): void;
}>();

const initEditor = () => {
  const el = document.querySelector("#container");
  if (el) {
    createEditor(el as HTMLElement, props.options);
  } else {
    destroy();
  }
};

const getValue = (): Promise<{ [key: string]: unknown }> => {
  return new Promise((resolve, reject) => {
    loadingBar.start();
    useEditor((editor) => {
      try {
        loadingBar.finish();
        resolve(JSON.parse(editor?.getValue()));
      } catch (error) {
        loadingBar.error();
        reject(error);
      }
    });
  });
};

const initEditorEvent = () => {
  useEditor((editor) => {
    editor.onDidFocusEditorText(() => {
      emits("focus");
    });
    editor.onDidBlurEditorText(() => {
      emits("blur");
    });
    editor.onDidChangeModelContent(() => {
      data.value = editor.getValue();
      emits("update:modelValue", data.value);
    });
  });
};

const updateMonacoVal = (_val?: string, format?: boolean) => {
  const { modelValue, preComment } = props;
  const val = preComment
    ? `${preComment}\n${_val || modelValue}`
    : _val || modelValue;
  updateVal(val, format);
};

defineExpose({
  useEditor,
  getValue,
  onFormatDoc,
});

watchEffect(() => {
  data.value = props.modelValue;
});

onMounted(() => {
  loadingBar.start();
  setTimeout(() => {
    initEditor();
    initEditorEvent();
    updateMonacoVal(props.modelValue, props.format);
    loadingBar.finish();
  }, 100);
});

onUnmounted(() => {
  destroy();
});
</script>

<style lang="stylus" scoped>
.editor__box
  width 100%
  height 100%

  &__container
    width 100%
    height 100%
</style>
