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

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const review = useReviewStore()
const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return /* targetChain.value?.length > 0 && */ reviewApp.value?.length > 0
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
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
  getContentApplicationsKeysThroughCheCko()
})

watch(reviewApp, () => {
  if (!ready()) return
  getContentApplicationsKeysThroughCheCko()
})

onMounted(() => {
  if (!ready()) return
  getContentApplicationsKeysThroughCheCko()
})

</script>
