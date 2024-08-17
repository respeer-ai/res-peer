<template>
  <q-table
    :rows='cpNodes'
    row-key='nodeId'
    :columns='(columns as never)'
    :rows-per-page-options='[5]'
    flat
    bordered
  >
    <template #body='props'>
      <q-tr :props='props' @click='onRowClick(props.row)' class='cursor-pointer'>
        <q-td key='selection' :props='props'>
          <q-checkbox :model-value='nodeId === props.row.nodeId' />
        </q-td>
        <q-td key='vendorInformation' :props='props'>
          <div class='text-bold'>
            <div>{{ shortId(props.row.nodeId, 8) }}</div>
            <div class='row'>
              <q-avatar size='36px' color='red-1'>
                <q-img :src='props.row.brandLogo' width='36px' height='36px' fit='contain' />
              </q-avatar>
              <span :style='{lineHeight: "36px", marginLeft: "8px"}'>{{ props.row.brandName }}</span>
            </div>
          </div>
        </q-td>
        <q-td key='modelInformation' :props='props'>
          <div>{{ props.row.resourceType }} {{ props.row.deviceModel }}</div>
          <div><a :href='props.row.aiModelUrl'>{{ props.row.aiModel }}</a></div>
        </q-td>
        <q-td key='deviceInformation' :props='props'>
          <div>{{ props.row.cpuModel }} {{ bytes2HumanReadable(props.row.memoryBytes) }} RAM</div>
          <div>{{ bytes2HumanReadable(props.row.storageBytes) }} {{ props.row.storageType }}</div>
        </q-td>
        <q-td key='taskPrice' :props='props'>
          <div :style='{lineHeight: "24px"}'>
            <q-img src='https://avatars.githubusercontent.com/u/107513858?s=48&v=4' width='24px' height='24px' />
            {{ props.row.quotaPrice }} TLINERA
          </div>
        </q-td>
        <q-td key='freeQuota' :props='props'>
          {{ props.row.freeQuota }} Tasks
        </q-td>
        <q-td key='supportedTaskTypes' :props='props'>
          <div v-for='taskType in props.row.supportedTaskTypes' :key='taskType'>
            <q-badge color='red-2' class='text-black'>
              {{ taskTypeName(taskType) }}
            </q-badge>
          </div>
        </q-td>
      </q-tr>
    </template>
  </q-table>
</template>

<script setup lang='ts'>
import { CPNode, taskTypeName, useCPRegistryStore } from 'src/stores/cpregistry'
import { computed, defineModel } from 'vue'
import { shortId } from 'src/utils/shortid'

const cpRegistry = useCPRegistryStore()
const cpNodes = computed(() => cpRegistry.nodes || [])

const nodeId = defineModel({ required: true, type: String })

const onRowClick = (row: CPNode) => {
  if (nodeId.value === row.nodeId) {
    nodeId.value = undefined as unknown as string
  } else {
    nodeId.value = row.nodeId
  }
}

const bytes2HumanReadable = (bytes: number) => {
  if (bytes >= 1024 * 1024 * 1024 * 1024) {
    return (bytes / 1024 / 1024 / 1024 / 1024).toFixed(2) + ' TiB'
  }
  if (bytes >= 1024 * 1024 * 1024) {
    return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GiB'
  }
  if (bytes >= 1024 * 1024) {
    return (bytes / 1024 / 1024).toFixed(2) + ' MiB'
  }
  if (bytes >= 1024) {
    return (bytes / 1024).toFixed(2) + ' KiB'
  }
  return bytes.toString() + ' B'
}

const columns = computed(() => [
  {
    name: 'selection',
    align: 'left',
    field: (row: CPNode) => row.nodeId
  },
  {
    name: 'vendorInformation',
    label: 'Vendor Information',
    align: 'left',
    field: (row: CPNode) => row.nodeId
  },
  {
    name: 'modelInformation',
    label: 'Model Information',
    align: 'left',
    field: (row: CPNode) => row.aiModel
  },
  {
    name: 'deviceInformation',
    label: 'Device Information',
    align: 'left',
    field: (row: CPNode) => row.deviceModel
  },
  {
    name: 'taskPrice',
    label: 'Price per Task',
    align: 'left',
    field: (row: CPNode) => row.quotaPrice
  },
  {
    name: 'freeQuota',
    label: 'Trial Quota',
    align: 'left',
    field: (row: CPNode) => row.freeQuota
  },
  {
    name: 'supportedTaskTypes',
    label: 'Supported Task Types',
    align: 'left',
    field: (row: CPNode) => row.supportedTaskTypes
  }
])

</script>
