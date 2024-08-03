<template>
  <div class='row' :style='{margin: "0 16px"}'>
    <q-img
      :src='resPeerLogo' width='80px' fit='contain' class='cursor-pointer'
      @click='onLogoClick'
    />
    <q-space />
    <span class='text-grey-6'>Earn Linera token with your Creativity</span>
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
</template>

<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { ref } from 'vue'
import { useSettingStore } from 'src/stores/setting'
import * as constants from 'src/const'

import resPeerLogo from 'src/assets/ResPeer.png'

const router = useRouter()
const route = useRoute()
const setting = useSettingStore()

interface Query {
  port: number
  host: string
  cheCkoConnect?: boolean
}

const port = ref((route.query as unknown as Query).port || constants.port)
const host = ref((route.query as unknown as Query).host || constants.host)

const onGithubClick = (uri: string) => {
  window.open(uri)
}

const onLogoClick = () => {
  setting.currentTab = 'feed'
  void router.push({
    path: '/',
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
