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
  return targetChain.value?.length > 0 && reviewApp.value?.length > 0
}

const getActivityApplicationsKeys = () => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getActivityApplicationsKeys {
      activityApplications {
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
    const activityApplications = graphqlResult.data(res, 'activityApplications')
    review.activityApplicationsKeys = graphqlResult.keys(activityApplications) as Array<number>
  })
}

watch(blockHeight, () => {
  if (!ready()) return
  getActivityApplicationsKeys()
})

watch(reviewApp, () => {
  if (!ready()) return
  getActivityApplicationsKeys()
})

watch(targetChain, () => {
  if (!ready()) return
  getActivityApplicationsKeys()
})

onMounted(() => {
  if (!ready()) return
  getActivityApplicationsKeys()
})

</script>
