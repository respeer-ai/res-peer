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
    <div class='row' :style='{width: "480px"}'>
      <q-space />
      <div class='header-icon'>
        <q-icon
          name='bi-layout-wtf' size='24px' :color='tab == "store" ? "red-6" : "black"' class='cursor-pointer'
          @click='onNFTMarketClick'
        />
        <q-tooltip :offset='[0, 4]' class='bg-grey-2 text-grey-8 shadow-4'>
          NFT Market Place
        </q-tooltip>
      </div>
      <div class='header-icon'>
        <q-icon
          name='bi-columns-gap' size='24px' :color='tab == "activity" ? "red-6" : "black"' class='cursor-pointer'
          @click='onActivityClick'
        />
        <q-tooltip :offset='[0, 4]' class='bg-grey-2 text-grey-8 shadow-4'>
          Activity Center
        </q-tooltip>
      </div>
      <q-btn
        flat rounded class='bg-red-2'
        @click='onLoginClick'
        :style='{marginLeft: "16px"}'
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
                  <div :style='{marginLeft: "8px"}' class='cursor-pointer'>
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
                  <div :style='{marginLeft: "8px"}' class='cursor-pointer'>
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
              :style='{margin: "24px 0 0 0"}'
            />
            <div class='text-grey-6 text-center' :style='{margin: "8px 0 16px 0"}'>
              Powered by CheCko
            </div>
          </q-card>
        </q-menu>
        <q-img src='~assets/CheCko.png' width='24px' />
        <div :style='{margin: "2px 0 0 8px"}' class='text-brown-8 text-bold'>
          {{ account?.length ? shortid.shortId(account, 4) : 'Login' }}
        </div>
      </q-btn>
      <div class='header-icon' v-if='account?.length'>
        <q-icon
          name='bi-grid-1x2' size='24px' :color='tab == "dashboard" ? "red-6" : "black"' class='cursor-pointer'
          @click='onDashboardClick'
        />
        <q-tooltip :offset='[0, 4]' class='bg-grey-2 text-grey-8 shadow-4'>
          Dashboard
        </q-tooltip>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { ref, computed } from 'vue'
import { Cookies } from 'quasar'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'
import * as constants from 'src/const'
import { shortid } from 'src/utils'
import { Web3 } from 'web3'

import resPeerLogo from 'src/assets/ResPeer.png'

const router = useRouter()
const logining = ref(false)
const user = useUserStore()
const route = useRoute()
const account = computed(() => user.account?.trim())
const chainId = computed(() => user.chainId?.trim())
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
  tab.value = 'store'
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
