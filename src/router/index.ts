import routes from "./routes";
import type { App } from "vue";
import { useUserStore } from "@/store";
import { createRouter, createWebHashHistory } from "vue-router";
import { getLoginUser } from "@/utils";

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

router.beforeEach((to, from) => {
  const store = useUserStore();
  const data = getLoginUser();
  if (data) {
    store.setLogin(true);
    store.setMode(data.mode);
    store.setUserInfo(data.info);
    return true;
  }
  if (!store.isLogin) {
    if (to.name !== "Login" || (to.name !== "Login" && from.name !== "Login")) {
      router.replace({
        name: store.isLogin ? (to.name as string) : "Login",
      });
    }
  }
  return true;
});

export const useRoutes = (app: App, callback?: (app: App) => void) => {
  app.use(router);

  if (callback instanceof Function) {
    router.isReady().finally(() => {
      callback(app);
    });
  }
};
