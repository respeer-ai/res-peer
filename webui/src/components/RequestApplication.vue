<script setup lang="ts">
import { onMounted, watch, computed } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import * as constants from 'src/const'
import { targetChain } from 'src/stores/chain'
import { useApplicationStore } from 'src/stores/application'
import { useUserStore } from 'src/stores/user'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const application = useApplicationStore()
const user = useUserStore()
const account = computed(() => user.account)

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const requestApplication = async (index: number, retry: boolean) => {
  if (index >= constants.appIds.length) {
    return
  }
  if (retry) {
    setTimeout(() => {
      void requestApplication(index + 1, retry)
    }, 3000)
    return
  }
  const appId = constants.appIds[index]
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation requestApplication ($chainId: String!, $applicationId: String!, $targetChainId: String!) {
      requestApplication(chainId: $chainId, applicationId: $applicationId, targetChainId: $targetChainId)
    }
  `))
  onDone(() => {
    setTimeout(() => {
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
          application.foundationApp = constants.Apps.foundationApp
          break
        case 5:
          application.activityApp = constants.Apps.activityApp
          break
      }
    }, 1000)
    void requestApplication(index + 1, false)
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

const requestApplicationThroughCheCko = (index: number, retry: boolean) => {
  if (index >= constants.appIds.length) {
    return
  }
  if (retry) {
    setTimeout(() => {
      void requestApplicationThroughCheCko(index + 1, retry)
    }, 3000)
    return
  }
  const appId = constants.appIds[index]
  const query = gql`
    mutation requestApplication ($chainId: String!, $applicationId: String!, $targetChainId: String!) {
      requestApplication(chainId: $chainId, applicationId: $applicationId, targetChainId: $targetChainId)
    }`
  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      query: {
        query: query.loc?.source?.body,
        variables: {
          applicationId: appId,
          targetChainId: constants.appDeployChain
        },
        operationName: 'requestApplication'
      }
    }
  }).then((result) => {
    console.log(result)
  }).catch((e) => {
    console.log(e)
  })
}

watch(account, () => {
  if (targetChain.value && account) {
    void requestApplicationThroughCheCko(0, false)
  }
})

watch(targetChain, () => {
  if (targetChain.value && account) {
    void requestApplicationThroughCheCko(0, false)
  }
})

onMounted(() => {
  if (targetChain.value && account) {
    void requestApplicationThroughCheCko(0, false)
  }
})

</script>
