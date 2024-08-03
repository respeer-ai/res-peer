<template>
  <q-tabs
    v-model='tab'
    vertical
    class='text-black'
  >
    <q-tab name='settings' label='Settings' />
    <q-tab name='contents' label='Contents' />
    <q-tab name='credits' label='Credits' />
    <q-tab name='assets' label='Assets' />
    <q-tab v-if='reviewer' name='review-contents' label='Review Content' />
    <q-tab v-if='reviewer' name='review-assets' label='Review Asset' />
    <q-tab v-if='reviewer' name='review-reviewers' label='Review Reviewer' />
    <q-tab v-if='reviewer' name='review-activities' label='Review Activity' />
    <q-tab v-if='!reviewer' name='apply-reviewer' label='Apply Reviewer' />
    <q-tab name='foundation' label='Foundation' />
    <q-tab name='activity' label='Activity' />
  </q-tabs>
</template>

<script setup lang='ts'>
import { computed, ref, watch } from 'vue'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'

const user = useUserStore()
const reviewer = computed(() => user.reviewer)
const setting = useSettingStore()
const tab = ref(setting.currentDashboardTab)

watch(tab, () => {
  setting.currentDashboardTab = tab.value
})

</script>

<style scoped lang='sass'>
</style>
