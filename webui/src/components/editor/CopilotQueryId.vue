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
        <div class='text-grey-6' :style='{maxWidth: "600px"}'>
          {{ text }}
        </div>
      </div>
      <q-space />
      <div>
        <q-card flat :style='{width: "320px", height: "80px"}'>
          <q-inner-loading
            :showing='!queryId?.queryId?.length'
            class='text-red-4'
          >
            <q-spinner-facebook size='80px' />
          </q-inner-loading>
          <div v-if='queryId?.queryId?.length' class='text-center' :style='{height: "100%", fontSize: "24px", paddingTop: "32px"}'>
            Query Id
          </div>
        </q-card>
        <div :style='{width: "320px", wordBreak: "break-word"}' :class='["text-center", error ? "text-red-6" : "text-grey-8"]'>
          {{ stepText }}
        </div>
      </div>
      <q-space />
    </div>
  </q-card>
  <RequestApplication
    v-if='node && step === 1'
    :application-id='node?.applicationId'
    @done='onRequestApplicationDone'
    @fail='onRequestApplicationFail'
  />
  <GetQueryId
    v-if='step === 2'
    :node-id='nodeId'
    :text='text'
    :task-type='taskType'
    v-model='queryId'
    v-model:signature='signature'
    @done='onGetQueryIdDone'
    @fail='onGetQueryIdFail'
  />
</template>

<script setup lang='ts'>
import { TaskType, taskTypeName, useCPRegistryStore } from 'src/stores/cpregistry'
import { computed, ref, toRef, defineModel, watch } from 'vue'

import RequestApplication from '../application/RequestApplication.vue'
import GetQueryId from '../copilot/GetQueryId.vue'
import { QueryId } from 'src/stores/copilot'

interface Props {
  nodeId: string
  text: string
  taskType: TaskType
}

const props = defineProps<Props>()
const nodeId = toRef(props, 'nodeId')
const text = toRef(props, 'text')
const taskType = toRef(props, 'taskType')

const step = ref(1)
const stepText = ref('Requesting application ...')
const error = ref(false)

const cpRegistry = useCPRegistryStore()
const node = computed(() => cpRegistry.nodes.find((el) => el.nodeId === nodeId.value))

const queryId = defineModel({ type: Object })
const signature = defineModel('signature', { type: String })

const onRequestApplicationDone = () => {
  stepText.value = 'Generating queryId ...'
  step.value++
}

const onRequestApplicationFail = () => {
  stepText.value = 'Failed to request application!'
  error.value = true
}

watch(queryId, () => {
  stepText.value = (queryId.value as QueryId)?.queryId
})

const onGetQueryIdDone = () => {
  stepText.value = (queryId.value as QueryId)?.queryId
}

const onGetQueryIdFail = () => {
  stepText.value = 'Failed to get queryId!'
  error.value = true
}

</script>
