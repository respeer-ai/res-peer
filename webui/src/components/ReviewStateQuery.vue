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

const ready = (): boolean => {
  return reviewApp.value?.length > 0 && (cheCkoConnect.value || targetChain.value?.length > 0)
}

const getReviewState = () => {
  const { /* result, */ refetch /*, fetchMore */, onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getReviewState {
      contentApprovedThreshold
      contentRejectedThreshold
      assetApprovedThreshold
      assetRejectedThreshold
      reviewerApprovedThreshold
      reviewerRejectedThreshold
      activityApprovedThreshold
      activityRejectedThreshold
      reviewerNumber
    }
  `, {
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  watch(blockHeight, () => {
    void refetch()
  })

  onResult((res) => {
    if (res.loading) return
    const ret = (res.data as Record<string, number>)
    review.contentApprovedThreshold = ret.contentApprovedThreshold
    review.contentRejectedThreshold = ret.contentRejectedThreshold
    review.assetApprovedThreshold = ret.assetApprovedThreshold
    review.assetRejectedThreshold = ret.assetRejectedThreshold
    review.reviewerApprovedThreshold = ret.reviewerApprovedThreshold
    review.reviewerRejectedThreshold = ret.reviewerRejectedThreshold
    review.activityApprovedThreshold = ret.activityApprovedThreshold
    review.activityRejectedThreshold = ret.activityRejectedThreshold
    review.reviewerNumber = ret.reviewerNumber
  })
}

const getReviewStateThroughCheCko = () => {
  const query = gql`
    query getReviewState {
      contentApprovedThreshold
      contentRejectedThreshold
      assetApprovedThreshold
      assetRejectedThreshold
      reviewerApprovedThreshold
      reviewerRejectedThreshold
      activityApprovedThreshold
      activityRejectedThreshold
      reviewerNumber
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: reviewApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {},
        operationName: 'getReviewState'
      }
    }
  }).then((result) => {
    const ret = (result as Record<string, number>)
    review.contentApprovedThreshold = ret.contentApprovedThreshold
    review.contentRejectedThreshold = ret.contentRejectedThreshold
    review.assetApprovedThreshold = ret.assetApprovedThreshold
    review.assetRejectedThreshold = ret.assetRejectedThreshold
    review.reviewerApprovedThreshold = ret.reviewerApprovedThreshold
    review.reviewerRejectedThreshold = ret.reviewerRejectedThreshold
    review.activityApprovedThreshold = ret.activityApprovedThreshold
    review.activityRejectedThreshold = ret.activityRejectedThreshold
    review.reviewerNumber = ret.reviewerNumber
  }).catch((e) => {
    console.log(e)
  })
}

watch(blockHeight, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getReviewStateThroughCheCko()
  } else {
    getReviewState()
  }
})

watch(targetChain, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getReviewStateThroughCheCko()
  } else {
    getReviewState()
  }
})

watch(reviewApp, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getReviewStateThroughCheCko()
  } else {
    getReviewState()
  }
})

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getReviewStateThroughCheCko()
  } else {
    getReviewState()
  }
})

</script>
