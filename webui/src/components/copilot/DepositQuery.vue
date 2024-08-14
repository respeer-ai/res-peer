<script setup lang='ts'>
import { computed, onMounted, toRef } from 'vue'
import { provideApolloClient, useMutation, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { useSettingStore } from 'src/stores/setting'
import { getClientOptions } from 'src/apollo'
import { targetChain } from 'src/stores/chain'
import { useCPRegistryStore } from 'src/stores/cpregistry'
import { useUserStore } from 'src/stores/user'
import { graphqlResult } from 'src/utils'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

interface Props {
  queryId: string
  nodeId: string
}
const props = defineProps<Props>()

const queryId = toRef(props, 'queryId')
const nodeId = toRef(props, 'nodeId')

const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const cpRegistry = useCPRegistryStore()
const cpNode = computed(() => cpRegistry.nodes.find((el) => el.nodeId === nodeId.value))
const user = useUserStore()
const loginAccount = computed(() => user.account)

const emit = defineEmits<{(ev: 'paid'): void,
  (ev: 'fail'): void,
  (ev: 'confirmed'): void
}>()

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && loginAccount.value?.length > 0
}

const queryDeposited = () => {
  const { /* result, refetch, fetchMore, */ onResult, onError } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query queryDeposited($publicKey: String!, $queryId: String!) {
      queryDeposited(publicKey: $publicKey, queryId: $queryId)
    }
  `, {
    endpoint: 'copilot',
    applicationId: cpNode.value?.applicationId,
    publicKey: loginAccount.value,
    queryId: queryId.value,
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    emit('confirmed')
  })

  onError((e) => {
    console.log('queryDeposited', e)
    emit('fail')
  })
}

const queryDepositedThroughCheCko = () => {
  const query = gql`
    query queryDepositedThroughCheCko($publicKey: String!, $queryId: String!) {
      queryDeposited(publicKey: $publicKey, queryId: $queryId)
    }`
  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: cpNode.value?.applicationId,
      query: {
        query: query.loc?.source?.body,
        variables: {
          publicKey: loginAccount.value,
          queryId: queryId.value
        },
        operationName: 'queryDepositedThroughCheCko'
      }
    }
  }).then((result) => {
    const paid = graphqlResult.keyValue(result, 'queryDeposited') as boolean
    if (paid) emit('confirmed')
    else setTimeout(() => queryDepositedThroughCheCko(), 1000)
  }).catch((e) => {
    console.log(e)
    emit('fail')
  })
}

const _queryDeposited = () => {
  if (cheCkoConnect.value) {
    queryDepositedThroughCheCko()
  } else {
    void queryDeposited()
  }
}

const depositQuery = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation depositQuery($queryId: String!) {
      deposit(queryId: $queryId)
    }
  `))

  onDone(() => {
    emit('paid')
    _queryDeposited()
  })
  onError((e) => {
    console.log(e)
    emit('fail')
  })
  await mutate({
    queryId: queryId.value,
    endpoint: 'copilot',
    chainId: targetChain.value,
    applicationId: cpNode.value?.applicationId
  })
}

const depositQueryThroughCheCko = () => {
  const query = gql`
    mutation depositQueryThroughCheCko($queryId: String!) {
      deposit(queryId: $queryId)
    }`
  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      applicationId: cpNode.value?.applicationId,
      query: {
        query: query.loc?.source?.body,
        variables: {
          queryId: queryId.value
        },
        operationName: 'depositQueryThroughCheCko'
      }
    }
  }).then(() => {
    emit('paid')
    _queryDeposited()
  }).catch((e) => {
    console.log(e)
    emit('fail')
  })
}

const _depositQuery = () => {
  if (cheCkoConnect.value) {
    depositQueryThroughCheCko()
  } else {
    void depositQuery()
  }
}

onMounted(() => {
  if (!ready()) return emit('fail')
  _depositQuery()
})

</script>
