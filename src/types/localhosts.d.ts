import type { RouteMeta } from 'vue-router'

export interface CreateLocalHostsData {
  name: string
  ext: {
    content: string
    icon: string | RouteMeta['icon']
    title: string
  }
}
