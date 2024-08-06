<template>
  <q-input dense label='Title' v-model='title' />
  <q-input
    v-model='content' type='textarea' filled
    :style='{marginTop: "16px"}'
  />
  <div class='row' :style='{marginTop: "24px"}'>
    <q-space />
    <q-btn
      dense flat rounded label='Submit'
      @click='onPublishClick'
      class='bg-red-5 text-white'
      :style='{width: "80px"}'
    />
    <q-btn
      dense flat rounded label='Cancel'
      color='grey-8'
      @click='onCancelClick'
      :style='{width: "80px"}'
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { CID } from 'multiformats/cid'
import * as json from 'multiformats/codecs/json'
import { sha256 } from 'multiformats/hashes/sha2'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { targetChain } from 'src/stores/chain'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const title = ref('')
const content = ref('')

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)

const emit = defineEmits<{(ev: 'canceled'): void,
  (ev: 'submitted'): void,
  (ev: 'error'): void
}>()

const submitContent = async () => {
  const bytes = json.encode({ content })
  const hash = await sha256.digest(bytes)
  const cid = CID.create(1, json.code, hash).toString()

  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation submitContent ($cid: String!, $title: String!, $content: String!) {
      submitContent(cid: $cid, title: $title, content: $content)
    }
  `))
  onDone(() => {
    emit('submitted')
  })
  onError((error) => {
    console.log(error)
    emit('error')
  })
  await mutate({
    cid,
    title: title.value,
    content: content.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
}

const submitContentThroughCheCko = async () => {
  const bytes = json.encode({ content })
  const hash = await sha256.digest(bytes)
  const cid = CID.create(1, json.code, hash).toString()

  const query = gql`
    mutation submitContent ($cid: String!, $title: String!, $content: String!) {
      submitContent(cid: $cid, title: $title, content: $content)
    }
  `
  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      applicationId: reviewApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          cid,
          title: title.value,
          content: content.value,
          chainId: targetChain.value
        },
        operationName: 'submitContent'
      }
    }
  }).then((result) => {
    console.log(result)
    emit('submitted')
  }).catch((e) => {
    console.log(e)
    emit('error')
  })
}

const onPublishClick = () => {
  if (title.value.length <= 0 || content.value.length <= 0) {
    return
  }
  if (cheCkoConnect.value) {
    void submitContentThroughCheCko()
  } else {
    void submitContent()
  }
}

const onCancelClick = () => {
  emit('canceled')
}

</script>
