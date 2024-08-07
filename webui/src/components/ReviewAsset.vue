<template>
  <div class='row'>
    <q-space />
    <div :style='{maxWidth:"800px"}'>
      <div class='row' :style='{margin:"16px 0"}'>
        <div class='row cursor-pointer' :style='{lineHeight:"32px"}' @click='onBackClick'>
          <q-icon name='arrow_back' size='32px' />
          <span :style='{marginLeft:"8px"}'>{{ $t('MSG_REVIEW_ASSET') }}</span>
        </div>
        <q-space />
        <div class='row' :style='{lineHeight:"32px"}'>
          <span><strong>{{ asset?.name }}</strong></span>
        </div>
      </div>
      <q-separator />
      <div :style='{marginTop:"24px"}'>
        <div :style='{fontWeight: "bold", fontSize: "28px", wordBreak: "break-word", marginBottom: "16px"}'>
          {{ asset?.name || 'You should have a title!' }}
        </div>
        <div>
          By
          <span class='text-grey-6 text-bold cursor-pointer'>
            {{ asset?.author || 'Anonymous' }}
          </span>
        </div>
        <div>
          At
          <span class='text-grey-6 text-bold'>
            {{ asset?.createdAt ? date.formatDate(asset?.createdAt / 1000) : '' }}
          </span>
        </div>
        <div>
          Cid
          <span class='text-grey-6 text-bold cursor-pointer'>
            {{ asset?.cid }}
          </span>
        </div>
        <div v-if='asset?.uris.length' :style='{margin:"24px 0"}'>
          <q-img
            v-for='uri in asset.uris' :key='uri'
            :src='asset.baseUri + "/" + uri'
            width='320px'
            :style='{borderRadius:"8px", margin:"8px"}'
          >
            <div class='absolute-bottom text-subtitle1 text-center'>
              {{ asset.baseUri + "/" + uri }}
            </div>
            <template #error>
              <div class='absolute-full flex flex-center bg-negative text-white'>
                Cannot load image {{ asset.baseUri + "/" + uri }}
              </div>
            </template>
          </q-img>
        </div>
        <div
          v-else
          :style='{margin: "24px 0 24px 0", fontSize: "16px", wordBreak: "break-word"}'
          v-html='"You should have some content!"'
        />
      </div>
      <q-separator />
      <div :style='{marginTop: "24px"}'>
        <q-input v-model='reason' type='textarea' :label='$t("MSG_REVIEW_REASON")' :disable='reviewed' />
      </div>
      <div :style='{marginTop: "24px"}' class='row'>
        <q-btn :label='$t("MSG_APPROVE")' :style='{marginRight:"16px",color: _review?.approved ? "blue" : ""}' @click='onApproveClick' :disable='reviewed' />
        <q-btn :label='$t("MSG_REJECT")' :style='{color: _review && !_review?.approved ? "red" : ""}' @click='onRejectClick' :disable='reviewed' />
      </div>
    </div>
    <q-space />
  </div>
</template>

<script lang='ts' setup>
import { useReviewStore } from 'src/stores/review'
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Cookies, date } from 'quasar'
import { provideApolloClient, useMutation } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { ApolloClient } from '@apollo/client/core'
import { targetChain } from 'src/stores/chain'
import { useUserStore } from 'src/stores/user'
import { v4 as uuidv4 } from 'uuid'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

interface Query {
  cid: string
}

const route = useRoute()
const cid = computed(() => (route.query as unknown as Query).cid)
const review = useReviewStore()
const asset = computed(() => review.asset(cid.value))
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const router = useRouter()
const user = useUserStore()
const account = computed(() => user.account)
const reviewed = computed(() => review.assetReviewed(cid.value, account.value))
const _review = computed(() => review.assetReview(cid.value, account.value))
const reason = ref(_review.value?.reason || 'I supper like this art not only it\'s about Linera, but also it\'s created by KK.' + uuidv4())
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)

const approveAsset = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation approveAsset ($cid: String!, $reason: String!) {
      approveAsset(cid: $cid reason: $reason)
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    cid: asset.value?.cid,
    reason: reason.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
}

const approveAssetThroughCheCko = () => {
  const query = gql`
    mutation approveAsset ($cid: String!, $reason: String!) {
      approveAsset(cid: $cid reason: $reason)
    }`
  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      applicationId: reviewApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          cid: asset.value?.cid,
          reason: reason.value
        },
        operationName: 'approveAsset'
      }
    }
  }).then((result) => {
    console.log(result)
  }).catch((e) => {
    console.log(e)
  })
}

const onApproveClick = () => {
  if (!asset.value || !reason.value.length) {
    return
  }
  if (cheCkoConnect.value) {
    approveAssetThroughCheCko()
  } else {
    void approveAsset()
  }
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-assets',
      host: Cookies.get('service-host'),
      port: Cookies.get('service-port'),
      cheCkoConnect: Cookies.get('cheCkoConnect')
    }
  })
}

const rejectAsset = async () => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation rejectAsset ($cid: String!, $reason: String!) {
      rejectAsset(cid: $cid, reason: $reason)
    }
  `))
  onDone(() => {
    // TODO
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    cid: asset.value?.cid,
    reason: reason.value,
    endpoint: 'review',
    chainId: targetChain.value
  })
}

const rejectAssetThroughCheCko = () => {
  const query = gql`
    mutation rejectAsset ($cid: String!, $reason: String!) {
      rejectAsset(cid: $cid, reason: $reason)
    }`
  window.linera.request({
    method: 'linera_graphqlMutation',
    params: {
      applicationId: reviewApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          cid: asset.value?.cid,
          reason: reason.value
        },
        operationName: 'rejectAsset'
      }
    }
  }).then((result) => {
    console.log(result)
  }).catch((e) => {
    console.log(e)
  })
}

const onRejectClick = () => {
  if (!asset.value || !reason.value.length) {
    return
  }
  if (cheCkoConnect.value) {
    rejectAssetThroughCheCko()
  } else {
    void rejectAsset()
  }
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-assets',
      host: Cookies.get('service-host'),
      port: Cookies.get('service-port'),
      cheCkoConnect: Cookies.get('cheCkoConnect')
    }
  })
}

const onBackClick = () => {
  void router.push({
    path: '/dashboard',
    query: {
      tab: 'review-assets',
      host: Cookies.get('service-host'),
      port: Cookies.get('service-port'),
      cheCkoConnect: Cookies.get('cheCkoConnect')
    }
  })
}

</script>
