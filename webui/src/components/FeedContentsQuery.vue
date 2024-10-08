<script setup lang="ts">
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useContentStore, Content } from 'src/stores/content'
import { computed, watch, ref, onMounted } from 'vue'
import { useBlockStore } from 'src/stores/block'
import { targetChain } from 'src/stores/chain'
import { graphqlResult } from 'src/utils'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

const content = useContentStore()
const contentsKeys = computed(() => content.contentsKeys)
const contents = computed(() => content.contents)
const recommends = computed(() => content.recommends)
const comments = computed(() => content.comments)
const contentIndex = ref(-1)
const contentKey = computed(() => contentIndex.value >= 0 ? contentsKeys.value[contentIndex.value] : undefined)
const mutateKeys = computed(() => content.mutateKeys)
const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const feedApp = computed(() => application.feedApp)

const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)

const getContent = (contentKey: string, done?: () => void) => {
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getContent($contentKey: String!) {
      contents {
        entry(key: $contentKey) {
          value {
            accounts
            cid
            commentToCid
            title
            content
            cover
            abbreviation
            author
            likes
            dislikes
            createdAt
          }
        }
      }
      contentRecommends {
        entry(key: $contentKey) {
          value
        }
      }
      contentComments {
        entry(key: $contentKey) {
          value
        }
      }
    }
  `, {
    contentKey: `${contentKey}`,
    endpoint: 'feed',
    chainId: targetChain.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const _contents = graphqlResult.data(res, 'contents')
    contents.value.set(contentKey, graphqlResult.entryValue(_contents) as Content)
    const _recommends = graphqlResult.data(res, 'contentRecommends')
    recommends.value.set(contentKey, graphqlResult.entryValue(_recommends) as Array<string>)
    const _comments = graphqlResult.data(res, 'contentComments')
    comments.value.set(contentKey, graphqlResult.entryValue(_comments) as Array<string>)
    done?.()
  })
}

const getContentThroughCheCko = (contentKey: string, done?: () => void) => {
  const query = gql`
    query getContent($contentKey: String!) {
      contents {
        entry(key: $contentKey) {
          value {
            accounts
            cid
            commentToCid
            title
            content
            cover
            abbreviation
            author
            likes
            dislikes
            createdAt
          }
        }
      }
      contentRecommends {
        entry(key: $contentKey) {
          value
        }
      }
      contentComments {
        entry(key: $contentKey) {
          value
        }
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: feedApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          contentKey: `${contentKey}`
        },
        operationName: 'getContent'
      }
    }
  }).then((result) => {
    const _contents = graphqlResult.keyValue(result, 'contents')
    contents.value.set(contentKey, graphqlResult.entryValue(_contents) as Content)
    const _recommends = graphqlResult.keyValue(result, 'contentRecommends')
    recommends.value.set(contentKey, graphqlResult.entryValue(_recommends) as Array<string>)
    const _comments = graphqlResult.keyValue(result, 'contentComments')
    comments.value.set(contentKey, graphqlResult.entryValue(_comments) as Array<string>)
    done?.()
  }).catch((e) => {
    console.log(e)
  })
}

watch(contentKey, () => {
  if (!contentKey.value) {
    return
  }
  const index = mutateKeys.value.findIndex((el) => el === contentKey.value)
  if (contents.value.get(contentKey.value) && index < 0) {
    contentIndex.value++
    return
  }

  if (cheCkoConnect.value) {
    getContentThroughCheCko(contentKey.value, () => {
      contentIndex.value++
      mutateKeys.value.splice(index, 1)
    })
  } else {
    getContent(contentKey.value, () => {
      contentIndex.value++
      mutateKeys.value.splice(index, 1)
    })
  }
})

watch(contentsKeys, () => {
  if (contentsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

watch(blockHeight, () => {
  if (contentsKeys.value.length === 0) {
    return
  }
  contentIndex.value = 0
})

onMounted(() => {
  content.mutateKeys.forEach((contentKey) => {
    if (cheCkoConnect.value) {
      getContentThroughCheCko(contentKey)
    } else {
      getContent(contentKey)
    }
  })
  content.mutateKeys = []
})

</script>
