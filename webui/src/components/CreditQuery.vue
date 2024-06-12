<script setup lang="ts">
import { useBlockStore } from 'src/stores/block'
import { AgeAmount, useUserStore } from 'src/stores/user'
import { computed, onMounted, watch } from 'vue'
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useApplicationStore } from 'src/stores/application'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'

const user = useUserStore()
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const account = computed(() => user.account)
const application = useApplicationStore()
const creditApp = computed(() => application.creditApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const ready = () => {
  return account.value?.length && creditApp.value?.length && (cheCkoConnect.value || targetChain.value?.length)
}

const getBalance = () => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getBalance($owner: String!) {
      spendables {
        entry(key: $owner) {
          value
        }
      }
      balances {
        entry(key: $owner) {
          value {
            amounts {
              amount
              expired
            }
          }
        }
      }
    }
  `, {
    owner: account.value,
    endpoint: 'credit',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const spendables = graphqlResult.data(res, 'spendables')
    user.spendable = graphqlResult.entryValue(spendables) as string
    const balances = graphqlResult.data(res, 'balances')
    user.amounts = graphqlResult.entryValueKeyValue(balances, 'amounts') as Array<AgeAmount>
  })
}

const getBalanceThroughCheCko = () => {
  const query = gql`
    query getBalance($owner: String!) {
      spendables {
        entry(key: $owner) {
          value
        }
      }
      balances {
        entry(key: $owner) {
          value {
            amounts {
              amount
              expired
            }
          }
        }
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: creditApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          owner: account.value
        },
        operationName: 'getBalance'
      }
    }
  }).then((result) => {
    const spendables = graphqlResult.keyValue(result, 'spendables')
    user.spendable = graphqlResult.entryValue(spendables) as string
    const balances = graphqlResult.keyValue(result, 'balances')
    user.amounts = graphqlResult.entryValueKeyValue(balances, 'amounts') as Array<AgeAmount>
  }).catch((e) => {
    console.log(e)
  })
}

watch(blockHeight, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getBalanceThroughCheCko()
  } else {
    getBalance()
  }
})

watch(account, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getBalanceThroughCheCko()
  } else {
    getBalance()
  }
})

watch(targetChain, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getBalanceThroughCheCko()
  } else {
    getBalance()
  }
})

watch(creditApp, () => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getBalanceThroughCheCko()
  } else {
    getBalance()
  }
})

onMounted(() => {
  if (!ready()) return
  if (cheCkoConnect.value) {
    getBalanceThroughCheCko()
  } else {
    getBalance()
  }
})

</script>
