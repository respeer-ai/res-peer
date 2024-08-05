import { defineStore } from 'pinia'

export const useSettingStore = defineStore('setting', {
  state: () => ({
    cheCkoConnect: true,
    searchTextFilter: '',
    currentMainTab: 'feed',
    currentDashboardTab: 'contents',
    showDrawerMenu: false
  }),
  getters: {},
  actions: {}
})
