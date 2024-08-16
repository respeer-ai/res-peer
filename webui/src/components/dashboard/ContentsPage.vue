<template>
  <q-tabs
    v-model='tab'
    class='text-black dashboard-tabs'
    indicator-color='red-6'
    align='left'
    v-if='!editing'
  >
    <q-tab name='submitted' label='SUBMITTED' />
    <q-tab name='published' label='PUBLISHED' />
    <q-tab name='rewarded' label='REWARDED' />
    <q-tab name='tip-others' label='TIP OTHERS' />
    <q-tab name='liked' label='LIKED' />
    <q-tab name='disliked' label='DISLIKED' />
    <q-tab name='drafts' label='DRAFTS' />
    <q-space />
    <q-btn
      flat
      rounded
      color='black'
      class='bg-red-1'
      @click='onWriteClick'
      :style='{width: "128px"}'
    >
      <div class='row'>
        <inline-svg
          :src='writeIcon'
          width='20'
          height='20'
          class='item-icon-active'
        />
        <span :style='{marginLeft: "8px"}'>WRITE</span>
      </div>
    </q-btn>
  </q-tabs>

  <q-tab-panels
    v-model='tab'
    animated
    swipeable
    vertical
    transition-prev='jump-up'
    transition-next='jump-up'
    class='text-black'
    v-if='!editing'
  >
    <q-tab-panel name='submitted'>
      <content-application-list />
    </q-tab-panel>
    <q-tab-panel name='published'>
      <article-list article-type='MY_ARTICLE' />
    </q-tab-panel>
    <q-tab-panel name='rewarded'>
      <article-list article-type='MY_REWARD' />
    </q-tab-panel>
    <q-tab-panel name='tip-others'>
      <article-list article-type='MY_TIP_OTHERS' />
    </q-tab-panel>
    <q-tab-panel name='liked'>
      <article-list article-type='MY_LIKE' />
    </q-tab-panel>
    <q-tab-panel name='disliked'>
      <article-list article-type='MY_DISLIKE' />
    </q-tab-panel>
    <q-tab-panel name='drafts'>
      <article-list article-type='DRAFTS' />
    </q-tab-panel>
  </q-tab-panels>

  <div class='row' :style='{width: "100%"}'>
    <div :style='{width: "800px", marginLeft: "calc(50% - 540px)"}'>
      <submit-content
        v-if='editing'
        @canceled='onSubmitContentCancel'
        @error='onSubmitContentError'
        @submitted='onSubmitContentSubmitted'
      />
    </div>
  </div>
</template>

<script setup lang='ts'>
import { ref } from 'vue'
import { writeIcon } from 'src/assets'
import { useRoute } from 'vue-router'

import SubmitContent from 'src/components/SubmitContent.vue'
import ArticleList from 'src/components/ArticleList.vue'
import ContentApplicationList from 'src/components/ContentApplicationList.vue'

interface Query {
  write: string
}

const route = useRoute()

const tab = ref('submitted')
const editing = ref((route.query as unknown as Query).write === 'true')

const onWriteClick = () => {
  editing.value = true
}

const onSubmitContentCancel = () => {
  editing.value = false
}

const onSubmitContentError = () => {
  editing.value = false
}

const onSubmitContentSubmitted = () => {
  editing.value = false
}

</script>
