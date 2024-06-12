<template>
  <div class='row'>
    <span class='text-h5'>Deposit Balance</span>
    <q-space />
    <q-btn
      dense flat v-if='!editing' label='Deposit'
      color='blue'
      @click='editing = !editing'
    />
  </div>
  <div class='row'>
    <q-space />
    <q-btn
      dense flat v-if='editing' label='Deposit'
      color='blue'
      @click='onDepositClick'
    />
  </div>
  <q-input
    v-if='editing' v-model='amount' type='number' filled
    :style='{marginTop: "16px"}' label='Deposit Amount'
  />
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { targetChain } from 'src/stores/chain'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const editing = ref(false)
const amount = ref(0)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const foundationApp = computed(() => application.foundationApp)

const deposit = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation userDeposit ($amount: String!) {
      userDeposit(amount: $amount)
    }
  `))
  onDone(() => {
    editing.value = !editing.value
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    amount: amount.value.toString(),
    endpoint: 'foundation',
    chainId: targetChain.value
  })
}

const depositThroughCheCko = () => {
  const query = gql`
    mutation userDeposit ($amount: String!) {
      userDeposit(amount: $amount)
    }`

  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      applicationId: foundationApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          amount: amount.value.toString()
        }
      }
    }
  }).then(() => {
    editing.value = !editing.value
  }).catch((e) => {
    console.log(e)
  })
}

const onDepositClick = () => {
  if (amount.value < 0) {
    return
  }
  if (cheCkoConnect.value) {
    depositThroughCheCko()
  } else {
    void deposit()
  }
}

</script>
