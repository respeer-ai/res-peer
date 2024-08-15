import { defineStore } from 'pinia'

enum ResourceType {
  CPU = 'CPU',
  GPU = 'GPU'
}

enum StorageType {
  NVME = 'NVME',
  SSD = 'SSD',
  HDD = 'HDD'
}

export enum TaskType {
  FixGrammar = 'FIX_GRAMMAR',
  RewriteEasierUnderstand = 'REWRITE_EASIER_UNDERSTAND',
  Paraphrase = 'PARAPHRASE',
  WriteFormally = 'WRITE_FORMALLY',
  WriteMoreNeutral = 'WRITE_MORE_NEUTRAL',
  GenerateIllustrate = 'GENERATE_ILLUSTRATE'
}

const taskTypes = new Map<TaskType, string>([
  [TaskType.FixGrammar, 'Fix the grammar'],
  [TaskType.RewriteEasierUnderstand, 'Rewrite to make this easier to understand'],
  [TaskType.Paraphrase, 'Paraphrase this'],
  [TaskType.WriteFormally, 'Write this more formally'],
  [TaskType.WriteMoreNeutral, 'Write in a more neutral way'],
  [TaskType.GenerateIllustrate, 'Generate illustrate']
])

export const taskTypeName = (taskType: TaskType) => {
  return taskTypes.get(taskType) || 'Unsupported task'
}

export const taskTypePrefix = (taskType: TaskType) => {
  if (taskType === TaskType.GenerateIllustrate) return ''
  return taskTypeName(taskType) + ': '
}

export interface CPNode {
  nodeId: string
  brandLogo: string
  brandName: string
  link: string
  applicationId: string
  resourceType: ResourceType
  deviceModel: string
  cpuModel: string
  storageType: StorageType,
  storageBytes: number
  memoryBytes: number
  freeQuota: number
  priceQuota: number
  quotaPrice: number
  supportedTaskTypes: TaskType[]
  paymentChainId: string
  available: boolean
  createdAt: number
}

export const useCPRegistryStore = defineStore('cpRegistry', {
  state: () => ({
    nodes: [] as CPNode[]
  }),
  getters: {},
  actions: {}
})
