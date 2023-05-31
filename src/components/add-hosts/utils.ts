import { VNode } from "vue";
import { nanoid } from "nanoid";
import { debounce } from "lodash-es";
import { useLocalStore } from "@/store";
import { SettingsSharp } from "@vicons/ionicons5";
import { type RouteRecordRaw } from "vue-router";

export interface Options {
  name: string;
  title: string;
  type: "remote" | "local";
  icon?: VNode;
  id?: string;
}

/**
 * 添加类型
 * @param id 特定 id
 * @param icon menu icon
 * @param title 汉字 label
 * @param name 大写 route Name
 */
export const addLocalHostsRoute = debounce(
  (
    { type, id, name, icon, title }: Options,
    cb: (data?: RouteRecordRaw) => void,
  ) => {
    const { hasHosts, create } = useLocalStore();
    const id_ = id || nanoid();
    const space = `${type}__${id_}`;
    if (!hasHosts(space)) {
      create(
        space,
        {
          path: "/edit",
          name,
          meta: {
            icon: icon || SettingsSharp,
            title,
            hidden: false,
          },
          redirect: "",
        },
        cb,
      );
    } else {
      cb();
    }
  },
  300,
);
