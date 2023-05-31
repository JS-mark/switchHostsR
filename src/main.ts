import i18n from "./langs";
import { createApp } from "vue";
import { createPinia } from "pinia";
import { useRoutes } from "./router";
import { useGlobalComp } from "./plugins/global-comp";
// 通用字体
import "vfonts/Lato.css";
// 等宽字体
import "vfonts/FiraCode.css";
import "./style.css";
import "virtual:svg-icons-register";
import App from "./App.vue";

createApp(App)
  .use(i18n)
  .use(createPinia())
  .use(useGlobalComp)
  .use(useRoutes)
  .mount("#app");
