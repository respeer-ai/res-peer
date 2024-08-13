<template>
  <q-card flat bordered :style='{padding: "48px"}'>
    <div class='row'>
      <div>
        <div :style='{fontSize: "28px"}'>
          Preparing task query
        </div>
        <div :style='{marginTop: "16px"}' class='text-bold'>
          Task Node:
        </div>
        <div class='row' :style='{marginTop: "8px", lineHeight: "24px"}'>
          <q-avatar size='24px'>
            <q-img :src='node?.brandLogo' width='24px' height='24px' />
          </q-avatar>
          <span :style='{marginLeft: "8px"}'>{{ node?.brandName }}</span>
          <span :style='{marginLeft: "8px"}' class='text-grey-8'>{{ nodeId }}</span>
        </div>
        <div :style='{marginTop: "16px"}' class='text-bold'>
          {{ taskTypeName(taskType) }}:
        </div>
        <div class='text-grey-6'>
          {{ text }}
        </div>
      </div>
      <q-space />
      <div>
        <q-card flat :style='{width: "240px", height: "80px"}'>
          <q-inner-loading
            :showing='true'
            class='text-red-4'
          >
            <q-spinner-facebook size='80px' />
          </q-inner-loading>
        </q-card>
        <div :style='{width: "240px"}' class='text-center text-grey-8'>
          {{ stepText }}
        </div>
      </div>
      <q-space />
    </div>
  </q-card>
</template>

<script setup lang='ts'>
import { TaskType, taskTypeName, useCPRegistryStore } from 'src/stores/cpregistry'
import { computed, ref, toRef } from 'vue'

interface Props {
  nodeId: string
  text: string
  taskType: TaskType
}

const props = defineProps<Props>()
const nodeId = toRef(props, 'nodeId')
const text = toRef(props, 'text')
const taskType = toRef(props, 'taskType')

const stepText = ref('Requesting application ...')

const cpRegistry = useCPRegistryStore()
const node = computed(() => cpRegistry.nodes.find((el) => el.nodeId === nodeId.value))

// TODO: request application
// TODO: application request subscribe
// TODO: get query Id

</script>
