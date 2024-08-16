<template>
  <div :style='{width: "960px", padding: "4px 4px 4px 4px"}'>
    <div class='row'>
      <q-avatar size='36px'>
        <q-img
          :src='userAvatar(_content.author) ? userAvatar(_content.author) : "~/assets/ResPeer.png"'
          width='36px'
          height='36px'
          fit='cover'
          :style='{borderRadius: "50%"}'
        >
          <template #error>
            <div class='absolute-full flex flex-center error' />
          </template>
        </q-img>
      </q-avatar>
      <div :style='{marginLeft: "16px"}'>
        <div class='text-grey-8 text-bold cursor-pointer'>
          {{ _content.author?.length ? _content.author : 'Anonymous' }}
        </div>
        <div class='text-grey-6 text-bold'>
          {{ date.formatDate(_content.createdAt / 1000, 'YYYY-MM-DD HH:mm:ss') }}
        </div>
      </div>
    </div>
    <div :style='{marginLeft: "52px", marginTop: "24px"}'>
      <div class='row'>
        <div>
          <div class='cursor-pointer' :style='{fontWeight: "bold", fontSize: "26px", wordBreak: "break-word"}' @click='onTitleClick(_content.cid)'>
            {{ _content.title?.length ? _content.title : 'You should have a title!' }}
          </div>
          <div class='cursor-pointer text-grey-8' :style='{fontSize: "16px", wordBreak: "break-word", marginTop: "8px"}' @click='onTitleClick(_content.cid)'>
            {{ _content.abbreviation?.length ? _content.abbreviation : 'You should have an abbreviation!' }}
          </div>
        </div>
        <q-space />
        <div :style='{background: "#1d1d1d0f", borderRadius: "16px", width: "180px", height: "120px"}' class='text-center'>
          <q-img :src='_content?.cover' height='120px' fit='contain' />
        </div>
      </div>
      <div :style='{marginTop: "24px"}' />
      <q-chat-message
        v-if='recommends.length'
        :text='[recommends[0].content]'
        bg-color='grey-2'
        :stamp='date.formatDate(recommends[0].createdAt / 1000, "YYYY-MM-DD HH:mm:ss")'
      />
      <q-chat-message
        v-else-if='comments.length'
        :text='[comments[0].content]'
        bg-color='grey-2'
        :stamp='date.formatDate(comments[0].createdAt / 1000, "YYYY-MM-DD HH:mm:ss")'
      />
      <div class='row' :style='{marginTop: "16px"}'>
        <div class='row cursor-pointer' @click='onLikeClick(_content.cid)'>
          <q-icon name='thumb_up' size='20px' :style='{marginRight: "6px"}' />
          {{ _content.likes }}
        </div>
        <div class='row cursor-pointer' :style='{marginLeft: "16px"}' @click='onDislikeClick(_content.cid)'>
          <q-icon name='thumb_down' size='20px' :style='{marginRight: "6px"}' />
          {{ _content.dislikes }}
        </div>
        <div class='row cursor-pointer' :style='{marginLeft: "16px"}' @click='onCommentClick(_content.cid)'>
          <q-icon name='comment' size='20px' :style='{marginRight: "6px"}' />
          {{ comments.length }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang='ts'>
import { useContentStore, Content } from 'src/stores/content'
import { computed, onMounted, ref, toRef, watch } from 'vue'
import { provideApolloClient, useMutation, useQuery } from '@vue/apollo-composable'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { getClientOptions } from 'src/apollo'
import { useCollectionStore } from 'src/stores/collection'
import { targetChain } from 'src/stores/chain'
import { Cookies, date } from 'quasar'
import { useRouter } from 'vue-router'
import { useSettingStore } from 'src/stores/setting'
import { useApplicationStore } from 'src/stores/application'

interface Props {
  cid: string
  expand?: boolean
  list?: boolean
}
const props = withDefaults(defineProps<Props>(), {
  list: true
})
const cid = toRef(props, 'cid')
const expand = toRef(props, 'expand')
const list = toRef(props, 'list')

const content = useContentStore()
const _content = computed(() => content.content(cid.value) as Content)
const contentText = ref(_content.value?.content)
const comments = computed(() => content._comments(cid.value))
const recommends = computed(() => content._recommends(cid.value).slice(0, expand.value ? undefined : 1))
const collection = useCollectionStore()
const options = /* await */ getClientOptions(/* {app, router ...} */)
const apolloClient = new ApolloClient(options)
const router = useRouter()
const setting = useSettingStore()
const cheCkoConnect = computed(() => setting.cheCkoConnect)
const application = useApplicationStore()
const marketApp = computed(() => application.marketApp)

const userAvatar = (account: string) => {
  const ids = collection.avatars.get(account)
  if (!ids) {
    return collection.nftBannerByID(1001, 1000)
  }
  return collection.nftBannerByID(ids[0], ids[1])
}

const ready = () => {
  return (cheCkoConnect.value || targetChain.value?.length) && _content.value
}

const _getContentAvatar = () => {
  const account = _content.value?.author
  const { result /*, fetchMore, onResult, onError */ } = provideApolloClient(apolloClient)(() => useQuery(gql`
    query getMarketInfo($account: String!) {
      avatars {
        entry(key: $account) {
          value
        }
      }
    }`, {
    account: `${account}`,
    endpoint: 'market',
    chainId: targetChain.value
  }))

  watch(result, () => {
    const res = result.value as Record<string, Array<number>>
    collection.avatars.set(account, res.avatars)
  })
}

const getContentAvatarThroughCheCko = () => {
  const account = _content.value?.author
  const query = gql`
    query getMarketInfo($account: String!) {
      avatars {
        entry(key: $account) {
          value
        }
      }
    }`

  window.linera.request({
    method: 'linera_graphqlQuery',
    params: {
      applicationId: marketApp.value,
      query: {
        query: query.loc?.source?.body,
        variables: {
          account: `${account}`
        },
        operationName: 'getMarketInfo'
      }
    }
  }).then((result) => {
    const res = result as Record<string, Array<number>>
    collection.avatars.set(account, res.avatars)
  }).catch((e) => {
    console.log(e)
  })
}

const getContentAvatar = () => {
  const account = _content.value?.author
  if (collection.avatars.get(account)) {
    return
  }
  if (cheCkoConnect.value) {
    getContentAvatarThroughCheCko()
  } else {
    _getContentAvatar()
  }
}

watch(targetChain, () => {
  if (!ready()) return
  getContentAvatar()
})

onMounted(() => {
  if (!ready()) return
  getContentAvatar()
  contentText.value = contentText.value.replace('<img ', '<img width="678px" fit="contain" ')
  console.log(_content.value?.cover, _content.value?.abbreviation, _content.value.cid)
})

const onLikeClick = async (cid: string) => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation like ($cid: String!) {
      like(ccid: $cid)
    }
  `))
  onDone(() => {
    content.mutateKeys.push(cid)
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    cid,
    endpoint: 'feed',
    chainId: targetChain.value
  })
}

const onDislikeClick = async (cid: string) => {
  const { mutate, onDone, onError } = provideApolloClient(apolloClient)(() => useMutation(gql`
    mutation dislike ($cid: String!) {
      dislike(ccid: $cid)
    }
  `))
  onDone(() => {
    content.mutateKeys.push(cid)
  })
  onError((error) => {
    console.log(error)
  })
  await mutate({
    cid,
    endpoint: 'feed',
    chainId: targetChain.value
  })
}

const emit = defineEmits<{(ev: 'comment'): void}>()

const onCommentClick = (cid: string) => {
  if (list.value) {
    void router.push({
      path: '/content',
      query: {
        cid,
        host: Cookies.get('service-host'),
        port: Cookies.get('service-port'),
        cheCkoConnect: Cookies.get('cheCkoConnect')
      }
    })
    return
  }
  emit('comment')
}

const onTitleClick = (cid: string) => {
  void router.push({
    path: '/content',
    query: {
      cid,
      host: Cookies.get('service-host'),
      port: Cookies.get('service-port'),
      cheCkoConnect: Cookies.get('cheCkoConnect')
    }
  })
}

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/ResPeer.png)
  border-radius: 50%
  background-size: cover
  background-repeat: no-repeat
</style>
