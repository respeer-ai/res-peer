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
          accept='image/*' :max-file-size='1000000' @select='onFileSelected($event)'
        >
          <template #header>
            <div />
          </template>
          <template #content='{ files }'>
            <div class='flex justify-center items-center' :style='{height: "106px"}' v-if='files.length'>
              <div class='flex flex-wrap gap-4'>
                <div v-if='false'>
                  <div v-for='file of files' :key='file.name + file.type + file.size' class='p-8 rounded-border flex flex-col border border-surface items-center gap-4'>
                    <div>
                      <img
                        role='presentation' :alt='file.name' :src='file.objectURL' width='100'
                        height='50'
                      >
                    </div>
                  </div>
                </div>
                <div v-if='coverBase64?.length'>
                  <img
                    :src='coverBase64' role='presentation' width='100' height='50'
                    alt='Cover Image'
                  >
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
          @click='onCoverCopilotClick'
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
  <q-dialog v-model='showCoverCopilot' full-width>
    <div :style='{padding: "96px"}'>
      <IllustrateCopilot :text='coverDescription' @cancel='onCoverCopilotCancel' @generated='(base64) => onCoverGenerated(base64)' />
    </div>
  </q-dialog>
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
import FileUpload, { FileUploadSelectEvent } from 'primevue/fileupload'
import { htmlToText } from 'html-to-text'

import TinymceEditor from './editor/TinymceEditor.vue'
import IllustrateCopilot from './editor/IllustrateCopilot.vue'

import { copilotIcon } from 'src/assets'

const title = ref('')
const content = ref('')
const abbreviation = ref('')
const coverDescription = computed({
  get: () => htmlToText(content.value, { wordwrap: false }).split(' ').slice(0, 77).join(' '),
  set: () => {
    // Do nothing
  }
})

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
    mutation submitContent ($cid: String!, $title: String!, $content: String!, $cover: String!, $abbreviation: String!) {
      submitContent(cid: $cid, title: $title, content: $content, cover: $cover, abbreviation: $abbreviation)
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
    cover: coverBase64.value,
    abbreviation: abbreviation.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
}

const submitContentThroughCheCko = async () => {
  const bytes = json.encode({ content })
  const hash = await sha256.digest(bytes)
  const cid = CID.create(1, json.code, hash).toString()

  const query = gql`
    mutation submitContent ($cid: String!, $title: String!, $content: String!, $cover: String!, $abbreviation: String!) {
      submitContent(cid: $cid, title: $title, content: $content, cover: $cover, abbreviation: $abbreviation)
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
          cover: coverBase64.value,
          abbreviation: abbreviation.value,
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

const onAdvancedUpload = (ev: unknown) => {
  console.log(ev)
}

const showCoverCopilot = ref(false)
const coverBase64 = ref(undefined as unknown as string)

const onFileSelected = async (ev: FileUploadSelectEvent) => {
  const data = await fetch(((ev.files as Array<unknown>)[0] as Record<string, string>).objectURL)
  const reader = new FileReader()
  reader.onload = () => {
    coverBase64.value = reader.result as string
  }
  reader.onerror = (e) => {
    console.log(e)
  }
  reader.readAsDataURL(await data.blob())
}

const onCoverGenerated = (base64: string) => {
  coverBase64.value = base64
  showCoverCopilot.value = false
}

const onCoverCopilotCancel = () => {
  showCoverCopilot.value = false
}

const onCoverCopilotClick = () => {
  showCoverCopilot.value = true
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
