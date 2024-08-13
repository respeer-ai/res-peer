<template>
  <div class='submit-content'>
    <q-input
      outlined label-slot
      v-model='title'
      :suffix='title.length + "/64"'
    >
      <template #label>
        <div class='row text-grey-8'>
          <div>Title</div>
          <div class='text-red-6'>
            *
          </div>
        </div>
      </template>
    </q-input>
    <div :style='{marginTop: "24px"}' />
    <div :style='{width: "100%", height: "500px"}'>
      <Editor
        v-if='false'
        v-model='content'
        editor-style='height: 320px; font-size: 16px'
        placeholder='Body'
        @selection-change='ev => onSelectedTextChange(ev)'
      />
      <TinymceEditor v-model='content' />
    </div>
    <div class='row content-operation' :style='{marginTop: "24px"}'>
      <div
        :style='{
          height: "128px",
          width: "220px"
        }'
        class='cursor-pointer cover-uploader'
      >
        <FileUpload
          name='demo[]' @upload='onAdvancedUpload($event)' :multiple='false'
          accept='image/*' :max-file-size='1000000'
        >
          <template #header>
            <div />
          </template>
          <template #content='{ files }'>
            <div class='flex justify-center items-center' :style='{height: "106px"}' v-if='files.length'>
              <div class='flex flex-wrap gap-4'>
                <div v-for='file of files' :key='file.name + file.type + file.size' class='p-8 rounded-border flex flex-col border border-surface items-center gap-4'>
                  <div>
                    <img
                      role='presentation' :alt='file.name' :src='file.objectURL' width='100'
                      height='50'
                    >
                  </div>
                </div>
              </div>
            </div>
          </template>
          <template #empty>
            <div class='flex justify-center items-center text-center' :style='{height: "106px", padding: "24px 0"}'>
              <div>
                <q-icon name='bi-plus-lg' size='36px' color='grey-6' />
                <div class='text-grey-6'>
                  Drag and drop cover or<br>Create with Copilot
                </div>
              </div>
            </div>
          </template>
        </FileUpload>
        <div
          :style='{marginLeft: "192px", marginTop: "-32px", borderRadius: "50%", width: "22px", height: "22px", padding: "1px"}'
          class='cursor-pointer shadow-6 helper-icon'
        >
          <q-img :src='copilotIcon' width='16px' height='16px' fit='contain' />
        </div>
      </div>
      <div
        :style='{
          height: "128px",
          width: "calc(100% - 220px)"
        }'
        class='abbreviation'
      >
        <q-input
          outlined
          v-model='abbreviation'
          type='textarea'
          placeholder='Use part of the first paragraph as abbreviation or create with Copilot.'
        />
        <div
          :style='{marginLeft: "calc(100% - 28px)", marginTop: "-32px", borderRadius: "50%", width: "22px", height: "22px", padding: "1px"}'
          class='cursor-pointer shadow-6 helper-icon'
        >
          <q-img
            :src='copilotIcon' width='16px' height='16px' fit='contain'
          />
        </div>
      </div>
    </div>
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

import Editor, { EditorSelectionChangeEvent } from 'primevue/editor'
import FileUpload from 'primevue/fileupload'
import TinymceEditor from './editor/TinymceEditor.vue'

import { copilotIcon } from 'src/assets'

const title = ref('')
const content = ref('')
const abbreviation = ref('')

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

interface Range {
  index: number
  length: number
}

const showCopilot = ref(false)

const onSelectedTextChange = (ev: EditorSelectionChangeEvent) => {
  const range = ev.range as Range
  if (!range || range.length === 0) {
    showCopilot.value = false
    return
  }
  showCopilot.value = true
}

const onAdvancedUpload = (ev: unknown) => {
  console.log(ev)
}

</script>

<style scoped lang='sass'>
.p-editor
  border-radius: 64px !important

::v-deep .p-fileupload-header
  display: none !important

::v-deep .p-progressbar
  display: none !important
</style>
