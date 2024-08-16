<template>
  <div
    v-for='(_content, index) in contents'
    :key='_content.cid'
    :style='{width: "100%", margin: "16px 0 16px 0"}'
    class='row cursor-pointer content-card'
  >
    <q-space />
    <div>
      <content-card-view :cid='_content.cid' />
      <q-separator v-if='index > 0' :style='{margin: "48px 0"}' />
    </div>
    <q-space />
  </div>
</template>

<script setup lang='ts'>
import { useContentStore } from 'src/stores/content'
import { computed } from 'vue'

import ContentCardView from './ContentCardViewV1.vue'

const content = useContentStore()
const contents = computed(() => content._contents())

</script>

<style scoped lang='sass'>
.content-card:hover
  background: $grey-3
  border-radius: 16px
  transition: 1s
</style>
