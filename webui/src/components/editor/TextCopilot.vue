<template>
  <q-card class='q-pa-md' :style='{padding: "36px 24px"}'>
    <q-stepper
      v-model='step'
      class='full-width'
    >
      <q-step
        :name='1'
        title='Select Vendor'
        :done='step > 1'
      >
        <div class='text-black'>
          <CPNodeSelector v-model='cpNodeId' />
        </div>
      </q-step>
      <q-step
        :name='2'
        title='Get Task Id'
        :done='step > 2'
      >
        <div class='text-black'>
          <CopilotQueryId :node-id='cpNodeId' :text='text' :task-type='taskType' v-model='queryId' />
        </div>
      </q-step>
      <q-step
        :name='3'
        title='Pay Task'
        :done='step > 3'
      >
        <CopilotDepositQuery
          :node-id='cpNodeId' :text='text' :task-type='taskType' :query-id='queryId?.queryId'
          @done='onDepositQueryDone' @fail='onDepositQueryFail'
        />
      </q-step>
      <q-step
        :name='4'
        title='Execute Task'
        :done='step > 4'
      >
        {{ text }} {{ taskType }}
      </q-step>
      <q-step
        :name='5'
        title='Compare Result'
        :done='step > 5'
      >
        {{ text }} {{ taskType }}
      </q-step>
    </q-stepper>
    <div class='row'>
      <q-space />
      <q-btn
        flat
        rounded
        label='Next'
        class='bg-red-2'
        :disable='forwardable'
        @click='onNextClick'
      />
      <q-btn
        flat
        rounded
        label='Cancel'
        @click='onCancelClick'
      />
    </div>
  </q-card>
</template>

<script setup lang='ts'>
import { computed, defineProps, ref, toRef } from 'vue'
import { TaskType } from 'src/stores/cpregistry'
import { QueryId } from 'src/stores/copilot'

import CPNodeSelector from './CPNodeSelector.vue'
import CopilotQueryId from './CopilotQueryId.vue'
import CopilotDepositQuery from './CopilotDepositQuery.vue'

interface Props {
  text: string
  taskType: TaskType
}

const props = defineProps<Props>()
const text = toRef(props, 'text')
const taskType = toRef(props, 'taskType')

const step = ref(1)
const cpNodeId = ref(undefined as unknown as string)
const queryId = ref(undefined as unknown as QueryId)

const emit = defineEmits<{(ev: 'cancel'): void}>()

const onNextClick = () => {
  step.value++
}

const onCancelClick = () => {
  emit('cancel')
}

const forwardable = computed(() => {
  switch (step.value) {
    case 1: return !cpNodeId.value?.length
    case 2: return !queryId.value?.queryId?.length
    default: return false
  }
})

const onDepositQueryDone = () => {
  step.value++
}

const onDepositQueryFail = () => {
  // TODO
}

</script>
