<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'
import { useSettingStore } from 'src/stores/setting'

const application = useApplicationStore()
const marketApp = computed(() => application.marketApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && marketApp.value?.length > 0
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
    endpoint: 'market',
    chainId: targetChain.value
  })
}

const requestSubscribeThroughCheCko = () => {
  const query = gql`
    mutation requestSubscribe {
      requestSubscribe
    }`

  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      applicationId: marketApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {},
        operationName: 'requestSubscribe'
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

watch(marketApp, () => {
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
