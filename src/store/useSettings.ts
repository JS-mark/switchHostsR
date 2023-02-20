import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    name: 'Mark',
    logo: '',
  }),
  getters: {},
  actions: {
    setName(name: string) {
      if (!name) return
      this.name = name
    },
    setLogo(logo: string) {
      if (!logo) return
      this.logo = logo
    },
  },
})
