<template>
  <q-card :style='{padding: "24px"}'>
    <div>Node ID: {{ cpNode.nodeId }}</div>
    <div>Brand: <q-img :src='cpNode.brandLogo' width='36px' height='36px' /> {{ cpNode.brandName }}</div>
    <div>Computing Unit: {{ cpNode.resourceType }}</div>
    <div>Device Model: {{ cpNode.deviceModel }}</div>
    <div>CPU: {{ cpNode.cpuModel }}</div>
    <div>Storage Type: {{ cpNode.storageType }}</div>
    <div>Storage Bytes: {{ cpNode.storageBytes }}</div>
    <div>Memory Bytes: {{ cpNode.memoryBytes }}</div>
    <div>Free Quota: {{ cpNode.freeQuota }}</div>
    <div>Price Per Task: {{ cpNode.quotaPrice }}</div>
    <div>Supported Tasks: {{ cpNode.supportedTaskTypes.join(', ') }}</div>
    <div>Created At: {{ date.formatDate(cpNode.createdAt / 1000) }}</div>
  </q-card>
</template>

<script lang='ts' setup>
import { defineProps, toRef } from 'vue'
import { date } from 'quasar'
import { CPNode } from 'src/stores/cpregistry'

interface Props {
  cpNode: CPNode
}

const props = defineProps<Props>()
const cpNode = toRef(props, 'cpNode')

</script>

<style scoped lang='sass'>
.error
  background-image: url(../assets/ResPeer.png)
  border-radius: 50%
  background-size: cover
  background-repeat: no-repeat

.error-normal
  border-radius: 0

table
  font-family: arial, sans-serif
  border-collapse: collapse
  width: 100%

td, th
  border: 1px solid #dddddd
  text-align: left
  padding: 8px

tr:nth-child(even)
  background-color: #eeeeee
</style>
