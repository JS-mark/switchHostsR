/**
 * host 文件类型语言支持
 * @description: 参考网络完成
 */

import * as monaco from 'monaco-editor'

const lang = 'hosts'
// 1. 注册自定义语言
monaco.languages.register({ id: lang })

// 2. 添加关键字高亮
monaco.languages.setMonarchTokensProvider(lang, {
  tokenizer: {
    root: [
      [/(\b25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(\b25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(\b25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(\b25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(:\d{1,5})?\b/, 'constant'],
      [/(localhost|broadcasthost)\b/, 'custom-hostname'],
      [/(\b(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z]{2,}\b)/, 'custom-domain'], // 高亮域名
      [/#[^\n]*/, 'comment'], // 示例：高亮显示注释
    ],
  },
})

// 3. 添加自动完成
monaco.languages.registerCompletionItemProvider(lang, {

  provideCompletionItems: (model, position): monaco.languages.ProviderResult<monaco.languages.CompletionList> => {
    const word = model.getWordUntilPosition(position)
    const range = {
      startLineNumber: position.lineNumber,
      endLineNumber: position.lineNumber,
      startColumn: word.startColumn,
      endColumn: word.endColumn,
    }

    return { suggestions: [
      {
        label: 'demo',
        insertText: '0.0.0.0 www.baidu.com',
        kind: monaco.languages.CompletionItemKind.Snippet,
        detail: '示例代码',
        range,
      },
    ] }
  },
})

// 暗夜模式
monaco.editor.defineTheme('hosts-dark', {
  // 基础
  base: 'vs-dark',
  // 继承
  inherit: true,
  colors: {},
  // 规则
  rules: [
    { token: 'constant', foreground: 'e06c75', fontStyle: 'bold' },
    { token: 'comment', foreground: '8cc265', fontStyle: 'italic' },
    { token: 'keyword', foreground: '569cd6' },
    // 其他自定义规则
    { token: 'custom-domain', foreground: '1cbae4', fontStyle: 'bold' },
    { token: 'custom-hostname', foreground: 'fe8d59', fontStyle: 'bold' },
  ],
})

// 白昼模式
monaco.editor.defineTheme('hosts', {
  // 基础
  base: 'vs',
  // 继承
  inherit: true,
  colors: {},
  // 规则
  rules: [
    { token: 'constant', foreground: 'e06c75', fontStyle: 'bold' },
    { token: 'comment', foreground: '5ba2dd', fontStyle: 'italic' },
    { token: 'keyword', foreground: '569cd6' },
    // 其他自定义规则
    { token: 'custom-domain', foreground: '1cbae4', fontStyle: 'bold' },
    { token: 'custom-hostname', foreground: 'fe8d59', fontStyle: 'bold' },
  ],
})
