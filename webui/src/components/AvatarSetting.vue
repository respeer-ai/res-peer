<template>
  <div class='row'>
    <span class='text-h5'>Avatar</span>
    <q-space />
    <q-btn
      dense flat v-if='!editing' label='Set Avatar'
      color='blue'
      @click='editing = !editing'
    />
  </div>
  <div class='row'>
    <q-space />
    <q-btn
      dense flat v-if='editing' label='Set Avatar'
      color='blue'
      @click='onSetAvatarClick'
    />
  </div>
  <q-input
    v-if='editing' v-model='collectionId' type='number' filled
    :style='{marginTop: "16px"}' label='Avatar asset collection id'
  />
  <q-input
    v-if='editing' v-model='tokenId' type='number' filled
    :style='{marginTop: "16px"}' label='Avatar asset token id'
  />
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useCollectionStore } from 'src/stores/collection'
import { useUserStore } from 'src/stores/user'
import { targetChain } from 'src/stores/chain'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const editing = ref(false)
const collectionId = ref(0)
const tokenId = ref(0)
const collection = useCollectionStore()
const user = useUserStore()
const account = computed(() => user.account)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const marketApp = computed(() => application.marketApp)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const setAvatar = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation setAvatar ($collectionId: Int!, $tokenId: Int!) {
      setAvatar(collectionId: $collectionId, tokenId: $tokenId)
    }
  `))
  onDone(() => {
    editing.value = !editing.value
    collection.avatars.set(account.value, [collectionId.value, tokenId.value])
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    collectionId: parseInt(collectionId.value.toString()),
    tokenId: parseInt(tokenId.value.toString()),
    endpoint: 'market',
    chainId: targetChain.value
  })
}

const setAvatarThroughCheCko = () => {
  const query = gql`
    mutation setAvatar ($collectionId: Int!, $tokenId: Int!) {
      setAvatar(collectionId: $collectionId, tokenId: $tokenId)
    }`

  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      applicationId: marketApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          collectionId: parseInt(collectionId.value.toString()),
          tokenId: parseInt(tokenId.value.toString())
        },
        operationName: 'submitContent'
      }
    }
  }).then(() => {
    editing.value = !editing.value
    collection.avatars.set(account.value, [collectionId.value, tokenId.value])
  }).catch((e) => {
    console.log(e)
  })
}

const onSetAvatarClick = () => {
  if (collectionId.value <= 0) {
    return
  }
  if (tokenId.value <= 0) {
    return
  }
  if (cheCkoConnect.value) {
    setAvatarThroughCheCko()
  } else {
    void setAvatar()
  }
}

</script>
