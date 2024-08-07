<template>
  <q-table
    :rows='contents'
    :columns='(columns as never)'
  />
</template>

<script setup lang='ts'>
import { useFoundationStore } from 'src/stores/foundation'
import { useReviewStore, Content } from 'src/stores/review'
import { useUserStore } from 'src/stores/user'
import { publickeytoowner } from 'src/utils'
import { computed, ref, watch } from 'vue'

const review = useReviewStore()
const user = useUserStore()
const account = computed(() => user.account)
const owner = ref('')
const contents = computed(() => [...review.contents(account.value), ...review.contents(owner.value)])
const foundation = useFoundationStore()
const estimatedReward = computed(() => Number(foundation.authorRewardBalance) / foundation.authorRewardFactor)

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
    name: 'Approved',
    label: 'Approved',
    field: (row: Content) => row.approved
  }, {
    name: 'Rejected',
    label: 'Rejected',
    field: (row: Content) => row.rejected
  }, {
    name: 'Estimated Reward',
    label: 'Estimated Reward',
    field: (row: Content) => {
      if (row.approved >= review.reviewerNumber) {
        return '-'
      }
      if (row.approved >= review.contentApprovedThreshold) {
        return '-'
      }
      if (row.rejected >= review.contentRejectedThreshold) {
        return '-'
      }
      if (row.rejected >= review.reviewerNumber) {
        return '-'
      }
      return estimatedReward.value.toString() + ' Lineras'
    }
  }, {
    name: 'State',
    label: 'State',
    field: (row: Content) => {
      if (row.approved >= review.reviewerNumber) {
        return 'Approved'
      }
      if (row.approved >= review.contentApprovedThreshold) {
        return 'Approved'
      }
      if (row.rejected >= review.contentRejectedThreshold) {
        return 'Rejected'
      }
      if (row.rejected >= review.reviewerNumber) {
        return 'Rejected'
      }
      let approvedNeeded = review.reviewerNumber - row.approved
      if (review.contentApprovedThreshold < review.reviewerNumber) {
        approvedNeeded = review.contentApprovedThreshold - row.approved
      }
      return approvedNeeded.toString() + ' Approvals Needed'
    }
  }
])

watch(account, async () => {
  if (!account.value) {
    return
  }
  owner.value = await publickeytoowner.publicKeyToOwner(account.value)
})

</script>
