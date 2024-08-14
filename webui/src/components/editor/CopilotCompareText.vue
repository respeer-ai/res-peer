<template>
  <q-card flat bordered>
    <VueDiff
      mode='split'
      theme='light'
      :prev='prevText'
      :current='currentText'
      language='plaintext'
    />
  </q-card>
</template>

<script setup lang='ts'>
import { toRef } from 'vue'

interface Props {
  prevText: string
  currentText: string
}

const props = defineProps<Props>()
const prevText = toRef(props, 'prevText')
const currentText = toRef(props, 'currentText')

</script>
