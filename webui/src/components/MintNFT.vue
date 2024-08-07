<template>
  <div class='row'>
    <span class='text-h5'>Mint NFT</span>
    <q-space />
    <q-btn
      dense flat v-if='!editing' label='Mint'
      color='blue'
      @click='editing = !editing'
    />
  </div>
  <div class='row'>
    <q-space />
    <q-btn
      dense flat v-if='editing' label='Mint'
      color='blue'
      @click='onMintlick'
    />
  </div>
  <q-input
    v-if='editing' v-model='collectionId' type='number' filled
    :style='{marginTop: "16px"}' label='Collection ID'
  />
  <q-input
    v-if='editing' dense label='Index of uris which are approved'
    type='number' v-model='uriIndex'
  />
  <q-input v-if='editing' dense label='NFT name you like' v-model='name' />
  <q-toggle v-if='editing' v-model='ownPrice' />
  <q-input
    v-if='editing && ownPrice' v-model='price' type='number' filled
    :style='{marginTop: "16px"}' label='Price'
  />
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useCollectionStore } from 'src/stores/collection'
import { targetChain } from 'src/stores/chain'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const editing = ref(false)
const uriIndex = ref(-1)
const price = ref(0)
const ownPrice = ref(false)
const name = ref('')
const collectionId = ref(-1)
const collection = useCollectionStore()
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const marketApp = computed(() => application.marketApp)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const mintNft = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation mintNft ($collectionId: Int!, $uriIndex: Int!, $price: String, $name: String!) {
      mintNft(collectionId: $collectionId, uriIndex: $uriIndex, price: $price, name: $name)
    }
  `))
  onDone(() => {
    editing.value = !editing.value
    uriIndex.value = 0
    collection.mutateKeys.push(parseInt(collectionId.value.toString()))
  })
  onError((error) => {
    uriIndex.value = 0
    console.log(error)
  })
  await mutate({
    collectionId: parseInt(collectionId.value.toString()),
    uriIndex: parseInt(uriIndex.value.toString()),
    price: ownPrice.value ? price.value : undefined,
    name: name.value,
    endpoint: 'market',
    chainId: targetChain.value
  })
}

const mintNftThroughCheCko = () => {
  const query = gql`
    mutation mintNft ($collectionId: Int!, $uriIndex: Int!, $price: String, $name: String!) {
      mintNft(collectionId: $collectionId, uriIndex: $uriIndex, price: $price, name: $name)
    }`
  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      applicationId: marketApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          collectionId: parseInt(collectionId.value.toString()),
          uriIndex: parseInt(uriIndex.value.toString()),
          price: ownPrice.value ? price.value : undefined,
          name: name.value
        },
        operationName: 'mintNft'
      }
    }
  }).then(() => {
    editing.value = !editing.value
    uriIndex.value = 0
    collection.mutateKeys.push(parseInt(collectionId.value.toString()))
  }).catch((e) => {
    uriIndex.value = 0
    console.log(e)
  })
}

const onMintlick = () => {
  if (uriIndex.value < 0) {
    return
  }
  if (collectionId.value < 0) {
    return
  }
  if (!name.value.length) {
    return
  }
  if (cheCkoConnect.value) {
    mintNftThroughCheCko()
  } else {
    void mintNft()
  }
}

</script>
