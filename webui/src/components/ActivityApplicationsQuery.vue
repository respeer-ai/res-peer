<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useReviewStore, Activity } from 'src/stores/review'
import { computed, watch, ref, onMounted } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const review = useReviewStore()
const activityApplicationsKeys = computed(() => review.activityApplicationsKeys)
const activityApplications = computed(() => review.activityApplications)
const activityMutateKeys = computed(() => review.activityMutateKeys)
const activityIndex = ref(-1)
const activityApplicationKey = computed(() => activityIndex.value >= 0 ? activityApplicationsKeys.value[activityIndex.value] : undefined)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getActivityApplication = (activityApplicationKey: number, done?: () => void) => {
  const { /* result, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getActivityApplication($activityApplicationKey: Int!) {
      activityApplications {
        entry(key: $activityApplicationKey) {
          value {
            activityId
            budgetAmount
            reviewers
            approved
            rejected
            createdAt
          }
        }
      }
    }
  `, {
    activityApplicationKey,
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const _activityApplications = graphqlResult.data(res, 'activityApplications')
    activityApplications.value.set(Number(activityApplicationKey), graphqlResult.entryValue(_activityApplications) as Activity)
    done?.()
  })
}

const getActivityApplicationThroughCheCko = (activityApplicationKey: number, done?: () => void) => {
  const query = gql`
    query getActivityApplication($activityApplicationKey: Int!) {
      activityApplications {
        entry(key: $activityApplicationKey) {
          value {
            activityId
            budgetAmount
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
          activityApplicationKey
        },
        operationName: 'getActivityApplication'
      }
    }
  }).then((result) => {
    const _activityApplications = graphqlResult.keyValue(result, 'activityApplications')
    activityApplications.value.set(Number(activityApplicationKey), graphqlResult.entryValue(_activityApplications) as Activity)
    done?.()
  }).catch((e) => {
    console.log(e)
  })
}

watch(activityApplicationKey, () => {
  if (!activityApplicationKey.value) {
    return
  }

  const index = activityMutateKeys.value.findIndex((el) => el === activityApplicationKey.value)
  if (activityApplications.value.get(activityApplicationKey.value) && index < 0) {
    activityIndex.value++
    return
  }

  if (cheCkoConnect.value) {
    getActivityApplicationThroughCheCko(activityApplicationKey.value, () => {
      activityIndex.value++
      activityMutateKeys.value.splice(index, 1)
    })
  } else {
    getActivityApplication(activityApplicationKey.value, () => {
      activityIndex.value++
      activityMutateKeys.value.splice(index, 1)
    })
  }
})

watch(activityApplicationsKeys, () => {
  if (activityApplicationsKeys.value.length === 0) {
    return
  }
  activityIndex.value = 0
})

watch(blockHeight, () => {
  if (activityApplicationsKeys.value.length === 0) {
    return
  }
  activityIndex.value = 0
})

onMounted(() => {
  activityMutateKeys.value.forEach((activityKey) => {
    getActivityApplication(activityKey)
  })
  review.activityMutateKeys = []
})

</script>
