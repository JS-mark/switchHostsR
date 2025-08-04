import { createApp } from 'vue'

import i18n from './langs'
import App from './App.vue'
import { usePlugins } from './plugins'
import './style.less'

import 'vfonts/Lato.css'
import 'virtual:uno.css'
import '@unocss/reset/normalize.css'
import 'vfonts/FiraCode.css'
import 'virtual:svg-icons-register'

createApp(App)
  .use(usePlugins)
  .use(i18n)
  .mount('#app')
