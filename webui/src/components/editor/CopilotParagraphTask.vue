<template>
  <q-card flat bordered :style='{padding: "48px"}'>
    <div class='row'>
      <div>
        <div :style='{fontSize: "28px"}'>
          Executing task
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
        <div :style='{marginTop: "16px"}' class='text-bold'>
          Query Id:
        </div>
        <div class='text-grey-6'>
          {{ queryId?.queryId }}
        </div>
      </div>
      <q-space />
      <div>
        <q-card flat :style='{width: "320px", height: "160px"}'>
          <q-inner-loading
            :showing='!generated'
            class='text-red-4'
          >
            <q-spinner-facebook size='80px' />
          </q-inner-loading>
          <div v-if='generated' class='text-center flex items-center justify-center' :style='{width: "320px", wordBreak: "break-word", fontSize: "28px"}'>
            Generated text
          </div>
          <div v-if='generated' class='text-center flex items-center justify-center text-grey-8' :style='{width: "320px", height: "100%", wordBreak: "break-word"}'>
            {{ result }}
          </div>
        </q-card>
        <div :style='{width: "320px"}' :class='["text-center", error ? "text-red-6" : "text-grey-8"]'>
          {{ stepText }}
        </div>
      </div>
      <q-space />
    </div>
  </q-card>
  <ParagraphTask
    v-model='result'
    :node-id='nodeId'
    :query-id='queryId'
    :task-type='taskType'
    :text='text'
    :signature='signature'
    @done='onTaskDone'
    @fail='onTaskFail'
  />
</template>

<script setup lang='ts'>
import { TaskType, taskTypeName, useCPRegistryStore } from 'src/stores/cpregistry'
import { computed, ref, toRef, defineModel } from 'vue'
import { QueryId } from 'src/stores/copilot'

import ParagraphTask from '../copilot/ParagraphTask.vue'

interface Props {
  nodeId: string
  queryId: QueryId
  taskType: TaskType
  text: string
  signature: string
}

const emit = defineEmits<{(ev: 'done'): void,
  (ev: 'fail'): void
}>()

const props = defineProps<Props>()
const nodeId = toRef(props, 'nodeId')
const queryId = toRef(props, 'queryId')
const taskType = toRef(props, 'taskType')
const text = toRef(props, 'text')
const signature = toRef(props, 'signature')

const stepText = ref('Executing task ...')
const error = ref(false)
const generated = ref(false)

const result = defineModel({ type: String })

const cpRegistry = useCPRegistryStore()
const node = computed(() => cpRegistry.nodes.find((el) => el.nodeId === nodeId.value))

const onTaskDone = () => {
  generated.value = true
  stepText.value = ''
  emit('done')
}

const onTaskFail = () => {
  stepText.value = 'Failed execute task!'
  error.value = true
  emit('fail')
}

</script>
