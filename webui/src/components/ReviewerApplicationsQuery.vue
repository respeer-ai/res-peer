<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useReviewStore, Reviewer } from 'src/stores/review'
import { computed, watch, ref, onMounted } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const review = useReviewStore()
const reviewerApplicationsKeys = computed(() => review.reviewerApplicationsKeys)
const reviewerApplications = computed(() => review.reviewerApplications)
const reviewerMutateKeys = computed(() => review.reviewerMutateKeys)
const reviewerIndex = ref(-1)
const reviewerApplicationKey = computed(() => reviewerIndex.value >= 0 ? reviewerApplicationsKeys.value[reviewerIndex.value] : undefined)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getReviewerApplication = (reviewerApplicationKey: string, done?: () => void) => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getReviewerApplication($reviewerApplicationKey: String!) {
      reviewerApplications {
        entry(key: $reviewerApplicationKey) {
          value {
            chainId
            reviewer
            resume
            reviewers
            approved
            rejected
            createdAt
          }
        }
      }
    }
  `, {
    reviewerApplicationKey: `${reviewerApplicationKey}`,
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const _reviewerApplications = graphqlResult.data(res, 'reviewerApplications')
    reviewerApplications.value.set(reviewerApplicationKey, graphqlResult.entryValue(_reviewerApplications) as Reviewer)
    done?.()
  })
}

const getReviewerApplicationThroughCheCko = (reviewerApplicationKey: string, done?: () => void) => {
  const query = gql`
    query getReviewerApplication($reviewerApplicationKey: String!) {
      reviewerApplications {
        entry(key: $reviewerApplicationKey) {
          value {
            chainId
            reviewer
            resume
            reviewers
            approved
            rejected
            createdAt
          }
        }
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: reviewApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          reviewerApplicationKey: `${reviewerApplicationKey}`
        },
        operationName: 'getReviewerApplication'
      }
    }
  }).then((result) => {
    const _reviewerApplications = graphqlResult.keyValue(result, 'reviewerApplications')
    reviewerApplications.value.set(reviewerApplicationKey, graphqlResult.entryValue(_reviewerApplications) as Reviewer)
    done?.()
  }).catch((e) => {
    console.log(e)
  })
}

watch(reviewerApplicationKey, () => {
  if (!reviewerApplicationKey.value) {
    return
  }

  const index = reviewerMutateKeys.value.findIndex((el) => el === reviewerApplicationKey.value)
  if (reviewerApplications.value.get(reviewerApplicationKey.value) && index < 0) {
    reviewerIndex.value++
    return
  }

  if (cheCkoConnect.value) {
    getReviewerApplicationThroughCheCko(reviewerApplicationKey.value, () => {
      reviewerIndex.value++
      reviewerMutateKeys.value.splice(index, 1)
    })
  } else {
    getReviewerApplication(reviewerApplicationKey.value, () => {
      reviewerIndex.value++
      reviewerMutateKeys.value.splice(index, 1)
    })
  }
})

watch(reviewerApplicationsKeys, () => {
  if (reviewerApplicationsKeys.value.length === 0) {
    return
  }
  reviewerIndex.value = 0
})

watch(blockHeight, () => {
  if (reviewerApplicationsKeys.value.length === 0) {
    return
  }
  reviewerIndex.value = 0
})

onMounted(() => {
  reviewerMutateKeys.value.forEach((reviewerKey) => {
    if (cheCkoConnect.value) {
      getReviewerApplicationThroughCheCko(reviewerKey)
    } else {
      getReviewerApplication(reviewerKey)
    }
  })
  review.reviewerMutateKeys = []
})

</script>
