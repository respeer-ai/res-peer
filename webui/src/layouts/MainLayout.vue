<template>
  <q-layout view='lHh Lpr lFf'>
    <q-header elevated>
      <q-toolbar>
        <div :style='{width: "720px", margin: "0 auto"}' class='flex justify-center items-center row'>
          <q-img
            src='~assets/ResPeer.png' width='160px' fit='contain' class='cursor-pointer'
            @click='onLogoClick'
          />
          <q-space />
          <q-icon
            name='store' size='24px' :color='tab == "store" ? "green" : "black"' class='cursor-pointer'
            @click='onNFTMarketClick'
          />
          <q-btn
            flat rounded class='bg-red-2'
            @click='onLoginClick'
            :style='{marginLeft: "8px"}'
          >
            <q-menu
              v-if='account?.length'
              :style='{padding: "24px"}'
              anchor='bottom right'
              self='top right'
            >
              <q-card flat>
                <div class='text-brown-10 row flex justify-center items-center' :style='{margin: "24px 0 0 0", lineHeight: "48px"}'>
                  <q-space />
                  <div class='text-bold' :style='{fontSize: "36px", marginLeft: "8px"}'>
                    {{ Number(accountBalance).toFixed(4) }}
                  </div>
                  <q-space />
                </div>
                <div class='row' :style='{margin: "8px 0 48px 0", lineHeight: "24px"}'>
                  <q-space />
                  <q-img
                    src='https://avatars.githubusercontent.com/u/107513858?s=48&v=4'
                    width='24px'
                    height='24px'
                  />
                  <div class='text-brown-6' :style='{fontSize: "24px", marginLeft: "8px"}'>
                    TLINERA
                  </div>
                  <q-space />
                </div>
                <q-separator :style='{margin: "8px 0"}' />
                <div class='row flex justify-center items-center'>
                  <div :style='{width: "48px"}'>
                    <q-icon name='person' size='24px' class='text-red-6' />
                  </div>
                  <div>
                    <div class='text-grey-6'>
                      Address
                    </div>
                    <div class='row'>
                      <div class='text-bold'>
                        {{ shortid.shortId(account, 16) }}
                      </div>
                      <div :style='{marginLeft: "8px"}'>
                        <q-icon name='content_copy' size='16px' class='text-grey-6' />
                      </div>
                    </div>
                    <div class='text-grey-6'>
                      {{ Number(accountBalance).toFixed(4) }}
                    </div>
                  </div>
                </div>
                <q-separator :style='{margin: "8px 0"}' />
                <div class='row flex justify-center items-center'>
                  <div :style='{width: "48px"}'>
                    <q-icon name='link' size='24px' class='text-red-6' />
                  </div>
                  <div>
                    <div class='text-grey-6'>
                      Microchain
                    </div>
                    <div class='row'>
                      <div class='text-bold'>
                        {{ shortid.shortId(chainId, 16) }}
                      </div>
                      <div :style='{marginLeft: "8px"}'>
                        <q-icon name='content_copy' size='16px' class='text-grey-6' />
                      </div>
                    </div>
                    <div class='text-grey-6'>
                      {{ Number(chainBalance).toFixed(4) }}
                    </div>
                  </div>
                </div>
                <q-btn
                  flat rounded class='bg-red-2 full-width'
                  @click='onLogoutClick'
                  label='Logout'
                  :style='{margin: "24px 0 24px 0"}'
                />
              </q-card>
            </q-menu>
            <q-img src='~assets/CheCko.png' width='24px' />
            <div :style='{margin: "2px 0 0 8px"}' class='text-brown-8 text-bold'>
              {{ account?.length ? shortid.shortId(account, 4) : 'Login' }}
            </div>
          </q-btn>
          <q-icon
            name='local_activity' size='24px' :color='tab == "activity" ? "green" : "black"' class='cursor-pointer'
            :style='{marginLeft: "8px"}'
            @click='onActivityClick'
          />
          <q-icon
            v-if='account?.length'
            name='dashboard' size='24px' :color='tab == "dashboard" ? "green" : "black"' class='cursor-pointer'
            :style='{marginLeft: "8px"}'
            @click='onDashboardClick'
          />
        </div>
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
      <div class='row' :style='{width: "720px", margin: "0 auto"}'>
        <q-img
          src='~assets/ResPeer.png' width='80px' fit='contain' class='cursor-pointer'
          @click='onLogoClick'
        />
        <q-space />
        <span class='text-grey-6'>Peer-to-Peer content publishing application on Linera</span>
        <q-space />
        <q-img
          src='https://avatars.githubusercontent.com/u/107513858?s=48&v=4'
          width='24px'
          height='24px'
          :style='{marginLeft: "8px", marginTop: "4px"}'
          class='cursor-pointer'
          @click='onGithubClick("https://github.com/linera-io/linera-protocol.git")'
        />
        <q-img
          src='https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSAPxlNRQziXOi61fD4jtkxAm-v6pPbT_UIF5IL1_PqCQ&s=10'
          width='24px'
          height='24px'
          :style='{marginLeft: "8px", marginTop: "4px"}'
          class='cursor-pointer'

          @click='onGithubClick("https://github.com/web3eye-io/res-peer.git")'
        />
      </div>
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
import { useRouter, useRoute } from 'vue-router'
import { onBeforeMount, ref, computed } from 'vue'
import { Cookies } from 'quasar'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'
import * as constants from 'src/const'
import { shortid } from 'src/utils'
import { Web3 } from 'web3'

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

const router = useRouter()
const logining = ref(false)
const user = useUserStore()
const route = useRoute()
const tab = ref('feed')
const account = computed(() => user.account?.trim())
const chainId = computed(() => user.chainId?.trim())
const accountBalance = computed(() => user.accountBalance)
const chainBalance = computed(() => user.chainBalance)
const setting = useSettingStore()

interface Query {
  port: number
  host: string
  cheCkoConnect?: boolean
}

const port = ref((route.query as unknown as Query).port || constants.port)
const host = ref((route.query as unknown as Query).host || constants.host)
const cheCkoConnect = ref(((route.query as unknown as Query).cheCkoConnect || 'true') === 'true')

const onGithubClick = (uri: string) => {
  window.open(uri)
}

const onDashboardClick = () => {
  tab.value = 'dashboard'
  void router.push({
    path: '/dashboard',
    query: {
      host: host.value,
      port: port.value
    }
  })
}

const onActivityClick = () => {
  tab.value = 'activity'
  void router.push({
    path: '/activities',
    query: {
      host: host.value,
      port: port.value
    }
  })
}

const onLogoClick = () => {
  tab.value = 'feed'
  void router.push({
    path: '/',
    query: {
      host: host.value,
      port: port.value
    }
  })
}

const getProviderState = () => {
  window.linera.request({
    method: 'metamask_getProviderState'
  }).then((result) => {
    user.chainId = ((result as Record<string, string>).chainId).substring(2)
    console.log(user.chainId, chainId.value)
  }).catch((e) => {
    console.log('metamask_getProviderState', e)
  })
}

const onLoginClick = () => {
  if (account.value.length) {
    return
  }
  if (!window.linera) {
    return window.open('https://github.com/respeer-ai/linera-wallet.git')
  }

  logining.value = true
  const web3 = new Web3(window.linera)
  web3.eth.requestAccounts()
    .then((accounts) => {
      logining.value = false
      if (accounts.length) {
        Cookies.set('account', accounts[0])
        user.account = accounts[0]
        getProviderState()
      }
    })
    .catch((e) => {
      logining.value = false
      console.log('eth_requestAccounts', e)
    })
}

const onLogoutClick = () => {
  Cookies.remove('account')
  user.$reset()
}

onBeforeMount(() => {
  Cookies.set('service-port', port.value.toString())
  Cookies.set('service-host', host.value.toString())
  Cookies.set('cheCkoConnect', cheCkoConnect.value === undefined ? 'true' : cheCkoConnect.value ? 'true' : 'false')

  user.account = Cookies.get('account')
  setting.cheCkoConnect = Cookies.get('cheCkoConnect') === 'true'

  if (user.account?.length) {
    setTimeout(() => {
      getProviderState()
    }, 1000)
  }
})

const onNFTMarketClick = () => {
  tab.value = 'store'
  void router.push({
    path: '/market',
    query: {
      host: host.value,
      port: port.value
    }
  })
}
</script>

<style scoped lang="sass">
.q-layout__section--marginal
  background-color: white
</style>
