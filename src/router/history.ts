import type { RouteRecordRaw } from 'vue-router'
import { useLocalStore } from '@/store'

const HISTORY_NAME = '__LocalHostsLists'

export function HistoryMenus() {
  const { restore } = useLocalStore()
  const data = localStorage.getItem(HISTORY_NAME)
  if (!data)
    return
  const list = JSON.parse(data)
  restore(list)
}

/**
 * 设置历史
 * @param space
 * @param data
 */
export function setHistory(space: string, data: RouteRecordRaw) {
  const listData = localStorage.getItem(HISTORY_NAME)
  let list: { space: string, data: RouteRecordRaw }[] = []
  if (listData)
    list = JSON.parse(listData)

  list.push({
    space,
    data,
  })
  localStorage.setItem(HISTORY_NAME, JSON.stringify(list))
}
