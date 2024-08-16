<template>
  <q-card :style='{padding: "24px"}'>
    <div class='row'>
      <q-avatar size='64px'>
        <q-img :src='cpNode.brandLogo' width='64px' height='64px' />
      </q-avatar>
      <div :style='{marginLeft: "16px", borderBottom: "1px solid lightgrey", width: "calc(100% - 64px - 16px)"}'>
        <div :style='{fontSize: "20px"}' class='text-bold text-grey-9'>
          {{ cpNode.nodeId }}
        </div>
        <div class='row'>
          <q-badge rounded>
            {{ cpNode.brandName }}
          </q-badge>
          <q-badge rounded :style='{marginLeft: "4px"}' v-if='cpNode.freeQuota'>
            {{ cpNode.freeQuota }} Trials
          </q-badge>
          <q-badge rounded :style='{marginLeft: "4px"}'>
            {{ cpNode.quotaPrice }} TLINERA / Per Task
          </q-badge>
          <q-badge rounded outline color='red-6' :style='{marginLeft: "4px"}'>
            <div>
              {{ cpNode.resourceType }}
            </div>
            <span :style='{marginLeft: "4px", color: "grey"}'>{{ cpNode.deviceModel }}</span>
          </q-badge>
        </div>
      </div>
    </div>
    <div :style='{marginLeft: "80px", marginTop: "24px"}' class='text-grey-8'>
      <div class='row' :style='{borderBottom: "1px solid lightgrey", padding: "8px 0"}'>
        <div :style='{width: "160px"}'>
          CPU
        </div>
        <div :style='{width: "calc(100% - 160px)"}'>
          {{ cpNode.cpuModel }}
        </div>
      </div>
      <div class='row' :style='{borderBottom: "1px solid lightgrey", padding: "8px 0"}'>
        <div :style='{width: "160px"}' class='row'>
          <div>
            Storage
          </div>
          <div :style='{marginLeft: "4px"}'>
            <q-badge rounded>
              {{ cpNode.storageType }}
            </q-badge>
          </div>
        </div>
        <div :style='{width: "calc(100% - 160px)"}'>
          {{ bytes2HumanReadable(cpNode.storageBytes) }}
        </div>
      </div>
      <div class='row' :style='{borderBottom: "1px solid lightgrey", padding: "8px 0"}'>
        <div :style='{width: "160px"}'>
          Memory
        </div>
        <div :style='{width: "calc(100% - 160px)"}'>
          {{ bytes2HumanReadable(cpNode.memoryBytes) }}
        </div>
      </div>
      <div class='row' :style='{borderBottom: "1px solid lightgrey", padding: "8px 0"}'>
        <div :style='{width: "160px"}'>
          Supported Tasks
        </div>
        <div :style='{width: "calc(100% - 160px)"}'>
          <q-badge
            v-for='taskType in cpNode.supportedTaskTypes' :key='taskType'
            :style='{marginRight: "4px"}' rounded
          >
            {{ taskTypeName(taskType) }}
          </q-badge>
        </div>
      </div>
      <div class='row' :style='{borderBottom: "1px solid lightgrey", padding: "8px 0"}'>
        <div :style='{width: "160px"}'>
          Model
        </div>
        <div :style='{width: "calc(100% - 160px)"}'>
          <a :href='cpNode.aiModelUrl'>{{ cpNode.aiModel }}</a>
        </div>
      </div>
      <div class='row' :style='{padding: "8px 0"}'>
        <div :style='{width: "160px"}'>
          Created At
        </div>
        <div :style='{width: "calc(100% - 160px)"}'>
          {{ date.formatDate(cpNode.createdAt / 1000, 'YYYY-MM-DD HH:mm:ss') }}
        </div>
      </div>
    </div>
  </q-card>
</template>

<script lang='ts' setup>
import { defineProps, toRef } from 'vue'
import { date } from 'quasar'
import { CPNode, taskTypeName } from 'src/stores/cpregistry'

interface Props {
  cpNode: CPNode
}

const props = defineProps<Props>()
const cpNode = toRef(props, 'cpNode')

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

</script>

<style scoped lang='sass'>
</style>
