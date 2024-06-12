<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
// import { useChainStore } from 'src/stores/chain'
import { computed, onMounted, watch } from 'vue'
import { useChainStore } from 'src/stores/chain'
import { useSettingStore } from 'src/stores/setting'

// const chain = useChainStore()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const chain = useChainStore()
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const getChains = () => {
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getChains {
      chains {
        list
        default
      }
    }
  `, {
    endpoint: 'main'
  }, {
    fetchPolicy: 'network-only'
  }))

  watch(result, () => {
    const r = result.value as Record<string, unknown>
    const chains = r.chains as Record<string, unknown>
    chain.chains = chains.list as Array<string>
    chain.defaultChain = chains.default as string
    chain.targetChain = chain.defaultChain
  })
}

onMounted(() => {
  if (!cheCkoConnect.value) {
    getChains()
  }
})

</script>
