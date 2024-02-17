import { defineStore } from 'pinia'

export interface Feature {
  key: string
  enable: boolean
}

export const useFeatureStore = defineStore('feature', {
  state: () => ({
    mapData: new Map<string, boolean>(),
  }),
  actions: {
    init(list: Feature[]) {
      if (!Array.isArray(list))
        return
      list.forEach((item) => {
        this.setFeature(item.key, item.enable)
      })
    },
    setFeature(feature: string, enable: boolean) {
      if (!feature)
        return
      this.mapData.set(feature, enable)
    },
    getFeatureStatus(feature: string) {
      if (!feature)
        return
      return this.mapData.get(feature) || false
    },
  },
})
