<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'
import { useFoundationStore } from 'src/stores/foundation'
import { graphqlResult } from 'src/utils'

const user = useUserStore()
const account = computed(() => user.account)
const application = useApplicationStore()
const foundationApp = computed(() => application.foundationApp)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const foundation = useFoundationStore()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = (): boolean => {
  return account.value?.length > 0 && foundationApp.value?.length > 0 && targetChain.value?.length > 0
}

watch(targetChain, () => {
  if (!ready()) return
  getFoundationInfo()
})

watch(foundationApp, () => {
  if (!ready()) return
  getFoundationInfo()
})

watch(account, () => {
  if (!ready()) return
  getFoundationInfo()
})

watch(blockHeight, () => {
  if (!ready()) return
  getFoundationInfo()
})

const getFoundationInfo = () => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => {
    if (account.value) {
      return useQuery(gql`
        query getFoundationInfo($account: String!) {
          foundationBalance
          reviewRewardPercent
          reviewRewardBalance
          reviewRewardFactor
          authorRewardPercent
          authorRewardBalance
          authorRewardFactor
          activityRewardPercent
          activityRewardBalance
          userBalances {
            entry(key: $account) {
              value
            }
          }
        }
      `, {
        account: `${account.value}`,
        endpoint: 'foundation',
        chainId: targetChain.value
      }, {
        fetchPolicy: 'network-only'
      })
    }
    return useQuery(gql`
        query getFoundationInfo {
          foundationBalance
          reviewRewardPercent
          reviewRewardBalance
          reviewRewardFactor
          authorRewardPercent
          authorRewardBalance
          authorRewardFactor
          activityRewardPercent
          activityRewardBalance
        }
      `, {
      endpoint: 'foundation',
      chainId: targetChain.value
    }, {
      fetchPolicy: 'network-only'
    })
  })

  onResult((res) => {
    if (res.loading) return
    const userBalances = graphqlResult.data(res, 'userBalances')
    foundation.userLineraBalance = graphqlResult.entryValue(userBalances) as string
    foundation.foundationBalance = graphqlResult.keyValue(res, 'foundationBalance') as string
    foundation.reviewRewardBalance = graphqlResult.keyValue(res, 'reviewRewardBalance') as string
    foundation.authorRewardBalance = graphqlResult.keyValue(res, 'reviewRewardBalance') as string
    foundation.activityRewardBalance = graphqlResult.keyValue(res, 'reviewRewardBalance') as string
    foundation.reviewRewardPercent = graphqlResult.keyValue(res, 'reviewRewardPercent') as number
    foundation.reviewRewardFactor = graphqlResult.keyValue(res, 'reviewRewardFactor') as number
    foundation.authorRewardPercent = graphqlResult.keyValue(res, 'authorRewardPercent') as number
    foundation.authorRewardFactor = graphqlResult.keyValue(res, 'authorRewardFactor') as number
    foundation.activityRewardPercent = graphqlResult.keyValue(res, 'activityRewardPercent') as number
  })
}

onMounted(() => {
  if (!ready()) return
  getFoundationInfo()
})

</script>
