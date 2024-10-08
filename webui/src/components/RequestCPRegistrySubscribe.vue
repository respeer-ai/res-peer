<script lang='ts' setup>
import { computed, onMounted, watch } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import { targetChain } from 'src/stores/chain'
import { useApplicationStore } from 'src/stores/application'
import { useSettingStore } from 'src/stores/setting'

const application = useApplicationStore()
const cpRegistryApp = computed(() => application.cpRegistryApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && cpRegistryApp.value?.length > 0
}

const requestSubscribe = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation requestSubscribe {
      requestSubscribe
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    endpoint: 'cp-registry',
    chainId: targetChain.value
  })
}

const requestSubscribeThroughCheCko = () => {
  const query = gql`
    mutation requestSubscribeThroughCheCko {
      requestSubscribe
    }
  `
  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      applicationId: cpRegistryApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {},
        operationName: 'requestSubscribeThroughCheCko'
      }
    }
  }).then((result) => {
    console.log(result)
  }).catch((e) => {
    console.log(e)
  })
}

watch(targetChain, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    requestSubscribeThroughCheCko()
  } else {
    void requestSubscribe()
  }
})

watch(cpRegistryApp, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    requestSubscribeThroughCheCko()
  } else {
    void requestSubscribe()
  }
})

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    requestSubscribeThroughCheCko()
  } else {
    void requestSubscribe()
  }
})
</script>
