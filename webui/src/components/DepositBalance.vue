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
      @click='onDepositlick'
    />
  </div>
  <q-input
    v-if='editing' v-model='amount' type='number' filled
    :style='{marginTop: "16px"}' label='Deposit Amount'
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'

const editing = ref(false)
const amount = ref(0)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const onDepositlick = async () => {
  if (amount.value < 0) {
    return
  }

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation deposit ($amount: String!) {
      deposit(amount: $amount)
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
    endpoint: 'mall'
  })
}

</script>