// eslint.config.js
import tm2js from '@tm2js/eslint-config'

export default await tm2js(
  {
    // Enable stylistic formatting rules
    // stylistic: true,

    // Or customize the stylistic rules
    stylistic: {
      indent: 2, // 4, or 'tab'
      quotes: 'single', // or 'double'
    },

    // TypeScript and Vue are auto-detected, you can also explicitly enable them:
    typescript: true,
    // 开启 vue
    vue: true,
    jsonc: true,
    yaml: true,

    // `.eslintignore` is no longer supported in Flat config, use `ignores` instead
    ignores: [
      './src-tauri',
      // ...globs
    ],
  },
  {
    // Remember to specify the file glob here, otherwise it might cause the vue plugin to handle non-vue files
    files: ['**/*.vue'],
    rules: {
      'vue/operator-linebreak': ['error', 'before'],
    },
  },
  {
    // Remember to specify the file glob here, otherwise it might cause the vue plugin to handle non-vue files
    files: ['**/*.ts'],
    rules: {
      'ts/no-unsafe-argument': 'off',
      'ts/no-namespace': 'off',
    },
  },
  {
    // Without `files`, they are general rules for all files
    rules: {
      'import/order': 'off',
      'style/semi': ['error', 'never'],
      'unicorn/no-new-array': 'off',
      'prefer-promise-reject-errors': 'off',
      'import/first': 'off',
      'ts/ban-ts-comment': 'off',
      'no-restricted-syntax': 'off',
      'no-console': 'off',
      'tm2js/top-level-function': 'off',
    },
  },
)
