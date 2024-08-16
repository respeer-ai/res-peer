<template>
  <q-card flat bordered :style='{padding: "24px", height: "320px"}' class='text-center'>
    <div :style='{fontSize: "28px", lineHeight: "28px"}'>
      Confirm Illustrate Description
    </div>
    <div :style='{marginTop: "16px", width: "100%"}'>
      <textarea v-model='illustrateDescription' auto-resize />
    </div>
  </q-card>
</template>

<script setup lang='ts'>
import { defineModel } from 'vue'

const illustrateDescription = defineModel({ type: String })

</script>

<style scoped lang='sass'>
textarea
  font-size: 0.8rem
  letter-spacing: 1px
  padding: 10px
  width: 100%
  max-width: 800px
  line-height: 1.5
  border-radius: 5px
  border: 1px solid $red-1
  box-shadow: 1px 1px 1px $red-1
  height: calc(320px - 28px - 16px - 48px)

textarea:focus
  outline: none !important
  border: 1px solid $red-2
  box-shadow: 1px 1px 1px $red-2
</style>
