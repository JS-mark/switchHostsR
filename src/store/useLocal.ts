import type { RouteRecordRaw } from 'vue-router'
import type { CreateLocalHostsData } from '@/types'

import { defineStore } from 'pinia'
import { globalEventEmitter } from '@/utils'
import { setHistory } from '@/router/history'

export const useLocalStore = defineStore('local', {
  state: () => ({
    name: 'Mark',
    hosts: new Map(),
  }),
  getters: {},
  actions: {
    setName(name: string) {
      if (!name)
        return
      this.name = name
    },
    /**
     * 创建
     * @param space
     * @param data
     * @returns { void }
     */
    create(
      space: string,
      data: CreateLocalHostsData,
      cb?: (route?: RouteRecordRaw) => void,
    ) {
      if (!space || this.hasHosts(this.getSpace(space))) {
        cb && cb()
        return
      }
      /**
       * Notice: 未重写类型，使用删除 `redirect` 达到目的
       */
      // @ts-expect-error
      const route: RouteRecordRaw = {
        ...data,
        component: () => import('@/pages/edit/index.vue'),
      }
      this.hosts.set(this.getSpace(space), route)
      // 插入数据
      setHistory(space, data)
      // 触发事件更新
      globalEventEmitter.emit('onAddRoute')
      cb && cb(route)
    },
    restore(list: { space: string, data: RouteRecordRaw }[]) {
      const arr: RouteRecordRaw[] = []
      list.forEach((item) => {
        const { space, data } = item
        if (!space || this.hasHosts(this.getSpace(space)))
          return

        /**
         * Notice: 未重写类型，使用删除 `redirect` 达到目的
         */
        // @ts-expect-error
        const route: RouteRecordRaw = {
          ...data,
          component: () => import('@/pages/edit/index.vue'),
        }
        this.hosts.set(this.getSpace(space), route)
        // 触发事件更新
        arr.push(route)
      })
      globalEventEmitter.emit('onRestoreHistoryRoute', arr)
    },
    clear(space: string) {
      if (!space)
        return
      this.hosts.delete(this.getSpace(space))
    },
    destroy() {
      this.hosts = new Map()
      globalEventEmitter.off('onAddRoute')
    },
    getSpace(space: string) {
      return `${this.name}__${space}`
    },
    hasHosts(space: string): RouteRecordRaw | undefined {
      if (!space)
        return
      return this.hosts.get(`${this.name}__${space}`)
    },
  },
  persist: {
    enabled: true, // true 表示开启持久化保存
  },
})
