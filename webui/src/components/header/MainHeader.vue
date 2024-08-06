<template>
  <div :style='{width: "100%", margin: "0 auto"}' class='flex justify-center items-center row'>
    <div :style='{borderRadius: "18px"}' class='input-background'>
      <q-input
        v-model='setting.searchTextFilter'
        debounce='500'
        rounded
        outlined
        placeholder='Title / Content / Comment'
        :style='{width: "400px"}'
        color='transparent'
      >
        <template #prepend>
          <q-icon name='bi-filter-left' size='24px' color='red-4' />
        </template>
      </q-input>
    </div>
    <q-space />
    <q-img
      :src='resPeerLogo' width='160px' height='36px' fit='contain'
      class='cursor-pointer'
      @click='onLogoClick'
    />
    <q-space />
    <div class='row' :style='{width: "400px"}'>
      <q-space />
      <div class='header-icon row'>
        <inline-svg
          :src='marketPlaceIcon'
          width='24'
          height='24'
          :class='[ "item-icon cursor-pointer", tab === "market-place" ? "item-icon-active" : "" ]'
          @click='onNFTMarketClick'
        />
        <q-tooltip :offset='[0, 4]' class='bg-grey-2 text-grey-8 shadow-4'>
          NFT Market Place
        </q-tooltip>
      </div>
      <div class='header-icon'>
        <inline-svg
          :src='computingRegistryIcon'
          width='24'
          height='24'
          :class='[ "item-icon cursor-pointer", tab === "computing-registry" ? "item-icon-active" : "" ]'
          @click='onComputingMarketClick'
        />
        <q-tooltip :offset='[0, 4]' class='bg-grey-2 text-grey-8 shadow-4'>
          Computing Registry
        </q-tooltip>
      </div>
      <div class='header-icon'>
        <inline-svg
          :src='activityCenterIcon'
          width='24'
          height='24'
          :class='[ "item-icon cursor-pointer", tab === "activity" ? "item-icon-active" : "" ]'
          @click='onActivityClick'
        />
        <q-tooltip :offset='[0, 4]' class='bg-grey-2 text-grey-8 shadow-4'>
          Activity Center
        </q-tooltip>
      </div>
      <q-btn
        flat rounded class='bg-red-1'
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
            <div class='row flex justify-center items-center' :style='{margin: "12px 0 36px 0", fontSize: "28px"}'>
              <q-space />
              <div :style='{marginLeft: "8px"}'>
                {{ Number(accountBalance).toFixed(4) }}
              </div>
              <div :style='{margin: "8px 0 0 8px", fontSize: "12px"}'>
                TLINERA
              </div>
              <q-space />
            </div>
            <q-separator :style='{margin: "0 0 16px 0"}' />
            <div class='row'>
              <div :style='{width: "24px"}'>
                <q-img :src='addressIcon' width='16px' height='16px' />
              </div>
              <div>
                <div class='text-grey-6'>
                  Address
                </div>
                <div class='row'>
                  <div class='text-bold'>
                    {{ shortid.shortId(account, 14) }}
                  </div>
                  <div :style='{marginLeft: "8px"}' class='cursor-pointer'>
                    <q-img :src='copyIcon' width='16px' height='16px' />
                  </div>
                </div>
                <div class='text-grey-6'>
                  {{ Number(accountBalance).toFixed(4) }}
                </div>
              </div>
            </div>
            <div class='row' :style='{margin: "12px 0 0 0"}'>
              <div :style='{width: "24px"}'>
                <q-img :src='microchainIcon' width='16px' height='16px' />
              </div>
              <div>
                <div class='text-grey-6'>
                  Microchain
                </div>
                <div class='row'>
                  <div class='text-bold'>
                    {{ shortid.shortId(chainId, 14) }}
                  </div>
                  <div :style='{marginLeft: "8px"}' class='cursor-pointer'>
                    <q-img :src='copyIcon' width='16px' height='16px' />
                  </div>
                </div>
                <div class='text-grey-6'>
                  {{ Number(chainBalance).toFixed(4) }}
                </div>
              </div>
            </div>
            <q-btn
              flat rounded class='bg-red-6 full-width text-white'
              @click='onLogoutClick'
              label='Logout'
              :style='{margin: "24px 0 0 0"}'
            />
            <div class='text-grey-6 text-center' :style='{margin: "8px 0 16px 0", fontSize: "12px"}'>
              Powered by CheCko
            </div>
          </q-card>
        </q-menu>
        <q-img src='https://avatars.githubusercontent.com/u/107513858?s=48&v=4' width='24px' height='24px' />
        <div :style='{margin: "2px 0 0 8px"}' class='text-brown-8 text-bold'>
          {{ account?.length ? shortid.shortId(account, 4) : 'Login' }}
        </div>
      </q-btn>
      <q-avatar
        v-if='account?.length' class='cursor-pointer avatar'
        @click='onDashboardClick'
      >
        <q-img width='100%' height='100%' :src='avatar' />
        <q-menu
          v-if='account?.length'
          :style='{padding: "24px", width: "280px"}'
          anchor='bottom right'
          self='top right'
        >
          <q-card flat>
            <div class='text-center'>
              <q-avatar size='80px' class='cursor-pointer avatar'>
                <q-img :src='avatar' width='100%' height='100%' fit='contain' />
              </q-avatar>
              <div :style='{fontWeight: 600, fontSize: "16px", lineHeight: "20px", margin: "16px 0 0 0"}'>
                {{ username }}
              </div>
              <div :style='{fontSize: "10px", color: "rgba(26, 26, 26, 0.6)"}'>
                {{ spendableCredits?.length ? spendableCredits : 0 }} CREDITS, {{ lineraBalance?.length ? lineraBalance : 0 }} TLINERA
              </div>
            </div>
            <q-separator :style='{margin: "16px 0"}' />
            <div class='header-dashboard-menu cursor-pointer'>
              Profile
            </div>
            <div class='header-dashboard-menu cursor-pointer'>
              Contents
            </div>
            <div class='header-dashboard-menu cursor-pointer'>
              Assets
            </div>
            <div class='header-dashboard-menu cursor-pointer'>
              Earnings
            </div>
            <div class='header-dashboard-menu cursor-pointer'>
              Reviewer DAO
            </div>
            <div class='header-dashboard-menu cursor-pointer'>
              Foundation
            </div>
            <div class='header-dashboard-menu cursor-pointer'>
              Users Club
            </div>
            <div class='header-dashboard-menu cursor-pointer'>
              Settings
            </div>
            <q-separator :style='{margin: "16px 0"}' />
            <div class='header-dashboard-menu cursor-pointer'>
              Logout
            </div>
          </q-card>
        </q-menu>
      </q-avatar>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { ref, computed } from 'vue'
import { Cookies } from 'quasar'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'
import { useCollectionStore } from 'src/stores/collection'
import { useFoundationStore } from 'src/stores/foundation'
import * as constants from 'src/const'
import { shortid } from 'src/utils'
import { Web3 } from 'web3'
import { addressIcon, microchainIcon, copyIcon } from 'src/assets'

import resPeerLogo from 'src/assets/ResPeer.png'
import marketPlaceIcon from 'src/assets/MarketPlaceIcon.svg'
import computingRegistryIcon from 'src/assets/ComputingRegistryIcon.svg'
import activityCenterIcon from 'src/assets/ActivityCenterIcon.svg'
import resPeerFavoriteIcon from 'src/assets/ResPeerFavoriteIcon.png'

const router = useRouter()
const logining = ref(false)
const user = useUserStore()
const route = useRoute()
const account = computed(() => user.account?.trim())
const chainId = computed(() => user.chainId?.trim())
const username = computed(() => user.username || 'ResPeer User')
const spendableCredits = computed(() => user.spendable)
const foundation = useFoundationStore()
const lineraBalance = computed(() => foundation.userLineraBalance)
const collection = useCollectionStore()
const avatar = computed(() => userAvatar(user.account))
const accountBalance = computed(() => user.accountBalance)
const chainBalance = computed(() => user.chainBalance)
const setting = useSettingStore()
const tab = computed({
  get: () => setting.currentMainTab,
  set: (v: string) => {
    setting.currentMainTab = v
  }
})

interface Query {
  port: number
  host: string
  cheCkoConnect?: boolean
}

const port = ref((route.query as unknown as Query).port || constants.port)
const host = ref((route.query as unknown as Query).host || constants.host)
const cheCkoConnect = ref(((route.query as unknown as Query).cheCkoConnect || 'true') === 'true')

const userAvatar = (account: string) => {
  const ids = collection.avatars.get(account)
  if (!ids) {
    return collection.nftBannerByID(1001, 1000) || resPeerFavoriteIcon
  }
  return collection.nftBannerByID(ids[0], ids[1]) || resPeerFavoriteIcon
}

const onDashboardClick = () => {
  tab.value = 'dashboard'
  void router.push({
    path: '/dashboard',
    query: {
      host: host.value,
      port: port.value,
      cheCkoConnect: cheCkoConnect.value ? 'true' : 'false'
    }
  })
}

const onComputingMarketClick = () => {
  tab.value = 'computing'
  void router.push({
    path: '/computing',
    query: {
      host: host.value,
      port: port.value,
      cheCkoConnect: cheCkoConnect.value ? 'true' : 'false'
    }
  })
}

const onActivityClick = () => {
  tab.value = 'activity'
  void router.push({
    path: '/activities',
    query: {
      host: host.value,
      port: port.value,
      cheCkoConnect: cheCkoConnect.value ? 'true' : 'false'
    }
  })
}

const onLogoClick = () => {
  tab.value = 'feed'
  void router.push({
    path: '/',
    query: {
      host: host.value,
      port: port.value,
      cheCkoConnect: cheCkoConnect.value ? 'true' : 'false'
    }
  })
}

const getProviderState = () => {
  window.linera.request({
    method: 'metamask_getProviderState'
  }).then((result) => {
    user.chainId = ((result as Record<string, string>).chainId).substring(2)
  }).catch((e) => {
    console.log('metamask_getProviderState', e)
  })
}

const onLoginClick = () => {
  if (account.value?.length) {
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

const onNFTMarketClick = () => {
  tab.value = 'market-place'
  void router.push({
    path: '/market',
    query: {
      host: host.value,
      port: port.value,
      cheCkoConnect: cheCkoConnect.value ? 'true' : 'false'
    }
  })
}
</script>

<style scoped lang="sass">
.q-layout__section--marginal
  background-color: white
</style>
