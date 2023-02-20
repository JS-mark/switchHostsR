import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { useRouter } from './router'

import { useGlobalComp } from './plugins/global-comp'
// 通用字体
import 'vfonts/Lato.css'
// 等宽字体
import 'vfonts/FiraCode.css'
import './style.css'
import 'virtual:svg-icons-register'
import App from './App.vue'

createApp(App)
  .use(createPinia())
  .use(useGlobalComp)
  .use(useRouter)
  .mount('#app')
