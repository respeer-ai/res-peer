<template>
  <q-scroll-area
    class='fit'
    :bar-style='{
      borderRadius: "2px",
      width: "0"
    }'
    :thumb-style='{
      borderRadius: "2px",
      width: "2px"
    }'
  >
    <q-tabs
      v-model='tab'
      vertical
      class='text-black'
      no-caps
      :style='{padding: "16px 32px"}'
      indicator-color='red-6'
      narrow-indicator
    >
      <div class='row cursor-pointer drawer-item drawer-padding hover-bg' v-if='false'>
        <inline-svg
          :src='homeIcon'
          width='20'
          height='20'
          class='item-icon'
        />
        <div>Home</div>
      </div>
      <div class='row cursor-pointer drawer-item drawer-padding hover-bg' v-if='false'>
        <inline-svg
          :src='favoriteIcon'
          width='20'
          height='20'
          class='item-icon'
        />
        <div>Favorite</div>
      </div>
      <q-separator v-if='false' />
      <q-expansion-item label='CREATOR' class='text-grey-8' default-opened>
        <q-tab name='contents'>
          <div class='row'>
            <inline-svg
              :src='contentsIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "contents" ? "item-icon-active" : "" ]'
            />
            <div>Contents</div>
          </div>
        </q-tab>
        <q-tab name='assets'>
          <div class='row'>
            <inline-svg
              :src='assetsIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "assets" ? "item-icon-active" : "" ]'
            />
            <div>Assets</div>
          </div>
        </q-tab>
      </q-expansion-item>
      <q-separator />
      <q-expansion-item label='EARNINGS' class='text-grey-8' default-opened>
        <q-tab name='balance'>
          <div class='row'>
            <inline-svg
              :src='balanceIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "balance" ? "item-icon-active" : "" ]'
            />
            <div>Balance</div>
          </div>
        </q-tab>
        <q-tab name='computing'>
          <div class='row'>
            <inline-svg
              :src='computingIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "computing" ? "item-icon-active" : "" ]'
            />
            <div>Computing</div>
          </div>
        </q-tab>
        <q-tab name='credits'>
          <div class='row'>
            <inline-svg
              :src='creditsIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "credits" ? "item-icon-active" : "" ]'
            />
            <div>Credits</div>
          </div>
        </q-tab>
        <q-tab name='badges'>
          <div class='row'>
            <inline-svg
              :src='badgesIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "badges" ? "item-icon-active" : "" ]'
            />
            <div>Badges</div>
          </div>
        </q-tab>
      </q-expansion-item>
      <q-separator />
      <q-expansion-item label='REVIEWER DAO' class='text-grey-8'>
        <q-tab name='review-contents' v-if='reviewer'>
          <div class='row'>
            <inline-svg
              :src='reviewContentIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "review-contents" ? "item-icon-active" : "" ]'
            />
            <div>Contents</div>
          </div>
        </q-tab>
        <q-tab name='review-assets' v-if='reviewer'>
          <div class='row'>
            <inline-svg
              :src='reviewAssetIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "review-assets" ? "item-icon-active" : "" ]'
            />
            <div>Assets</div>
          </div>
        </q-tab>
        <q-tab name='review-reviewers' v-if='reviewer'>
          <div class='row'>
            <inline-svg
              :src='reviewReviewerIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "review-reviewers" ? "item-icon-active" : "" ]'
            />
            <div>Reviewers</div>
          </div>
        </q-tab>
        <q-tab name='review-activities' v-if='reviewer'>
          <div class='row'>
            <inline-svg
              :src='reviewActivityIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "review-activities" ? "item-icon-active" : "" ]'
            />
            <div>Activities</div>
          </div>
        </q-tab>
        <q-tab name='apply-reviewer' v-if='!reviewer'>
          <div class='row'>
            <inline-svg
              :src='applyReviewerIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "apply-reviewer" ? "item-icon-active" : "" ]'
            />
            <div>Apply Reviewer</div>
          </div>
        </q-tab>
      </q-expansion-item>
      <q-separator />
      <q-expansion-item label='FOUNDATION' class='text-grey-8'>
        <q-tab name='foundation'>
          <div class='row'>
            <inline-svg
              :src='foundationIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "foundation" ? "item-icon-active" : "" ]'
            />
            <div>Foundation</div>
          </div>
        </q-tab>
      </q-expansion-item>
      <q-separator />
      <q-expansion-item label='USERS CLUB' class='text-grey-8'>
        <q-tab name='activities'>
          <div class='row'>
            <inline-svg
              :src='activitiesIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "activities" ? "item-icon-active" : "" ]'
            />
            <div>Activities</div>
          </div>
        </q-tab>
      </q-expansion-item>
      <q-separator />
      <q-expansion-item label='SETTINGS' class='text-grey-8'>
        <q-tab name='settings'>
          <div class='row'>
            <q-icon class='item-icon' name='bi-body-text' size='24px' />
            <inline-svg
              :src='settingsIcon'
              width='20'
              height='20'
              :class='[ "item-icon", tab === "settings" ? "item-icon-active" : "" ]'
            />
            <div>Settings</div>
          </div>
        </q-tab>
      </q-expansion-item>
    </q-tabs>
  </q-scroll-area>
</template>

<script setup lang='ts'>
import { computed, ref, watch } from 'vue'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'

import homeIcon from 'src/assets/HomeIcon.svg'
import favoriteIcon from 'src/assets/FavoriteIcon.svg'
import contentsIcon from 'src/assets/ContentsIcon.svg'
import assetsIcon from 'src/assets/AssetsIcon.svg'
import balanceIcon from 'src/assets/BalanceIcon.svg'
import computingIcon from 'src/assets/ComputingIcon.svg'
import creditsIcon from 'src/assets/CreditsIcon.svg'
import badgesIcon from 'src/assets/BadgesIcon.svg'
import reviewActivityIcon from 'src/assets/ReviewActivityIcon.svg'
import reviewAssetIcon from 'src/assets/ReviewAssetIcon.svg'
import reviewReviewerIcon from 'src/assets/ReviewReviewerIcon.svg'
import reviewContentIcon from 'src/assets/ReviewContentIcon.svg'
import foundationIcon from 'src/assets/FoundationIcon.svg'
import activitiesIcon from 'src/assets/ActivitiesIcon.svg'
import settingsIcon from 'src/assets/SettingIcon.svg'
import applyReviewerIcon from 'src/assets/ApplyReviewerIcon.svg'

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
