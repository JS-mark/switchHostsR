import routes from "./routes";
import type { App } from "vue";
import { useUserStore } from "@/store";
import { createRouter, createWebHashHistory } from "vue-router";

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

router.beforeEach((to, from) => {
  const store = useUserStore();
  if (to.name === "Login" || from.name === "Login") {
    return true;
  }
  if (!store.isLogin) {
    to.name !== "Login" &&
      from.name !== "Login" &&
      router.replace({
        name: store.isLogin ? (to.name as string) : "Login",
      });
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
