import { useLocalStore } from '@/store'
import type { CreateLocalHostsData } from '@/types'

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
export function setHistory(space: string, data: CreateLocalHostsData) {
  const listData = localStorage.getItem(HISTORY_NAME)
  let list: { space: string, data: CreateLocalHostsData }[] = []
  if (listData)
    list = JSON.parse(listData)

  list.push({
    space,
    data,
  })
  localStorage.setItem(HISTORY_NAME, JSON.stringify(list))
}

/**
 * update历史
 * @param space
 * @param data
 */
export function updateHistory(space: string, data: CreateLocalHostsData) {
  const listData = localStorage.getItem(HISTORY_NAME)
  let list: { space: string, data: CreateLocalHostsData }[] = []
  if (listData)
    list = JSON.parse(listData)

  list.push({
    space,
    data,
  })
  localStorage.setItem(HISTORY_NAME, JSON.stringify(list))
}
