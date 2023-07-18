<script setup lang="ts">
import { useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { useBlockStore } from 'src/stores/block'
import { useContentStore } from 'src/stores/content'
import { computed, watch } from 'vue'

const { refetch, onResult } = useQuery(gql`
  query getContentsKeys {
    contentsKeys
  }
`, {
  endpoint: 'feed'
})

const block = useBlockStore()
const blockHeight = computed(() => block.blockHeight)
watch(blockHeight, () => {
  void refetch()
})

const content = useContentStore()
onResult((res) => {
  if (res.loading) {
    return
  }
  content.contentsKeys = (res.data as Record<string, Array<string>>).contentsKeys
})

</script>