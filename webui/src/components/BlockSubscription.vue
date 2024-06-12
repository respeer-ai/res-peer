<script setup lang="ts">
import { provideApolloClient, useSubscription } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'
import { onMounted, onUnmounted, watch, ref, computed } from 'vue'
import { useSettingStore } from 'src/stores/setting'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const block = useBlockStore()
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = () => {
  return cheCkoConnect.value || targetChain.value?.length > 0
}

const subscribe = () => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useSubscription(gql`
    subscription notifications($chainId: String!) {
      notifications(chainId: $chainId)
    }
  `, {
    chainId: targetChain.value
  }))

  onResult((res) => {
    const data = res.data as Record<string, Record<string, Record<string, Record<string, unknown>>>>
    if (data.notifications.reason.NewBlock) {
      block.blockHeight = data.notifications.reason.NewBlock.height as number
      block.blockHash = data.notifications.reason.NewBlock.hash as string
    }
  })
}

watch(targetChain, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    subscribeThroughCheCko()
  } else {
    subscribe()
  }
})

const subscriptionHandler = (msg: unknown) => {
  const _msg = msg as Record<string, Record<string, Record<string, Record<string, Record<string, Record<string, unknown>>>>>>
  const data = _msg.data.result
  if (data.notifications.reason.NewBlock) {
    block.blockHeight = data.notifications.reason.NewBlock.height as number
    block.blockHash = data.notifications.reason.NewBlock.hash as string
  }
}

const subscriptionId = ref('')

const subscribeThroughCheCko = () => {
  window.linera.request({
    method: 'linera_subscribe'
  }).then((_subscriptionId) => {
    subscriptionId.value = _subscriptionId as string
    window.linera.on('message', subscriptionHandler)
  }).catch((e) => {
    console.log('Fail subscribe', e)
  })
}

const unsubscribeThroughCheCko = () => {
  void window.linera.request({
    method: 'linera_unsubscribe',
    params: [subscriptionId.value]
  })
}

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    setTimeout(() => {
      subscribeThroughCheCko()
    }, 500)
  } else {
    subscribe()
  }
})

onUnmounted(() => {
  if (cheCkoConnect.value) {
    unsubscribeThroughCheCko()
  }
})

</script>
