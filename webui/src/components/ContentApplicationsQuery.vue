<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useReviewStore, Content } from 'src/stores/review'
import { computed, watch, ref } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useApplicationStore } from 'src/stores/application'
import { useSettingStore } from 'src/stores/setting'

const review = useReviewStore()
const contentApplicationsKeys = computed(() => review.contentApplicationsKeys)
const contentApplications = computed(() => review.contentApplications)
const contentIndex = ref(-1)
const contentApplicationKey = computed(() => contentIndex.value >= 0 ? contentApplicationsKeys.value[contentIndex.value] : undefined)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const application = useApplicationStore()
const reviewApp = computed(() => application.reviewApp)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length > 0) && reviewApp.value?.length > 0
}

const getContentApplication = (contentApplicationKey: string, done?: () => void) => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContentApplication($contentApplicationKey: String!) {
      contentApplications {
        entry(key: $contentApplicationKey) {
          value {
            cid
            commentToCid
            author
            title
            content
            reviewers
            approved
            rejected
            createdAt
          }
        }
      }
    }
  `, {
    contentApplicationKey: `${contentApplicationKey}`,
    endpoint: 'review',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const _contentApplications = graphqlResult.data(res, 'contentApplications')
    contentApplications.value.set(contentApplicationKey, graphqlResult.entryValue(_contentApplications) as Content)
    done?.()
  })
}

const getContentApplicationThroughCheCko = (contentApplicationKey: string, done?: () => void) => {
  const query = gql`
    query getContentApplication($contentApplicationKey: String!) {
      contentApplications {
        entry(key: $contentApplicationKey) {
          value {
            cid
            commentToCid
            author
            title
            content
            reviewers
            approved
            rejected
            createdAt
          }
        }
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: reviewApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          contentApplicationKey: `${contentApplicationKey}`
        },
        operationName: 'getContentApplication'
      }
    }
  }).then((result) => {
    const _contentApplications = graphqlResult.keyValue(result, 'contentApplications')
    contentApplications.value.set(contentApplicationKey, graphqlResult.entryValue(_contentApplications) as Content)
    done?.()
  }).catch((e) => {
    console.log(e)
  })
}

watch(contentApplicationKey, () => {
  if (!ready()) return
  if (!contentApplicationKey.value) {
    return
  }
  if (cheCkoConnect.value) {
    getContentApplicationThroughCheCko(contentApplicationKey.value, () => {
      contentIndex.value++
    })
  } else {
    getContentApplication(contentApplicationKey.value, () => {
      contentIndex.value++
    })
  }
})

watch(contentApplicationsKeys, () => {
  if (!ready()) return
  if (contentApplicationsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

watch(blockHeight, () => {
  if (!ready()) return
  if (contentApplicationsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

</script>
