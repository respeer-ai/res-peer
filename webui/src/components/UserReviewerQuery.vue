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

const application = useApplicationStore()
const user = useUserStore()
const reviewApp = computed(() => application.reviewApp)
const account = computed(() => user.account)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = (): boolean => {
  return account.value?.length > 0 && reviewApp.value?.length > 0 && targetChain.value?.length > 0
}

const userReviewerQuery = () => {
  const { result /*, refetch, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query reviewers($owner: String!) {
      reviewers(owner: $owner) {
        reviewer
      }
    }
  `, {
    owner: account.value,
    endpoint: 'review',
    chainId: targetChain.value
  }))

  watch(result, () => {
    const ret = result.value as Record<string, string>
    if (ret.reviewers) user.reviewer = true
  })
}

watch(blockHeight, () => {
  if (!ready()) return
  userReviewerQuery()
})

watch(account, () => {
  if (!ready()) return
  userReviewerQuery()
})

watch(targetChain, () => {
  if (!ready()) return
  userReviewerQuery()
})

watch(reviewApp, () => {
  if (!ready()) return
  userReviewerQuery()
})

onMounted(() => {
  if (!ready()) return
  userReviewerQuery()
})

</script>