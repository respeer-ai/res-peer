<script setup lang='ts'>
import { TaskType, taskTypePrefix, useCPRegistryStore } from 'src/stores/cpregistry'
import { useSettingStore } from 'src/stores/setting'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, toRef, defineModel } from 'vue'
import Web3 from 'web3'
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { graphqlResult } from 'src/utils'
import { targetChain } from 'src/stores/chain'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

interface Props {
  text: string
  taskType: TaskType
  nodeId: string
}
const props = defineProps<Props>()
const text = toRef(props, 'text')
const taskType = toRef(props, 'taskType')
const nodeId = toRef(props, 'nodeId')

const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const user = useUserStore()
const loginAccount = computed(() => user.account)
const cpRegistry = useCPRegistryStore()
const cpNode = computed(() => cpRegistry.nodes.find((el) => el.nodeId === nodeId.value))

const queryId = defineModel({ type: Object })
const signature = defineModel('signature', { type: String })

const emit = defineEmits<{(ev: 'done'): void,
  (ev: 'fail'): void
}>()

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && loginAccount.value?.length > 0
}

const getQueryId = (prompt: string, publicKey: string, signature: string) => {
  const { /* result, refetch, fetchMore, */ onResult, onError } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getQueryId($prompt: String!, $publicKey: String!, $signature: String!) {
      getQueryId(prompt: $prompt, publicKey: $publicKey, signature: $signature) {
        queryId
        nonce
        timestamp
      }
    }
  `, {
    endpoint: 'copilot',
    applicationId: cpNode.value?.applicationId,
    prompt,
    publicKey,
    signature,
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    queryId.value = graphqlResult.data(res, 'getQueryId')
    emit('done')
  })

  onError((e) => {
    console.log('getQueryId', e)
    emit('fail')
  })
}

const getQueryIdThroughCheCko = (prompt: string, publicKey: string, signature: string) => {
  const query = gql`
    query getQueryIdThroughCheCko($prompt: String!, $publicKey: String!, $signature: String!) {
      getQueryId(prompt: $prompt, publicKey: $publicKey, signature: $signature) {
        queryId
        nonce
        timestamp
      }
    }`
  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: cpNode.value?.applicationId,
      query: {
        query: query.loc?.source?.body,
        variables: {
          prompt,
          publicKey,
          signature
        },
        operationName: 'getQueryIdThroughCheCko'
      }
    }
  }).then((result) => {
    queryId.value = graphqlResult.keyValue(result, 'getQueryId')
    emit('done')
  }).catch((e) => {
    console.log(e)
    emit('fail')
  })
}

onMounted(() => {
  if (!ready()) return emit('fail')
  const web3 = new Web3(window.linera)
  const prompt = taskTypePrefix(taskType.value) + text.value
  const hexPrompt = web3.utils.utf8ToHex(prompt)
  web3.eth.sign(hexPrompt, '0x' + loginAccount.value.slice(0, 40)).then((v) => {
    signature.value = (v as string).replace('0x', '')
    if (cheCkoConnect.value) {
      getQueryIdThroughCheCko(prompt, loginAccount.value, (v as string).substring(2))
    } else {
      getQueryId(prompt, loginAccount.value, v as string)
    }
  }).catch((e) => {
    console.log('Sign', prompt, hexPrompt, loginAccount.value, e)
    emit('fail')
  })
})
</script>
