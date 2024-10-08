import { defineStore } from 'pinia'

export const useApplicationStore = defineStore('application', {
  state: () => ({
    feedApp: undefined as unknown as string,
    creditApp: undefined as unknown as string,
    marketApp: undefined as unknown as string,
    reviewApp: undefined as unknown as string,
    foundationApp: undefined as unknown as string,
    activityApp: undefined as unknown as string,
    blobGatewayApp: undefined as unknown as string,
    cpRegistryApp: undefined as unknown as string,
    copilotCpuApp: undefined as unknown as string,
    copilotGpuApp: undefined as unknown as string,
    illustratorCpuApp: undefined as unknown as string,
    illustratorGpuApp: undefined as unknown as string
  }),
  getters: {},
  actions: {}
})
