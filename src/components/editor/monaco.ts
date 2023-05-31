import "monaco-editor/esm/vs/editor/editor.all.js";
import "monaco-editor/esm/vs/editor/standalone/browser/accessibilityHelp/accessibilityHelp.js";
import "monaco-editor/esm/vs/basic-languages/monaco.contribution";
import "monaco-editor/esm/vs/language/json/monaco.contribution";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { useDark } from "@vueuse/core";
import emitter from "@/plugins/emitter";
export { monaco };

declare global {
  interface Window {
    __MonacoEditor: null | monaco.editor.IStandaloneCodeEditor;
  }
}

/**
 * hook 函数
 * @param language
 * @returns
 */
export default function useMonaco(language = "json") {
  const isDark = useDark({
    selector: "body",
    attribute: "color-scheme",
    valueDark: "vs-dark",
    valueLight: "vs",
  });
  let initReadOnly = false;

  const updateVal = async (val: string, format = true) => {
    useEditor((editor) => {
      editor?.setValue(val);
    });
    setTimeout(() => {
      useEditor(async (editor) => {
        initReadOnly && editor?.updateOptions({ readOnly: false });
        format &&
          (await editor?.getAction("editor.action.formatDocument")?.run());
        initReadOnly && editor?.updateOptions({ readOnly: true });
      });
    }, 100);
  };

  const useEditor = (
    cb: (editor: monaco.editor.IStandaloneCodeEditor) => void,
  ) => {
    if (window.__MonacoEditor) {
      cb(window.__MonacoEditor as monaco.editor.IStandaloneCodeEditor);
      return;
    }
    console.warn("editor not ready! 通过 cb 执行！");

    emitter.on("ready", (editor: monaco.editor.IStandaloneCodeEditor) => {
      cb(editor);
      emitter.off("ready");
    });
  };

  const createEditor = (
    el: HTMLElement | null,
    editorOption: monaco.editor.IStandaloneEditorConstructionOptions = {},
  ) => {
    if (window.__MonacoEditor) {
      return;
    }
    initReadOnly = !!editorOption.readOnly;
    const theme = isDark.value ? "vs-dark" : "vs";
    window.__MonacoEditor =
      el &&
      (monaco.editor.create(el, {
        language,
        minimap: { enabled: false },
        theme,
        lineNumbers: "on",
        multiCursorModifier: "ctrlCmd",
        scrollbar: {
          verticalScrollbarSize: 8,
          horizontalScrollbarSize: 8,
        },
        tabSize: 2,
        automaticLayout: true, // 自适应宽高
        ...editorOption,
      }) as unknown as monaco.editor.IStandaloneCodeEditor);
    emitter.emit("ready", window.__MonacoEditor);
    return window.__MonacoEditor;
  };

  const destroy = () => {
    useEditor((editor) => {
      editor.dispose();
      window.__MonacoEditor = null;
    });
  };

  const onFormatDoc = () => {
    useEditor((editor) => {
      editor?.getAction("editor.action.formatDocument")?.run();
    });
  };
  return {
    updateVal,
    useEditor,
    destroy,
    createEditor,
    onFormatDoc,
  };
}
