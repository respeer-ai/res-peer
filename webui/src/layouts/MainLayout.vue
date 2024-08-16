<template>
  <q-layout view='hHh Lpr hFf'>
    <q-header bordered>
      <q-toolbar>
        <MainHeader />
      </q-toolbar>
    </q-header>

    <q-drawer
      v-if='drawerOpen'
      :width='280'
      show-if-above
      :breakpoint='500'
      bordered
    >
      <MainDrawer />
    </q-drawer>

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
      <RequestCPRegistrySubscribe />
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
      <computing-nodes-query />

      <q-page-sticky position='bottom-right' :offset='[18, 18]'>
        <div :style='{width: "56px"}'>
          <q-btn fab color='red-6' @click='onWriteClick'>
            <inline-svg
              :src='writeIcon'
              width='20'
              height='20'
              :style='{color: "white"}'
            />
          </q-btn>
          <q-btn outline fab color='red-6' :style='{marginTop: "16px"}'>
            <inline-svg
              :src='createIcon'
              width='20'
              height='20'
              :style='{color: "red-6"}'
            />
          </q-btn>
        </div>
        <div :style='{marginTop: "24px"}' />
      </q-page-sticky>
    </q-page-container>

    <q-footer bordered :style='{height: "32px", lineHeight: "32px"}'>
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
import { useRoute, useRouter } from 'vue-router'
import { computed, onBeforeMount, ref, watch } from 'vue'
import { Cookies } from 'quasar'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'
import * as constants from 'src/const'

import MainHeader from 'src/components/header/MainHeader.vue'
import MainFooter from 'src/components/footer/MainFooter.vue'
import MainDrawer from 'src/components/drawer/MainDrawer.vue'

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
import RequestCPRegistrySubscribe from 'src/components/RequestCPRegistrySubscribe.vue'
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
import ComputingNodesQuery from 'src/components/ComputingNodesQuery.vue'
import { createIcon, writeIcon } from 'src/assets'

const logining = ref(false)
const user = useUserStore()
const route = useRoute()
const setting = useSettingStore()
const drawerOpen = computed(() => setting.showDrawerMenu)

interface Query {
  port: number
  host: string
  cheCkoConnect?: boolean
  owner?: string
  chainId?: string
}

const port = ref((route.query as unknown as Query).port || constants.port)
const host = ref((route.query as unknown as Query).host || constants.host)
const cheCkoConnect = ref(((route.query as unknown as Query).cheCkoConnect || 'true') === 'true')
const owner = ref((route.query as unknown as Query).owner || constants.appDeployOwner)
const chainId = ref((route.query as unknown as Query).chainId || constants.appDeployChain)

const getProviderState = () => {
  window.linera?.request({
    method: 'metamask_getProviderState'
  }).then((result) => {
    user.chainId = ((result as Record<string, string>).chainId).substring(2)
  }).catch((e) => {
    console.log('metamask_getProviderState', e)
  })
}

watch(() => window.linera, () => {
  if (setting.cheCkoConnect) {
    if (user.account?.length) {
      setTimeout(() => {
        getProviderState()
      }, 1000)
    }
  }
})

onBeforeMount(() => {
  Cookies.set('service-port', port.value.toString(), { path: '/' })
  Cookies.set('service-host', host.value.toString(), { path: '/' })
  Cookies.set('cheCkoConnect', cheCkoConnect.value === undefined ? 'true' : cheCkoConnect.value ? 'true' : 'false', { path: '/' })

  user.account = Cookies.get('account')
  setting.cheCkoConnect = Cookies.get('cheCkoConnect') === 'true'

  if (setting.cheCkoConnect) {
    if (user.account?.length) {
      setTimeout(() => {
        getProviderState()
      }, 1000)
    }
  } else {
    user.account = owner.value
    user.chainId = chainId.value
  }
})

const router = useRouter()

const onWriteClick = () => {
  setting.currentDashboardTab = 'contents'
  void router.push({
    path: 'dashboard',
    query: {
      host: Cookies.get('service-host'),
      port: Cookies.get('service-port'),
      cheCkoConnect: Cookies.get('cheCkoConnect'),
      write: 'true'
    }
  })
}

</script>

<style scoped lang="sass">
.q-layout__section--marginal
  background-color: white
</style>
