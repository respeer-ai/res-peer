<template>
  <div
    v-for='(_content, index) in contents'
    :key='_content.cid'
    :style='{width: "100%", margin: "16px 0 16px 0"}'
    class='row cursor-pointer'
  >
    <q-space />
    <div>
      <div class='content-card'>
        <content-card-view :cid='_content.cid' />
      </div>
      <q-separator v-if='index < contents.length - 1' :style='{margin: "24px 0"}' />
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
  .item-icon
    color: $red-6 !important
</style>
