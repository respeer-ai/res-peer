<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { computed, watch, ref } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { Activity, useActivityStore } from 'src/stores/activity'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const activity = useActivityStore()
const activitiesKeys = computed(() => activity.activitiesKeys)
const activities = computed(() => activity.activities)
const application = useApplicationStore()
const activityApp = computed(() => application.activityApp)
const contentIndex = ref(-1)
const activityKey = computed(() => contentIndex.value >= 0 ? activitiesKeys.value[contentIndex.value] : undefined)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getActivities = (activityKey: number, done?: () => void) => {
  const { /* result, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getActivities($activityKey: Int!) {
      activities {
        entry(key: $activityKey) {
          value {
            id
            title
            slogan
            banner
            posters
            introduction
            host
            hostResume
            createdAt
            activityType
            votable
            voteType
            objectType
            objectCandidates
            condition {
              classes
              minWords
              maxWords
            }
            sponsors
            prizeConfigs {
              place
              medal
              title
              rewardAmount
            }
            announcements
            prizeAnnouncement
            voterRewardPercent
            votePowers
            voters
            budgetAmount
            joinType
            location
            comments
            registers
            registerStartAt
            registerEndAt
            voteStartAt
            voteEndAt
            participantors
            winners {
              place
              objectId
            }
            finalized
          }
        }
      }
    }
  `, {
    activityKey,
    endpoint: 'activity',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const _activities = graphqlResult.data(res, 'activities')
    activities.value.set(Number(activityKey), graphqlResult.entryValue(_activities) as Activity)
    done?.()
  })
}

const getActivitiesThroughCheCko = (activityKey: number, done?: () => void) => {
  const query = gql`
    query getActivities($activityKey: Int!) {
      activities {
        entry(key: $activityKey) {
          value {
            id
            title
            slogan
            banner
            posters
            introduction
            host
            hostResume
            createdAt
            activityType
            votable
            voteType
            objectType
            objectCandidates
            condition {
              classes
              minWords
              maxWords
            }
            sponsors
            prizeConfigs {
              place
              medal
              title
              rewardAmount
            }
            announcements
            prizeAnnouncement
            voterRewardPercent
            votePowers
            voters
            budgetAmount
            joinType
            location
            comments
            registers
            registerStartAt
            registerEndAt
            voteStartAt
            voteEndAt
            participantors
            winners {
              place
              objectId
            }
            finalized
          }
        }
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: activityApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          activityKey
        },
        operationName: 'getActivities'
      }
    }
  }).then((result) => {
    const _activities = graphqlResult.keyValue(result, 'activities')
    activities.value.set(Number(activityKey), graphqlResult.entryValue(_activities) as Activity)
    done?.()
  }).catch((e) => {
    console.log(e)
  })
}

watch(activityKey, () => {
  if (!activityKey.value) {
    return
  }
  if (cheCkoConnect.value) {
    getActivitiesThroughCheCko(activityKey.value, () => {
      contentIndex.value++
    })
  } else {
    getActivities(activityKey.value, () => {
      contentIndex.value++
    })
  }
})

watch(activitiesKeys, () => {
  if (activitiesKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

watch(blockHeight, () => {
  if (activitiesKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

</script>
