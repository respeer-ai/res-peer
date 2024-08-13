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
          <CopilotQueryId :node-id='cpNodeId' :text='text' :task-type='taskType' />
        </div>
      </q-step>
      <q-step
        :name='3'
        title='Pay Task'
        :done='step > 3'
      >
        {{ text }} {{ taskType }}
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
        :disable='!cpNodeId?.length'
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
import { defineProps, ref, toRef } from 'vue'
import { TaskType } from 'src/stores/cpregistry'

import CPNodeSelector from './CPNodeSelector.vue'
import CopilotQueryId from './CopilotQueryId.vue'

interface Props {
  text: string
  taskType: TaskType
}

const props = defineProps<Props>()
const text = toRef(props, 'text')
const taskType = toRef(props, 'taskType')

const step = ref(1)
const cpNodeId = ref(undefined as unknown as string)

const emit = defineEmits<{(ev: 'cancel'): void}>()

const onNextClick = () => {
  step.value++
}

const onCancelClick = () => {
  emit('cancel')
}

</script>
