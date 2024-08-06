<template>
  <q-table
    :rows='contents'
    :columns='(columns as never)'
  />
</template>

<script setup lang='ts'>
import { Content, useContentStore } from 'src/stores/content'
import { useUserStore } from 'src/stores/user'
import { computed, toRef } from 'vue'

interface Props {
  articleType: string
}

const props = defineProps<Props>()
const articleType = toRef(props, 'articleType')

const content = useContentStore()
const user = useUserStore()
const account = computed(() => user.account)
const contents = computed(() => {
  switch (articleType.value) {
    case 'MY_ARTICLE':
      return Array.from(content._contents(account.value)).sort((a, b) => a.createdAt > b.createdAt ? 1 : -1).filter((el) => el.author === account.value)
    case 'MY_LIKE':
      return Array.from(content._contents(account.value)).sort((a, b) => a.createdAt > b.createdAt ? 1 : -1).filter((el) => {
        if (el.accounts === undefined) {
          return false
        }
        const index = Object.keys(el.accounts).findIndex((key) => key === account.value)
        if (index < 0) {
          return false
        }
        return Object.values(el.accounts)[index]
      })
    case 'MY_DISLIKE':
      return Array.from(content._contents(account.value)).sort((a, b) => a.createdAt > b.createdAt ? 1 : -1).filter((el) => {
        if (el.accounts === undefined) {
          return false
        }
        const index = Object.keys(el.accounts).findIndex((key) => key === account.value)
        if (index < 0) {
          return false
        }
        return !Object.values(el.accounts)[index]
      })
  }
  return Array.from(content._contents(account.value)).sort((a, b) => a.createdAt > b.createdAt ? 1 : -1).filter((el) => el.author === account.value)
})

const columns = computed(() => [
  {
    name: 'CID',
    label: 'CID',
    field: (row: Content) => row.cid
  }, {
    name: 'Title',
    label: 'Title',
    field: (row: Content) => row.title
  }, {
    name: 'Like',
    label: 'Like',
    field: (row: Content) => row.likes
  }, {
    name: 'Dislike',
    label: 'Dislike',
    field: (row: Content) => row.dislikes
  }
])

</script>
