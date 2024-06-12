<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useCollectionStore } from 'src/stores/collection'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'

const user = useUserStore()
const account = computed(() => user.account)
const application = useApplicationStore()
const marketApp = computed(() => application.marketApp)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = (): boolean => {
  return account.value?.length > 0 && marketApp.value?.length > 0 && (cheCkoConnect.value || targetChain.value?.length > 0)
}

watch(targetChain, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getMarketInfoThroughCheCko()
  } else {
    getMarketInfo()
  }
})

watch(marketApp, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getMarketInfoThroughCheCko()
  } else {
    getMarketInfo()
  }
})

watch(account, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getMarketInfoThroughCheCko()
  } else {
    getMarketInfo()
  }
})

watch(blockHeight, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getMarketInfoThroughCheCko()
  } else {
    getMarketInfo()
  }
})

const collection = useCollectionStore()

const getMarketInfo = () => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => {
    if (account.value) {
      return useQuery(gql`
        query getMarketInfo($account: String!) {
          collections {
            keys
          }
          creditsPerLinera
          tradeFeePercent
          maxCreditsPercent
          assets {
            entry(key: $account) {
              value
            }
          }
          avatars {
            entry(key: $account) {
              value
            }
          }
        }
      `, {
        account: `${account.value}`,
        endpoint: 'market',
        chainId: targetChain.value
      }, {
        fetchPolicy: 'network-only'
      })
    }
    return useQuery(gql`
        query getMarketInfo {
          collections {
            keys
          }
          creditsPerLinera
          tradeFeePercent
          maxCreditsPercent
        }
      `, {
      account: `${account.value}`,
      endpoint: 'market',
      chainId: targetChain.value
    }, {
      fetchPolicy: 'network-only'
    })
  })

  onResult((res) => {
    if (res.loading) return
    const collections = graphqlResult.data(res, 'collections')
    collection.collectionsKeys = graphqlResult.keys(collections) as Array<number>
    collection.creditsPerLinera = graphqlResult.keyValue(res, 'creditsPerLinera') as string
    collection.maxCreditsPercent = graphqlResult.keyValue(res, 'maxCreditsPercent') as number
    collection.tradeFeePercent = graphqlResult.keyValue(res, 'maxCreditsPercent') as number
    const _assets = graphqlResult.data(res, 'assets')
    const assets = graphqlResult.entryValue(_assets) as Record<number, Array<number>>
    if (assets) {
      Object.keys(assets).forEach((key, index) => {
        collection.assets.set(parseInt(key), Object.values(assets)[index])
      })
    }
    const _avatar = graphqlResult.data(res, 'avatars')
    collection.avatars.set(account.value, graphqlResult.entryValue(_avatar) as Array<number>)
  })
}

const getMarketInfoThroughCheCko = () => {
  const query = (() => {
    if (account.value) {
      return gql`
        query getMarketInfo($account: String!) {
          collections {
            keys
          }
          creditsPerLinera
          tradeFeePercent
          maxCreditsPercent
          assets {
            entry(key: $account) {
              value
            }
          }
          avatars {
            entry(key: $account) {
              value
            }
          }
        }`
    }
    return gql`
      query getMarketInfo {
        collections {
          keys
        }
        creditsPerLinera
        tradeFeePercent
        maxCreditsPercent
      }`
  })()

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: marketApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          account: `${account.value}`
        },
        operationName: 'getMarketInfo'
      }
    }
  }).then((result) => {
    const collections = graphqlResult.keyValue(result, 'collections')
    collection.collectionsKeys = graphqlResult.keys(collections) as Array<number>
    collection.creditsPerLinera = graphqlResult.keyValue(result, 'creditsPerLinera') as string
    collection.maxCreditsPercent = graphqlResult.keyValue(result, 'maxCreditsPercent') as number
    collection.tradeFeePercent = graphqlResult.keyValue(result, 'maxCreditsPercent') as number
    const _assets = graphqlResult.keyValue(result, 'assets')
    const assets = graphqlResult.entryValue(_assets) as Record<number, Array<number>>
    if (assets) {
      Object.keys(assets).forEach((key, index) => {
        collection.assets.set(parseInt(key), Object.values(assets)[index])
      })
    }
    const _avatar = graphqlResult.keyValue(result, 'avatars')
    collection.avatars.set(account.value, graphqlResult.entryValue(_avatar) as Array<number>)
  }).catch((e) => {
    console.log(e)
  })
}

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getMarketInfoThroughCheCko()
  } else {
    getMarketInfo()
  }
})

</script>
