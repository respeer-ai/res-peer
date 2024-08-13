<script setup lang="ts">
import { onMounted, computed, toRef } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation, useQuery } from '@vue/apollo-composable'
import * as constants from 'src/const'
import { targetChain } from 'src/stores/chain'
import { useUserStore } from 'src/stores/user'
import { useSettingStore } from 'src/stores/setting'
import { graphqlResult } from 'src/utils'

interface Props {
  applicationId: string
}

const props = defineProps<Props>()
const applicationId = toRef(props, 'applicationId')
const emit = defineEmits<{(ev: 'done'): void,
  (ev: 'fail'): void
}>()

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const user = useUserStore()
const account = computed(() => user.account)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && account.value?.length > 0
}

const getApplications = (done?: (applicationIds: string[]) => void) => {
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
    done?.(Array.from((applications as Record<string, string>[]).map((el) => el.id)))
  })
}

const getApplicationsThroughCheCko = (done?: (applicationIds: string[]) => void) => {
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
    done?.(Array.from((applications as Record<string, string>[]).map((el) => el.id)))
  }).catch((e) => {
    console.log(e)
  })
}

const onApplications = (applications: string[]) => {
  if (applications.includes(applicationId.value)) {
    emit('done')
  } else {
    void requestApplication(true)
  }
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const requestApplication = async (retry: boolean) => {
  if (retry) {
    setTimeout(() => {
      void requestApplication(false)
    }, 1000)
    return
  }
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation requestApplication ($chainId: String!, $applicationId: String!, $targetChainId: String!) {
      requestApplication(chainId: $chainId, applicationId: $applicationId, targetChainId: $targetChainId)
    }
  `))
  onDone(() => {
    getApplications(onApplications)
  })
  onError((error) => {
    void requestApplication(true)
    console.log(error)
  })
  await mutate({
    chainId: targetChain.value,
    applicationId: applicationId.value,
    targetChainId: constants.appDeployChain,
    endpoint: 'main'
  })
}

const onApplicationsThroughCheCko = (applications: string[]) => {
  if (applications.includes(applicationId.value)) {
    emit('done')
  } else {
    void requestApplicationThroughCheCko(true)
  }
}

const requestApplicationThroughCheCko = (retry: boolean) => {
  if (retry) {
    setTimeout(() => {
      void requestApplicationThroughCheCko(false)
    }, 1000)
    return
  }
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
          applicationId: applicationId.value,
          targetChainId: constants.appDeployChain
        },
        operationName: 'requestApplicationWithoutBlockProposal'
      }
    }
  }).then(() => {
    getApplicationsThroughCheCko(onApplicationsThroughCheCko)
  }).catch((e) => {
    void requestApplicationThroughCheCko(true)
    console.log(e)
  })
}

const _requestApplication = () => {
  if (cheCkoConnect.value) {
    if (window.linera) {
      setTimeout(() => {
        void requestApplicationThroughCheCko(false)
      }, 500)
    } else {
      setTimeout(() => {
        _requestApplication()
      }, 500)
    }
  } else {
    void requestApplication(false)
  }
}

onMounted(() => {
  if (!ready()) return emit('fail')
  _requestApplication()
})

</script>
