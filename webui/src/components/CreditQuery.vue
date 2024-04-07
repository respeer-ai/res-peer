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

const user = useUserStore()
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const account = computed(() => user.account)
const application = useApplicationStore()
const creditApp = computed(() => application.creditApp)
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return account.value?.length && creditApp.value?.length && targetChain.value?.length
}

const getBalance = () => {
  const { /* result, */ refetch, /* fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
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

  watch(blockHeight, () => {
    if (!ready()) {
      return
    }
    void refetch({
      owner: account.value,
      endpoint: 'credit',
      chainId: targetChain.value
    })
  })

  watch(account, () => {
    if (!ready()) {
      return
    }
    void refetch({
      owner: account.value,
      endpoint: 'credit',
      chainId: targetChain.value
    })
  })
}

watch(targetChain, () => {
  if (!ready()) return
  getBalance()
})

watch(creditApp, () => {
  if (!ready()) return
  getBalance()
})

onMounted(() => {
  if (!ready()) return
  getBalance()
})

</script>
