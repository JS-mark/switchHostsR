import i18n from './langs'
import App from './App.vue'
import { createApp } from 'vue'
import { useRoutes } from './router'
import { usePinia } from './plugins/usePinia'
import { useFeature } from './plugins/use-feature'
import { useGlobalComponents } from './plugins/global-comp'


import './style.less'
import 'vfonts/Lato.css'
import 'virtual:windi.css'
import 'vfonts/FiraCode.css'
import 'virtual:svg-icons-register'

createApp(App)
  .use(i18n)
  .use(usePinia)
  .use(useGlobalComponents)
  .use(useRoutes)
  .use(useFeature)
  .mount('#app')
