import { defineStore } from "pinia";
import emitter from "@/plugins/emitter";
import { setHistory } from "@/router/history";
import { type RouteRecordRaw } from "vue-router";

export const useLocalStore = defineStore("local", {
  state: () => ({
    name: "Mark",
    hosts: new Map(),
  }),
  getters: {},
  actions: {
    setName(name: string) {
      if (!name) return;
      this.name = name;
    },
    /**
     * 创建
     * @param space
     * @param data
     * @returns
     */
    create(
      space: string,
      data: RouteRecordRaw,
      cb?: (route?: RouteRecordRaw) => void,
    ) {
      if (!space || this.hasHosts(this.getSpace(space))) {
        cb && cb();
        return;
      }
      delete data.redirect;
      /**
       * Notice: 未重写类型，使用删除 `redirect` 达到目的
       */
      // @ts-ignore
      const route: RouteRecordRaw = {
        ...data,
        component: () => import("@/pages/edit/index.vue"),
      };
      this.hosts.set(this.getSpace(space), route);
      setHistory(space, route);
      // 触发事件更新
      emitter.emit("onAddRoute");
      cb && cb(route);
    },
    restore(list: { space: string; data: RouteRecordRaw }[]) {
      const arr = list.map((item) => {
        const { space, data } = item;
        if (!space || this.hasHosts(this.getSpace(space))) {
          return;
        }
        /**
         * Notice: 未重写类型，使用删除 `redirect` 达到目的
         */
        // @ts-ignore
        const route: RouteRecordRaw = {
          ...data,
          component: () => import("@/pages/edit/index.vue"),
        };
        this.hosts.set(this.getSpace(space), route);
        // 触发事件更新
        return route;
      });
      emitter.emit("onRestoreHistoryRoute", arr as RouteRecordRaw[]);
    },
    clear(space: string) {
      if (!space) return;
      this.hosts.delete(this.getSpace(space));
    },
    destroy() {
      this.hosts = new Map();
      emitter.off("onAddRoute");
    },
    getSpace(space: string) {
      return `${this.name}__${space}`;
    },
    hasHosts(space: string): RouteRecordRaw | undefined {
      if (!space) return;
      return this.hosts.get(`${this.name}__${space}`);
    },
  },
});
