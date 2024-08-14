<script setup lang="ts">
import { onMounted, watch, computed } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation, useQuery } from '@vue/apollo-composable'
import * as constants from 'src/const'
import { targetChain } from 'src/stores/chain'
import { useApplicationStore } from 'src/stores/application'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'
import { graphqlResult } from 'src/utils'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const application = useApplicationStore()
const user = useUserStore()
const account = computed(() => user.account)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && account.value?.length > 0
}

const getApplications = (index: number, done?: (index: number, applicationIds: string[]) => void) => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getApplications ($chainId: String!) {
      applications (chainId: $chainId) {
        id
      }
    }
  `, {
    endpoint: 'main',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const applications = graphqlResult.data(res, 'applications')
    done?.(index, Array.from((applications as Record<string, string>[]).map((el) => el.id)))
  })
}

const getApplicationsThroughCheCko = (index: number, done?: (index: number, applicationIds: string[]) => void) => {
  const query = gql`
    query getApplicationsThroughCheCko ($chainId: String!) {
      applications (chainId: $chainId) {
        id
      }
    }`
  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      query: {
        query: query.loc?.source?.body,
        variables: {},
        operationName: 'getApplicationsThroughCheCko'
      }
    }
  }).then((result) => {
    const applications = graphqlResult.keyValue(result, 'applications')
    done?.(index, Array.from((applications as Record<string, string>[]).map((el) => el.id)))
  }).catch((e) => {
    console.log(e)
  })
}

const onApplications = (index: number, applications: string[]) => {
  if (applications.includes(constants.appIds[index])) {
    switch (index) {
      case 0:
        application.feedApp = constants.Apps.feedApp
        break
      case 1:
        application.creditApp = constants.Apps.creditApp
        break
      case 2:
        application.marketApp = constants.Apps.marketApp
        break
      case 3:
        application.reviewApp = constants.Apps.reviewApp
        break
      case 4:
        application.activityApp = constants.Apps.activityApp
        break
      case 5:
        application.foundationApp = constants.Apps.foundationApp
        break
      case 6:
        application.blobGatewayApp = constants.Apps.blobGatewayApp
        break
      case 7:
        application.cpRegistryApp = constants.Apps.cpRegistryApp
        break
      case 8:
        application.copilotApp = constants.Apps.copilotApp
        break
    }
    void requestApplication(index + 1, false)
  } else {
    void requestApplication(index, true)
  }
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const requestApplication = async (index: number, retry: boolean) => {
  if (index >= constants.appIds.length) {
    return
  }
  if (retry) {
    setTimeout(() => {
      void requestApplication(index, false)
    }, 1000)
    return
  }
  const appId = constants.appIds[index]
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation requestApplication ($chainId: String!, $applicationId: String!, $targetChainId: String!) {
      requestApplication(chainId: $chainId, applicationId: $applicationId, targetChainId: $targetChainId)
    }
  `))
  onDone(() => {
    getApplications(index, onApplications)
  })
  onError((error) => {
    void requestApplication(index, true)
    console.log(error)
  })
  await mutate({
    chainId: targetChain.value,
    applicationId: appId,
    targetChainId: constants.appDeployChain,
    endpoint: 'main'
  })
}

const onApplicationsThroughCheCko = (index: number, applications: string[]) => {
  if (applications.includes(constants.appIds[index])) {
    console.log('Requested application', index, constants.appIds[index])
    switch (index) {
      case 0:
        application.feedApp = constants.Apps.feedApp
        break
      case 1:
        application.creditApp = constants.Apps.creditApp
        break
      case 2:
        application.marketApp = constants.Apps.marketApp
        break
      case 3:
        application.reviewApp = constants.Apps.reviewApp
        break
      case 4:
        application.activityApp = constants.Apps.activityApp
        break
      case 5:
        application.foundationApp = constants.Apps.foundationApp
        break
      case 6:
        application.blobGatewayApp = constants.Apps.blobGatewayApp
        break
      case 7:
        application.cpRegistryApp = constants.Apps.cpRegistryApp
        break
      case 8:
        application.copilotApp = constants.Apps.copilotApp
        break
    }
    void requestApplicationThroughCheCko(index + 1, false)
  } else {
    console.log('ReRequest application', index, constants.appIds[index])
    void requestApplicationThroughCheCko(index, true)
  }
}

const requestApplicationThroughCheCko = (index: number, retry: boolean) => {
  if (index >= constants.appIds.length) {
    return
  }
  if (retry) {
    setTimeout(() => {
      void requestApplicationThroughCheCko(index, false)
    }, 1000)
    return
  }
  const appId = constants.appIds[index]
  const query = gql`
    mutation requestApplicationWithoutBlockProposal ($chainId: String!, $applicationId: String!, $targetChainId: String!) {
      requestApplicationWithoutBlockProposal(chainId: $chainId, applicationId: $applicationId, targetChainId: $targetChainId)
    }`
  window.linera?.request({
    method: 'linera_graphqlMutation',
    params: {
      query: {
        query: query.loc?.source?.body,
        variables: {
          applicationId: appId,
          targetChainId: constants.appDeployChain
        },
        operationName: 'requestApplicationWithoutBlockProposal'
      }
    }
  }).then(() => {
    getApplicationsThroughCheCko(index, onApplicationsThroughCheCko)
  }).catch((e) => {
    void requestApplicationThroughCheCko(index, true)
    console.log(e)
  })
}

const requestApplications = () => {
  if (cheCkoConnect.value) {
    if (window.linera) {
      setTimeout(() => {
        void requestApplicationThroughCheCko(0, false)
      }, 500)
    } else {
      setTimeout(() => {
        requestApplications()
      }, 500)
    }
  } else {
    void requestApplication(0, false)
  }
}

watch(account, () => {
  if (!ready()) return
  requestApplications()
})

watch(targetChain, () => {
  if (!ready()) return
  requestApplications()
})

onMounted(() => {
  if (!ready()) return
  requestApplications()
})

</script>
