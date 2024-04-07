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

const user = useUserStore()
const account = computed(() => user.account)
const application = useApplicationStore()
const marketApp = computed(() => application.marketApp)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = (): boolean => {
  return account.value?.length > 0 && marketApp.value?.length > 0 && targetChain.value?.length > 0
}

watch(targetChain, () => {
  if (!ready()) return
  getMarketInfo()
})

watch(marketApp, () => {
  if (!ready()) return
  getMarketInfo()
})

watch(account, () => {
  if (!ready()) return
  getMarketInfo()
})

watch(blockHeight, () => {
  if (!ready()) return
  getMarketInfo()
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

onMounted(() => {
  if (!ready()) return
  getMarketInfo()
})

</script>
