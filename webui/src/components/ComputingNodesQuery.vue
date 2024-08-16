<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { computed, watch, onMounted } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'
import { CPNode, useCPRegistryStore } from 'src/stores/cpregistry'

const cpRegistry = useCPRegistryStore()
const application = useApplicationStore()
const cpRegistryApp = computed(() => application.cpRegistryApp)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && cpRegistryApp.value?.length
}

const getCPNodes = (done?: () => void) => {
  const { /* result, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getCPNodes {
      nodes {
        entries {
          value {
            nodeId
            brandLogo
            brandName
            link
            applicationId
            resourceType
            deviceModel
            cpuModel
            storageType
            storageBytes
            memoryBytes
            freeQuota
            priceQuota
            quotaPrice
            supportedTaskTypes
            aiModel
            aiModelUrl
            paymentChainId
            available
            createdAt
          }
        }
      }
    }
  `, {
    endpoint: 'cp-registry',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const _nodes = graphqlResult.data(res, 'nodes')
    const entries = graphqlResult.keyValue(_nodes, 'entries') as unknown[]
    cpRegistry.nodes = entries.map((el) => graphqlResult.keyValue(el, 'value')) as CPNode[]
    done?.()
  })
}

const getCPNodesThroughCheCko = (done?: () => void) => {
  const query = gql`
    query getCPNodesThroughCheCko {
      nodes {
        entries {
          value {
            nodeId
            brandLogo
            brandName
            link
            applicationId
            resourceType
            deviceModel
            cpuModel
            storageType
            storageBytes
            memoryBytes
            freeQuota
            priceQuota
            quotaPrice
            supportedTaskTypes
            paymentChainId
            available
            createdAt
          }
        }
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: cpRegistryApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {},
        operationName: 'getCPNodesThroughCheCko'
      }
    }
  }).then((result) => {
    const _nodes = graphqlResult.keyValue(result, 'nodes')
    const entries = graphqlResult.keyValue(_nodes, 'entries') as unknown[]
    cpRegistry.nodes = entries.map((el) => graphqlResult.keyValue(el, 'value')) as CPNode[]
    done?.()
  }).catch((e) => {
    console.log(e)
  })
}

const _getCPNodes = () => {
  if (cheCkoConnect.value) {
    getCPNodesThroughCheCko()
  } else {
    getCPNodes()
  }
}

watch(cpRegistryApp, () => {
  if (!ready()) return
  _getCPNodes()
})

watch(blockHeight, () => {
  if (!ready()) return
  _getCPNodes()
})

onMounted(() => {
  if (!ready()) return
  _getCPNodes()
})

</script>
