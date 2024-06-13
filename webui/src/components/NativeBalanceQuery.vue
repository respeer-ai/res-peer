<script setup lang="ts">
import { useBlockStore } from 'src/stores/block'
import { useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'
import Web3 from 'web3'

const user = useUserStore()
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const account = computed(() => user.account)
const chainId = computed(() => user.chainId)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = () => {
  return account.value?.length && chainId.value?.length && (cheCkoConnect.value || targetChain.value?.length)
}

interface ChainAccountBalances {
  chain_balance: number
  account_balances: Record<string, number>
}

const getChainAccountBalances = () => {
  const chainIds = [chainId.value]
  const publicKeys = [account.value]

  const { /* result, refetch, fetchMore, */ onResult, onError } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getChainAccountBalances($chainIds: [String!]!, $publicKeys: [String!]!) {
      balances(chainIds: $chainIds, publicKeys: $publicKeys)
    }
  `, {
    chainIds,
    publicKeys
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    const balances = graphqlResult.data(res, 'balances') as Record<string, ChainAccountBalances>
    user.chainBalance = balances[chainId.value]?.chain_balance.toString()
  })

  onError(() => {
    // console.log('Get pending block', error)
  })
}

const getChainAccountBalancesThroughCheCko = () => {
  const chainIds = [chainId.value]
  const publicKeys = [account.value]

  const query = gql`
    query getChainAccountBalances($chainIds: [String!]!, $publicKeys: [String!]!) {
      balances(chainIds: $chainIds, publicKeys: $publicKeys)
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      query: {
        query: query.loc?.source?.body,
        variables: {
          chainIds,
          publicKeys
        },
        operationName: 'getChainAccountBalances'
      }
    }
  }).then((result) => {
    const balances = graphqlResult.keyValue(result, 'balances') as Record<string, ChainAccountBalances>
    user.chainBalance = balances[chainId.value]?.chain_balance.toString()
  }).catch((e) => {
    console.log(e)
  })
}

const getAccountBalance = () => {
  const web3 = new Web3(window.linera)
  web3.eth.getBalance('0x' + account.value.slice(0, 40) /* Work around for web3.js address format */)
    .then((result) => {
      user.accountBalance = result.toString()
    })
    .catch((e) => {
      console.log('eth_getBalance', e)
    })
}

watch(blockHeight, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getAccountBalance()
    getChainAccountBalancesThroughCheCko()
  } else {
    getChainAccountBalances()
  }
})

watch(account, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getAccountBalance()
    getChainAccountBalancesThroughCheCko()
  } else {
    getChainAccountBalances()
  }
})

watch(chainId, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getChainAccountBalancesThroughCheCko()
  } else {
    getChainAccountBalances()
  }
})

watch(targetChain, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getChainAccountBalancesThroughCheCko()
  } else {
    getChainAccountBalances()
  }
})

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getAccountBalance()
    getChainAccountBalancesThroughCheCko()
  } else {
    getChainAccountBalances()
  }
})

</script>
