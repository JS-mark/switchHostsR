import type { VNode } from 'vue'
import { nanoid } from 'nanoid'
import { debounce } from 'lodash-es'
import { SettingsSharp } from '@vicons/ionicons5'
import type { RouteRecordRaw } from 'vue-router'
import { useLocalStore } from '@/store'

export interface Options {
  name: string
  title: string
  content?: string
  type: 'remote' | 'local'
  icon?: VNode
  id?: string
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
    { type, id, name, icon, title, content }: Options,
    cb: (data?: RouteRecordRaw) => void,
  ) => {
    const { hasHosts, create } = useLocalStore()
    const id_ = id || nanoid()
    const space = `${type}__${id_}`
    if (!hasHosts(space)) {
      create(
        space,
        {
          name,
          ext: {
            content: content || "",
            icon: icon || SettingsSharp,
            title,
          },
        },
        cb,
      )
    }
    else {
      cb()
    }
  },
  300,
)
