<template>
  <q-card class='q-pa-md' :style='{padding: "36px 24px"}'>
    <q-stepper
      v-model='step'
      class='full-width'
    >
      <q-step
        :name='1'
        title='Illustrate Description'
        :done='step > 1'
      >
        <CopilotIllustrateDescription v-model='illustrateDescription' />
      </q-step>
      <q-step
        :name='2'
        title='Select Vendor'
        :done='step > 2'
      >
        <div class='text-black'>
          <CPNodeSelector v-model='cpNodeId' />
        </div>
      </q-step>
      <q-step
        :name='3'
        title='Get Task Id'
        :done='step > 3'
      >
        <div class='text-black'>
          <CopilotQueryId
            :node-id='cpNodeId' :text='text' :task-type='taskType' v-model='queryId'
            v-model:signature='signature'
          />
        </div>
      </q-step>
      <q-step
        :name='4'
        title='Pay Task'
        :done='step > 4'
      >
        <CopilotDepositQuery
          :node-id='cpNodeId' :text='text' :task-type='taskType' :query-id='queryId?.queryId'
          @done='onDepositQueryDone' @fail='onDepositQueryFail'
        />
      </q-step>
      <q-step
        :name='5'
        title='Execute Task'
        :done='step > 5'
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
import { computed, defineProps, ref, toRef, defineModel, onMounted } from 'vue'
import { TaskType } from 'src/stores/cpregistry'
import { QueryId } from 'src/stores/copilot'

import CopilotIllustrateDescription from './CopilotIllustrateDescription.vue'
import CPNodeSelector from './CPNodeSelector.vue'
import CopilotQueryId from './CopilotQueryId.vue'
import CopilotDepositQuery from './CopilotDepositQuery.vue'
import CopilotParagraphTask from './CopilotParagraphTask.vue'

interface Props {
  text: string
}

const props = withDefaults(defineProps<Props>(), {
  text: ''
})
const text = toRef(props, 'text')
const taskType = ref(TaskType.GenerateIllustrate)

const step = ref(1)
const illustrateDescription = defineModel({ type: String })
const cpNodeId = ref(undefined as unknown as string)
const queryId = ref(undefined as unknown as QueryId)
const queryConfirmed = ref(false)
const signature = ref(undefined as unknown as string)
const resultText = ref(undefined as unknown as string)

const emit = defineEmits<{(ev: 'cancel'): void,
  (ev: 'generated', v: string): void
}>()

const onNextClick = () => {
  if (step.value === 5) {
    return emit('generated', resultText.value)
  }
  step.value++
}

const onCancelClick = () => {
  emit('cancel')
}

const forwardable = computed(() => {
  switch (step.value) {
    case 2: return !cpNodeId.value?.length
    case 3: return !queryId.value?.queryId?.length
    case 4: return !queryConfirmed.value
    case 5: return !resultText.value?.length
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

onMounted(() => {
  // Work around to defineModel issue
  illustrateDescription.value = text.value
})

</script>
