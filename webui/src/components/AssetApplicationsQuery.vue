<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useReviewStore, Asset } from 'src/stores/review'
import { computed, watch, ref } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const review = useReviewStore()
const assetApplicationsKeys = computed(() => review.assetApplicationsKeys)
const assetApplications = computed(() => review.assetApplications)
const contentIndex = ref(-1)
const assetApplicationKey = computed(() => contentIndex.value >= 0 ? assetApplicationsKeys.value[contentIndex.value] : undefined)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getAssetApplication = (assetApplicationKey: string, done?: () => void) => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getAssetApplication($assetApplicationKey: String!) {
      assetApplications(string: $assetApplicationKey) {
        cid
        baseUri
        uris
        author
        price
        name
        reviewers
        approved
        rejected
        createdAt
      }
    }
  `, {
    assetApplicationKey: `${assetApplicationKey}`,
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const _assetApplications = graphqlResult.data(res, 'assetApplications')
    assetApplications.value.set(assetApplicationKey, graphqlResult.entryValue(_assetApplications) as Asset)
    done?.()
  })
}

const getAssetApplicationThroughCheCko = (assetApplicationKey: string, done?: () => void) => {
  const query = gql`
    query getAssetApplication($assetApplicationKey: String!) {
      assetApplications(string: $assetApplicationKey) {
        cid
        baseUri
        uris
        author
        price
        name
        reviewers
        approved
        rejected
        createdAt
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: reviewApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          assetApplicationKey
        },
        operationName: 'getActivityApplication'
      }
    }
  }).then((result) => {
    const _assetApplications = graphqlResult.keyValue(result, 'assetApplications')
    assetApplications.value.set(assetApplicationKey, graphqlResult.entryValue(_assetApplications) as Asset)
    done?.()
  }).catch((e) => {
    console.log(e)
  })
}

watch(assetApplicationKey, () => {
  if (!assetApplicationKey.value) {
    return
  }
  if (cheCkoConnect.value) {
    getAssetApplicationThroughCheCko(assetApplicationKey.value, () => {
      contentIndex.value++
    })
  } else {
    getAssetApplication(assetApplicationKey.value, () => {
      contentIndex.value++
    })
  }
})

watch(assetApplicationsKeys, () => {
  if (assetApplicationsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

watch(blockHeight, () => {
  if (assetApplicationsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

</script>
