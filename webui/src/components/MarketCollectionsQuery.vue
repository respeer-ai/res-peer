<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useCollectionStore, Collection } from 'src/stores/collection'
import { computed, watch, ref } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const collection = useCollectionStore()
const collectionsKeys = computed(() => collection.collectionsKeys)
const collections = computed(() => collection.collections)
const collectionIndex = ref(-1)
const collectionKey = computed(() => collectionIndex.value >= 0 ? collectionsKeys.value[collectionIndex.value] : undefined)
const mutateKeys = computed(() => collection.mutateKeys)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const forceFetch = ref(false)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const marketApp = computed(() => application.marketApp)

const getCollection = (collectionKey: number, done?: () => void) => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getCollection($collectionKey: Int!) {
      collections {
        entry(key: $collectionKey) {
          value {
            price
            baseUri
            uris
            nfts
            collectionId
            name
            publisher
            createdAt
          }
        }
      }
    }
  `, {
    collectionKey: parseInt(collectionKey.toString()),
    endpoint: 'market',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const _collections = graphqlResult.data(res, 'collections')
    collections.value.set(collectionKey, graphqlResult.entryValue(_collections) as Collection)
    done?.()
  })
}

const getCollectionThroughCheCko = (collectionKey: number, done?: () => void) => {
  const query = gql`
    query getCollection($collectionKey: Int!) {
      collections {
        entry(key: $collectionKey) {
          value {
            price
            baseUri
            uris
            nfts
            collectionId
            name
            publisher
            createdAt
          }
        }
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: marketApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          collectionKey: parseInt(collectionKey.toString())
        },
        operationName: 'getCollection'
      }
    }
  }).then((result) => {
    const _collections = graphqlResult.data(result, 'collections')
    collections.value.set(collectionKey, graphqlResult.entryValue(_collections) as Collection)
    done?.()
  }).catch((e) => {
    console.log(e)
  })
}

watch(collectionKey, () => {
  if (!collectionKey.value) {
    return
  }
  const index = collection.mutateKeys.findIndex((el) => el === collectionKey.value)
  if (collections.value.get(collectionKey.value) && index < 0 && !forceFetch.value) {
    collectionIndex.value++
    return
  }
  if (cheCkoConnect.value) {
    getCollectionThroughCheCko(collectionKey.value, () => {
      collectionIndex.value++
    })
  } else {
    getCollection(collectionKey.value, () => {
      collectionIndex.value++
    })
  }
})

watch(targetChain, () => {
  if (collectionsKeys.value.length === 0) {
    return
  }
  forceFetch.value = false
  collectionIndex.value = 0
})

watch(collectionsKeys, () => {
  if (collectionsKeys.value.length === 0) {
    return
  }
  forceFetch.value = false
  collectionIndex.value = 0
})

watch(blockHeight, () => {
  if (collectionsKeys.value.length === 0) {
    return
  }
  forceFetch.value = true
  collectionIndex.value = 0
})

watch(mutateKeys, () => {
  if (collectionsKeys.value.length === 0) {
    return
  }
  forceFetch.value = false
  collectionIndex.value = 0
})

</script>
