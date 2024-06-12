<script lang='ts' setup>
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { useApplicationStore } from 'src/stores/application'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'
import { targetChain } from 'src/stores/chain'
import { useBlockStore } from 'src/stores/block'
import { Reviewer } from 'src/stores/review'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'

const application = useApplicationStore()
const user = useUserStore()
const reviewApp = computed(() => application.reviewApp)
const account = computed(() => user.account)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = (): boolean => {
  return account.value?.length > 0 && reviewApp.value?.length > 0 && (cheCkoConnect.value || targetChain.value?.length > 0)
}

const userReviewerQuery = () => {
  const { /* result,  refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query reviewers($owner: String!) {
      reviewers {
        entry(key: $owner) {
          value {
            reviewer
          }
        }
      }
      reviewerApplications {
        entry(key: $owner) {
          value {
            chainId
            reviewer
            approved
            rejected
            reviewers
            resume
          }
        }
      }
    }
  `, {
    owner: account.value,
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const reviewers = graphqlResult.data(res, 'reviewers')
    if (graphqlResult.entryValue(reviewers)) user.reviewer = true
    const reviewerApplications = graphqlResult.data(res, 'reviewerApplications')
    user.reviewerApplication = graphqlResult.entryValue(reviewerApplications) as Reviewer
  })
}

const userReviewerQueryThroughCheCko = () => {
  const query = gql`
    query reviewers($owner: String!) {
      reviewers {
        entry(key: $owner) {
          value {
            reviewer
          }
        }
      }
      reviewerApplications {
        entry(key: $owner) {
          value {
            chainId
            reviewer
            approved
            rejected
            reviewers
            resume
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
          owner: account.value
        },
        operationName: 'reviewers'
      }
    }
  }).then((result) => {
    const reviewers = graphqlResult.keyValue(result, 'reviewers')
    if (graphqlResult.entryValue(reviewers)) user.reviewer = true
    const reviewerApplications = graphqlResult.keyValue(result, 'reviewerApplications')
    user.reviewerApplication = graphqlResult.entryValue(reviewerApplications) as Reviewer
  }).catch((e) => {
    console.log(e)
  })
}

watch(blockHeight, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    userReviewerQueryThroughCheCko()
  } else {
    userReviewerQuery()
  }
})

watch(account, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    userReviewerQueryThroughCheCko()
  } else {
    userReviewerQuery()
  }
})

watch(targetChain, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    userReviewerQueryThroughCheCko()
  } else {
    userReviewerQuery()
  }
})

watch(reviewApp, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    userReviewerQueryThroughCheCko()
  } else {
    userReviewerQuery()
  }
})

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    userReviewerQueryThroughCheCko()
  } else {
    userReviewerQuery()
  }
})

</script>
