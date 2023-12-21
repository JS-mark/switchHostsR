import { resolve } from 'node:path'
import process from 'node:process'
import vue from '@vitejs/plugin-vue'
import type { ConfigEnv } from 'vite'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import monacoEditorPlugin from 'vite-plugin-monaco-editor'
import { createSvgIconsPlugin } from 'vite-plugin-svg-icons'
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'

export function getBaseConfig(env: ConfigEnv) {
  // 打印环境变量
  console.warn('[ env ]', env)
  return {
    plugins: [
      createSvgIconsPlugin({
        // 指定需要缓存的图标文件夹
        iconDirs: [resolve(process.cwd(), 'src/assets/icons')],
        // 指定symbolId格式
        symbolId: 'icon-[dir]-[name]',

        /**
         * 自定义插入位置
         * @default: body-last
         */
        inject: 'body-last',

        /**
         * custom dom id
         * @default: __svg__icons__dom__
         */
        customDomId: '__svg__icons__dom__',
      }),
      vue(),
      AutoImport({
        imports: [
          'vue',
          {
            'naive-ui': [
              'useDialog',
              'useMessage',
              'useNotification',
              'useLoadingBar',
            ],
          },
        ],
      }),
      Components({
        resolvers: [NaiveUiResolver()],
      }),
      // eslint-disable-next-line ts/ban-ts-comment
      // @ts-expect-error
      monacoEditorPlugin.default({}),
    ],
    resolve: {
      alias: {
        '@': resolve(__dirname, '../src'),
        '@utils': resolve(__dirname, '../src/utils'),
      },
    },

    // to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ['VITE_', 'TAURI_'],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // prevent vite from obscuring rust errors
    clearScreen: false,
    // tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1420,
      strictPort: true,
      proxy: {
        'https://weibo.com/ajax/profile/info': {
          target: 'https://weibo.com', // 需要代理的地址
          changeOrigin: true, // 是否跨域
          headers: {
            // header设置
            referer: 'https://weibo.com/', // referer
            origin: 'https://weibo.com/',
          },
        },
      },
    },
    build: {
      // Tauri supports es2021
      target:
        process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
      // don't minify for debug builds
      minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
      // produce sourcemaps for debug builds
      sourcemap: !!process.env.TAURI_DEBUG,
      rollupOptions: {
        output: {
          chunkFileNames: 'js/[name]-[hash].js',
          entryFileNames: 'js/[name]-[hash].js',
          assetFileNames: '[ext]/[name]-[hash].[ext]',
          manualChunks(id) {
            if (id.includes('node_modules')) {
              return id
                .toString()
                .split('node_modules/')[1]
                .split('/')[0]
                .toString()
            }
          },
        },
      },
    },
  }
}
