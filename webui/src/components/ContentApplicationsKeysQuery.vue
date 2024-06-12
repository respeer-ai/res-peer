<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { useReviewStore } from 'src/stores/review'
import { computed, onMounted, watch } from 'vue'
import { targetChain } from 'src/stores/chain'
import { useApplicationStore } from 'src/stores/application'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const review = useReviewStore()
const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && reviewApp.value?.length > 0
}

const getContentApplicationsKeys = () => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContentApplicationsKeys {
      contentApplications {
        keys
      }
    }
  `, {
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const contentApplications = graphqlResult.data(res, 'contentApplications')
    review.contentApplicationsKeys = graphqlResult.keys(contentApplications) as Array<string>
  })
}

const getContentApplicationsKeysThroughCheCko = () => {
  const query = gql`
    query getContentApplicationsKeys {
      contentApplications {
        keys
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: reviewApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {},
        operationName: 'getContentApplicationsKeys'
      }
    }
  }).then((result) => {
    const contentApplications = graphqlResult.keyValue(result, 'contentApplications')
    review.contentApplicationsKeys = graphqlResult.keyValue(contentApplications, 'keys') as Array<string>
  }).catch((e) => {
    console.log(e)
  })
}

watch(blockHeight, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getContentApplicationsKeysThroughCheCko()
  } else {
    getContentApplicationsKeys()
  }
})

watch(reviewApp, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getContentApplicationsKeysThroughCheCko()
  } else {
    getContentApplicationsKeys()
  }
})

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getContentApplicationsKeysThroughCheCko()
  } else {
    getContentApplicationsKeys()
  }
})

</script>
