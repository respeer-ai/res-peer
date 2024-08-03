<template>
  <q-layout view='lHh Lpr lFf'>
    <q-header elevated>
      <q-toolbar>
        <MainHeader />
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
      <chain-query />
      <request-application />
      <credit-query />
      <block-subscription />
      <feed-contents-keys-query />
      <feed-contents-query />
      <market-info-query />
      <market-collections-query />
      <request-feed-subscribe />
      <request-review-subscribe />
      <request-credit-subscribe />
      <request-foundation-subscribe />
      <request-market-subscribe />
      <request-activity-subscribe />
      <user-reviewer-query />
      <content-applications-keys-query />
      <content-applications-query />
      <asset-applications-keys-query />
      <asset-applications-query />
      <reviewer-applications-keys-query />
      <reviewer-applications-query />
      <review-state-query />
      <foundation-info-query />
      <activities-keys-query />
      <activities-query />
      <activity-applications-keys-query />
      <activity-applications-query />
      <native-balance-query />
    </q-page-container>

    <q-footer elevated :style='{height: "32px", lineHeight: "32px"}'>
      <MainFooter />
    </q-footer>
    <q-dialog
      v-model='logining'
      position='standard'
    >
      <q-card :style='{padding: "48px", width: "100%"}'>
        <div class='text-center text-brown-8 text-bold' :style='{fontSize: "20px"}'>
          Connecting Linera through CheCko...
        </div>
        <q-card :style='{height: "160px", width: "100%", padding: "48px"}' flat>
          <q-inner-loading
            :showing='logining'
            class='text-red-4'
          >
            <q-spinner-facebook size='80px' />
          </q-inner-loading>
        </q-card>
      </q-card>
    </q-dialog>
  </q-layout>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'
import { onBeforeMount, ref } from 'vue'
import { Cookies } from 'quasar'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'
import * as constants from 'src/const'

import MainHeader from 'src/components/header/MainHeader.vue'
import MainFooter from 'src/components/footer/MainFooter.vue'

import CreditQuery from 'src/components/CreditQuery.vue'
import BlockSubscription from 'src/components/BlockSubscription.vue'
import FeedContentsKeysQuery from 'src/components/FeedContentsKeysQuery.vue'
import FeedContentsQuery from 'src/components/FeedContentsQuery.vue'
import MarketInfoQuery from 'src/components/MarketInfoQuery.vue'
import MarketCollectionsQuery from 'src/components/MarketCollectionsQuery.vue'
import RequestApplication from 'src/components/RequestApplication.vue'
import RequestFeedSubscribe from 'src/components/RequestFeedSubscribe.vue'
import RequestCreditSubscribe from 'src/components/RequestCreditSubscribe.vue'
import RequestFoundationSubscribe from 'src/components/RequestFoundationSubscribe.vue'
import RequestMarketSubscribe from 'src/components/RequestMarketSubscribe.vue'
import RequestActivitySubscribe from 'src/components/RequestActivitySubscribe.vue'
import ChainQuery from 'src/components/ChainQuery.vue'
import UserReviewerQuery from 'src/components/UserReviewerQuery.vue'
import RequestReviewSubscribe from 'src/components/RequestReviewSubscribe.vue'
import ContentApplicationsKeysQuery from 'src/components/ContentApplicationsKeysQuery.vue'
import ContentApplicationsQuery from 'src/components/ContentApplicationsQuery.vue'
import AssetApplicationsKeysQuery from 'src/components/AssetApplicationsKeysQuery.vue'
import AssetApplicationsQuery from 'src/components/AssetApplicationsQuery.vue'
import ReviewerApplicationsKeysQuery from 'src/components/ReviewerApplicationsKeysQuery.vue'
import ReviewerApplicationsQuery from 'src/components/ReviewerApplicationsQuery.vue'
import ReviewStateQuery from 'src/components/ReviewStateQuery.vue'
import FoundationInfoQuery from 'src/components/FoundationInfoQuery.vue'
import ActivitiesKeysQuery from 'src/components/ActivitiesKeysQuery.vue'
import ActivitiesQuery from 'src/components/ActivitiesQuery.vue'
import ActivityApplicationsKeysQuery from 'src/components/ActivityApplicationsKeysQuery.vue'
import ActivityApplicationsQuery from 'src/components/ActivityApplicationsQuery.vue'
import NativeBalanceQuery from 'src/components/NativeBalanceQuery.vue'

const logining = ref(false)
const user = useUserStore()
const route = useRoute()
const setting = useSettingStore()

interface Query {
  port: number
  host: string
  cheCkoConnect?: boolean
}

const port = ref((route.query as unknown as Query).port || constants.port)
const host = ref((route.query as unknown as Query).host || constants.host)
const cheCkoConnect = ref(((route.query as unknown as Query).cheCkoConnect || 'true') === 'true')

const getProviderState = () => {
  window.linera.request({
    method: 'metamask_getProviderState'
  }).then((result) => {
    user.chainId = ((result as Record<string, string>).chainId).substring(2)
  }).catch((e) => {
    console.log('metamask_getProviderState', e)
  })
}

onBeforeMount(() => {
  Cookies.set('service-port', port.value.toString())
  Cookies.set('service-host', host.value.toString())
  Cookies.set('cheCkoConnect', cheCkoConnect.value === undefined ? 'true' : cheCkoConnect.value ? 'true' : 'false')

  user.account = Cookies.get('account')
  setting.cheCkoConnect = Cookies.get('cheCkoConnect') === 'true'

  if (setting.cheCkoConnect) {
    if (user.account?.length) {
      setTimeout(() => {
        getProviderState()
      }, 1000)
    }
  } else {
    user.account = constants.appDeployOwner
    user.chainId = constants.appDeployChain
  }
})
</script>

<style scoped lang="sass">
.q-layout__section--marginal
  background-color: white
</style>
