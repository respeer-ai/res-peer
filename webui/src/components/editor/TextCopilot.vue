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
          <CopilotQueryId
            :node-id='cpNodeId' :text='text' :task-type='taskType' v-model='queryId'
            v-model:signature='signature'
          />
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
        <CopilotParagraphTask
          v-model='resultText'
          :node-id='cpNodeId'
          :query-id='queryId'
          :task-type='taskType'
          :text='text'
          :signature='signature'
          @done='onExecuteTaskDone'
          @fail='onExecuteTaskFail'
        />
      </q-step>
      <q-step
        :name='5'
        title='Compare Result'
        :done='step > 5'
      >
        <CopilotCompareText
          :prev-text='text'
          :current-text='resultText'
        />
      </q-step>
    </q-stepper>
    <div class='row'>
      <q-space />
      <q-btn
        flat
        rounded
        :label='nextText'
        class='bg-red-2'
        :disable='forwardable'
        @click='onNextClick'
      />
      <q-btn
        flat
        rounded
        :label='cancelText'
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
import CopilotParagraphTask from './CopilotParagraphTask.vue'
import CopilotCompareText from './CopilotCompareText.vue'

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
const queryConfirmed = ref(false)
const signature = ref(undefined as unknown as string)
const resultText = ref(undefined as unknown as string)

const emit = defineEmits<{(ev: 'cancel'): void,
  (ev: 'changeText', v: string): void
}>()

const onNextClick = () => {
  if (step.value === 5) {
    return emit('changeText', resultText.value)
  }
  step.value++
}

const onCancelClick = () => {
  emit('cancel')
}

const forwardable = computed(() => {
  switch (step.value) {
    case 1: return !cpNodeId.value?.length
    case 2: return !queryId.value?.queryId?.length
    case 3: return !queryConfirmed.value
    case 4: return !resultText.value?.length
    default: return false
  }
})

const nextText = computed(() => {
  switch (step.value) {
    case 5: return 'Accept'
    default: return 'Next'
  }
})

const cancelText = computed(() => {
  switch (step.value) {
    case 5: return 'Discard'
    default: return 'Cancel'
  }
})

const onDepositQueryDone = () => {
  queryConfirmed.value = true
}

const onDepositQueryFail = () => {
  // TODO
}

const onExecuteTaskDone = () => {
  // TODO
}

const onExecuteTaskFail = () => {
  // TODO
}

</script>
