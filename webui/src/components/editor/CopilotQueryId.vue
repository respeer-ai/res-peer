<template>
  <div>{{ nodeId }} {{ text }} {{ taskType }}</div>
</template>

<script setup lang='ts'>
import { TaskType } from 'src/stores/cpregistry'
import { toRef } from 'vue'

interface Props {
  nodeId: string
  text: string
  taskType: TaskType
}

const props = defineProps<Props>()
const nodeId = toRef(props, 'nodeId')
const text = toRef(props, 'text')
const taskType = toRef(props, 'taskType')

</script>
