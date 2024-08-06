<template>
  <div :style='{maxWidth: "1440px", margin: "32px auto"}'>
    <q-tab-panels
      v-model='tab'
      animated
      swipeable
      vertical
      transition-prev='jump-up'
      transition-next='jump-up'
      class='text-black'
    >
      <q-tab-panel name='settings'>
        <avatar-setting />
      </q-tab-panel>
      <q-tab-panel name='contents'>
        <contents-page />
      </q-tab-panel>
      <q-tab-panel name='review-contents'>
        <review-contents />
      </q-tab-panel>
      <q-tab-panel name='review-assets'>
        <review-assets />
      </q-tab-panel>
      <q-tab-panel name='review-reviewers'>
        <review-reviewers />
      </q-tab-panel>
      <q-tab-panel name='review-activities'>
        <review-activities />
      </q-tab-panel>
      <q-tab-panel name='credits'>
        <user-balance />
      </q-tab-panel>
      <q-tab-panel name='assets'>
        <submit-asset />
        <div :style='{margin: "32px 0"}'>
          <mint-nft />
        </div>
        <asset-application-list :style='{margin: "32px 0"}' />
        <collection-list :style='{margin: "32px 0"}' />
        <nft-list nft-type='MY_PUBLISHES' :style='{margin: "32px 0"}' />
        <nft-list nft-type='MY_ASSETS' :style='{margin: "32px 0"}' />
      </q-tab-panel>
      <q-tab-panel name='apply-reviewer'>
        <apply-reviewer />
      </q-tab-panel>
      <q-tab-panel name='foundation'>
        <div :style='{marginBottom: "16px"}'>
          <deposit-balance />
        </div>
        <q-separator />
        <div :style='{marginTop: "16px"}'>
          <foundation-page />
        </div>
      </q-tab-panel>
      <q-tab-panel name='activity'>
        <activity-page />
      </q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<script setup lang='ts'>
import { computed, onBeforeMount, onBeforeUnmount } from 'vue'
import { useSettingStore } from 'src/stores/setting'

import UserBalance from 'src/components/UserBalance.vue'
import SubmitAsset from 'src/components/SubmitAsset.vue'
import MintNft from 'src/components/MintNFT.vue'
import CollectionList from 'src/components/CollectionList.vue'
import NftList from 'src/components/NftList.vue'
import DepositBalance from 'src/components/DepositBalance.vue'
import AvatarSetting from 'src/components/AvatarSetting.vue'
import ReviewContents from 'src/components/ReviewContents.vue'
import ReviewAssets from 'src/components/ReviewAssets.vue'
import ReviewReviewers from 'src/components/ReviewReviewers.vue'
import ApplyReviewer from 'src/components/ApplyReviewer.vue'
import AssetApplicationList from 'src/components/AssetApplicationList.vue'
import FoundationPage from 'src/components/FoundationPage.vue'
import ActivityPage from 'src/components/ActivityPage.vue'
import ReviewActivities from 'src/components/ReviewActivities.vue'

import ContentsPage from 'src/components/dashboard/ContentsPage.vue'

const setting = useSettingStore()
const tab = computed(() => setting.currentDashboardTab)

onBeforeMount(() => {
  setting.showDrawerMenu = true
})

onBeforeUnmount(() => {
  setting.showDrawerMenu = false
})

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/ResPeer.png)
  border-radius: 50%
  background-size: cover
  background-repeat: no-repeat
</style>
