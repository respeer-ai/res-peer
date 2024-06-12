<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useBlockStore } from 'src/stores/block'
import { computed, onMounted, watch } from 'vue'
import { targetChain } from 'src/stores/chain'
import { useApplicationStore } from 'src/stores/application'
import { useActivityStore } from 'src/stores/activity'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const activity = useActivityStore()
const application = useApplicationStore()
const activityApp = computed(() => application.activityApp)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && activityApp.value?.length > 0
}

const getActivitiesKeys = () => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getActivitiesKeys {
      activities {
        keys
      }
    }
  `, {
    endpoint: 'activity',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const activities = graphqlResult.data(res, 'activities')
    activity.activitiesKeys = graphqlResult.keys(activities) as Array<number>
  })
}

const getActivitiesKeysThroughCheCko = () => {
  const query = gql`
    query getActivitiesKeys {
      activities {
        keys
      }
    }`
  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: activityApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {},
        operationName: 'getActivitiesKeys'
      }
    }
  }).then((result) => {
    const activities = graphqlResult.keyValue(result, 'activities')
    activity.activitiesKeys = graphqlResult.keys(activities) as Array<number>
  }).catch((e) => {
    console.log(e)
  })
}

watch(blockHeight, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getActivitiesKeysThroughCheCko()
  } else {
    getActivitiesKeys()
  }
})

watch(activityApp, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getActivitiesKeysThroughCheCko()
  } else {
    getActivitiesKeys()
  }
})

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getActivitiesKeysThroughCheCko()
  } else {
    getActivitiesKeys()
  }
})

</script>
