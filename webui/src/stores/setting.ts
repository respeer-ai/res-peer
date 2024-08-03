import { defineStore } from 'pinia'

export const useSettingStore = defineStore('setting', {
  state: () => ({
    cheCkoConnect: true,
    searchTextFilter: '',
    currentTab: 'feed'
  }),
  getters: {},
  actions: {}
})
