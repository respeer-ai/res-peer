<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { useContentStore } from 'src/stores/content'
import { computed, onMounted, watch } from 'vue'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const content = useContentStore()
const application = useApplicationStore()
const feedApp = computed(() => application.feedApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = () => {
  return feedApp.value?.length && (cheCkoConnect.value || targetChain.value?.length)
}

const getContentsKeys = () => {
  const { /* result, */ refetch /*, fetchMore */, onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContentsKeys {
      contents {
        keys
      }
    }
  `, {
    endpoint: 'feed',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  // TODO: is it still work
  watch(blockHeight, () => {
    void refetch()
  })

  onResult((res) => {
    if (res.loading) return
    const contents = graphqlResult.data(res, 'contents')
    content.contentsKeys = graphqlResult.keys(contents) as Array<string>
  })
}

const getContentsKeysThroughCheCko = () => {
  const query = gql`
    query getContentsKeys {
      contents {
        keys
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: feedApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {},
        operationName: 'getContentsKeys'
      }
    }
  }).then((result) => {
    const contents = graphqlResult.keyValue(result, 'contents')
    content.contentsKeys = graphqlResult.keys(contents) as Array<string>
  }).catch((e) => {
    console.log(e)
  })
}

watch(blockHeight, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getContentsKeysThroughCheCko()
  } else {
    getContentsKeys()
  }
})

watch(feedApp, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getContentsKeysThroughCheCko()
  } else {
    getContentsKeys()
  }
})

watch(targetChain, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getContentsKeysThroughCheCko()
  } else {
    getContentsKeys()
  }
})

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getContentsKeysThroughCheCko()
  } else {
    getContentsKeys()
  }
})

</script>
