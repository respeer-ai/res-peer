<script setup lang='ts'>
import { targetChain } from 'src/stores/chain'
import { QueryId } from 'src/stores/copilot'
import { TaskType, taskTypeName, useCPRegistryStore } from 'src/stores/cpregistry'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, toRef, defineModel } from 'vue'
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

interface Props {
  nodeId: string
  queryId: QueryId
  taskType: TaskType
  text: string
  signature: string
}

const props = defineProps<Props>()
const nodeId = toRef(props, 'nodeId')
const queryId = toRef(props, 'queryId')
const taskType = toRef(props, 'taskType')
const text = toRef(props, 'text')
const signature = toRef(props, 'signature')

const emit = defineEmits<{(ev: 'done'): void,
  (ev: 'fail'): void
}>()

const user = useUserStore()
const loginAccount = computed(() => user.account)
const cpRegistry = useCPRegistryStore()
const cpNode = computed(() => cpRegistry.nodes.find((el) => el.nodeId === nodeId.value))
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const result = defineModel({ type: String })

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && loginAccount.value?.length > 0
}

const prompt = () => {
  const { /* result, refetch, fetchMore, */ onResult, onError } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query prompt($queryId: String!, $prompt: String!, $publicKey: String!, $signature: String!, $timestamp: Int!, $nonce: [Int]!) {
      prompt(queryId: $queryId, prompt: $prompt, publicKey: $publicKey, signature: $signature, timestamp: $timestamp, nonce: $nonce)
    }
  `, {
    endpoint: 'copilot',
    applicationId: cpNode.value?.applicationId,
    publicKey: loginAccount.value,
    queryId: queryId.value?.queryId,
    prompt: taskTypeName(taskType.value) + ': ' + text.value,
    signature: signature.value,
    timestamp: queryId.value?.timestamp,
    nonce: queryId.value?.nonce,
    chainId: cpNode.value?.paymentChainId
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    result.value = graphqlResult.data(res, 'prompt') as string
    emit('done')
  })

  onError((e) => {
    console.log('prompt', e)
    emit('fail')
  })
}

onMounted(() => {
  if (!ready()) return emit('fail')
  prompt()
})

</script>
