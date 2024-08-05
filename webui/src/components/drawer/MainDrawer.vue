<template>
  <q-scroll-area
    class='fit'
    :bar-style='{
      borderRadius: "2px",
      width: "4px"
    }'
    :thumb-style='{
      borderRadius: "2px",
      width: "4px"
    }'
  >
    <q-tabs
      v-model='tab'
      vertical
      class='text-black'
      no-caps
      :style='{padding: "16px 32px"}'
    >
      <div :class='[ "row cursor-pointer drawer-item", mainTab === "feed" ? "bg-red-1" : "" ]'>
        <q-icon name='bi-house' size='24px' :class='[ "item-icon", mainTab === "feed" ? "text-red-6" : "" ]' />
        <div>Home</div>
      </div>
      <div :class='[ "row cursor-pointer drawer-item", mainTab === "favorite" ? "bg-red-1" : "" ]'>
        <q-icon name='bi-heart' size='24px' :class='[ "item-icon", mainTab === "favorite" ? "text-red-6" : "" ]' />
        <div>Favorite</div>
      </div>
      <q-separator />
      <div class='drawer-section'>
        CREATOR
      </div>
      <q-tab name='contents'>
        <div class='row'>
          <q-icon class='item-icon' name='bi-body-text' size='24px' />
          <div>Contents</div>
        </div>
      </q-tab>
      <q-tab name='assets'>
        <div class='row'>
          <q-icon class='item-icon' name='bi-wallet' size='24px' />
          <div>Assets</div>
        </div>
      </q-tab>
      <q-separator />
      <div class='drawer-section'>
        EARNINGS
      </div>
      <q-tab name='balance'>
        <div class='row'>
          <q-icon class='item-icon' name='bi-wallet2' size='24px' />
          <div>Balance</div>
        </div>
      </q-tab>
      <q-tab name='computing'>
        <div class='row'>
          <q-icon class='item-icon' name='bi-motherboard' size='24px' />
          <div>Computing</div>
        </div>
      </q-tab>
      <q-tab name='credits'>
        <div class='row'>
          <q-icon class='item-icon' name='bi-bar-chart-steps' size='24px' />
          <div>Credits</div>
        </div>
      </q-tab>
      <q-tab name='badges'>
        <div class='row'>
          <q-icon class='item-icon' name='bi-suit-diamond' size='24px' />
          <div>Badges</div>
        </div>
      </q-tab>
      <q-separator />
      <div class='drawer-section'>
        REVIEWER DAO
      </div>
      <q-tab v-if='reviewer' name='review-contents' label='Contents' />
      <q-tab v-if='reviewer' name='review-assets' label='Assets' />
      <q-tab v-if='reviewer' name='review-reviewers' label='Reviewers' />
      <q-tab v-if='reviewer' name='review-activities' label='Activities' />
      <q-tab v-if='!reviewer' name='apply-reviewer' label='Apply Reviewer' />
      <q-separator />
      <div class='drawer-section'>
        FOUNDATION
      </div>
      <q-tab name='foundation' label='Foundation' />
      <q-separator />
      <div class='drawer-section'>
        USERS CLUB
      </div>
      <q-tab name='activity' label='Activities' />
      <q-separator />
      <div class='drawer-section'>
        SETTING
      </div>
      <q-tab name='settings' label='Settings' />
    </q-tabs>
  </q-scroll-area>
</template>

<script setup lang='ts'>
import { computed, ref, watch } from 'vue'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'

const user = useUserStore()
const reviewer = computed(() => user.reviewer)
const setting = useSettingStore()
const tab = ref(setting.currentDashboardTab)
const mainTab = computed(() => setting.currentMainTab)

watch(tab, () => {
  setting.currentDashboardTab = tab.value
})

</script>

<style scoped lang='sass'>
</style>
