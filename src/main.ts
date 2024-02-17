import { createApp } from 'vue'
import i18n from './langs'
import { useRoutes } from './router'
import { useGlobalComponents } from './plugins/global-comp'
import { useFeature } from './plugins/use-feature'
import { usePinia } from './plugins/usePinia'

// 通用字体
import 'vfonts/Lato.css'

// 等宽字体
import 'vfonts/FiraCode.css'
import './style.less'
import 'virtual:svg-icons-register'
import 'virtual:windi.css'
import App from './App.vue'

createApp(App)
  .use(i18n)
  .use(usePinia)
  .use(useGlobalComponents)
  .use(useRoutes)
  .use(useFeature)
  .mount('#app')
