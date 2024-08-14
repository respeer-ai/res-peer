<template>
  <q-card flat bordered :style='{padding: "48px"}'>
    <div class='row'>
      <div>
        <div :style='{fontSize: "28px"}'>
          Paying task
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
        <div :style='{marginTop: "16px"}' class='text-bold'>
          Query Id:
        </div>
        <div class='text-grey-6'>
          {{ queryId }}
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
        <div :style='{width: "240px"}' :class='["text-center", error ? "text-red-6" : "text-grey-8"]'>
          {{ stepText }}
        </div>
      </div>
      <q-space />
    </div>
  </q-card>
  <DepositQuery
    :node-id='nodeId'
    :query-id='queryId'
    @paid='onQueryPaid'
    @confirmed='onQueryConfirmed'
    @fail='onDepositQueryFail'
  />
</template>

<script setup lang='ts'>
import { TaskType, taskTypeName, useCPRegistryStore } from 'src/stores/cpregistry'
import { computed, ref, toRef } from 'vue'

import DepositQuery from '../copilot/DepositQuery.vue'

interface Props {
  nodeId: string
  queryId: string
  taskType: TaskType
  text: string
}

const emit = defineEmits<{(ev: 'done'): void,
  (ev: 'fail'): void
}>()

const props = defineProps<Props>()
const nodeId = toRef(props, 'nodeId')
const queryId = toRef(props, 'queryId')
const taskType = toRef(props, 'taskType')
const text = toRef(props, 'text')

const stepText = ref('Paying task ...')
const error = ref(false)

const cpRegistry = useCPRegistryStore()
const node = computed(() => cpRegistry.nodes.find((el) => el.nodeId === nodeId.value))

const onQueryPaid = () => {
  stepText.value = 'Waiting for confirmation ...'
}

const onDepositQueryFail = () => {
  stepText.value = 'Failed to pay task!'
  error.value = true
  emit('fail')
}

const onQueryConfirmed = () => {
  stepText.value = ''
  emit('done')
}
</script>
